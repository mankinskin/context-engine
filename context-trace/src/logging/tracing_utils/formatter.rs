//! Event formatter for compact log output

use super::{
    config::FormatConfig,
    field_visitor::FieldVisitor,
    string_utils::strip_ansi_codes,
    syntax::highlight_rust_signature,
    timer::CompactTimer,
};
use std::fmt::Write as _;
use tracing::{
    Level,
    Subscriber,
    field::Field,
};
use tracing_subscriber::{
    fmt::{
        FmtContext,
        FormatEvent,
        FormatFields,
        format,
        time::FormatTime,
    },
    registry::LookupSpan,
};

/// Custom event formatter that puts each field on its own line,
/// with multi-line values indented on subsequent lines.
pub(super) struct CompactFieldsFormatter {
    pub(super) timer: CompactTimer,
    pub(super) config: FormatConfig,
}

impl CompactFieldsFormatter {
    pub(super) fn new(config: FormatConfig) -> Self {
        Self {
            timer: CompactTimer::new(),
            config,
        }
    }
}

impl<S, N> FormatEvent<S, N> for CompactFieldsFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        // Get current span context for indentation
        let span_count =
            ctx.event_scope().map(|scope| scope.count()).unwrap_or(0);

        // Check if this is a span lifecycle event
        let mut message_text = String::new();
        event.record(&mut |field: &Field, value: &dyn std::fmt::Debug| {
            if field.name() == "message" {
                message_text = format!("{:?}", value);
            }
        });

        let is_span_event = message_text == "\"new\""
            || message_text == "\"enter\""
            || message_text == "\"exit\""
            || message_text == "\"close\"";

        // Skip redundant span events - only show enter and close
        // Also skip based on configuration
        let should_skip = message_text == "\"new\"" 
            || message_text == "\"exit\""
            || (message_text == "\"enter\"" && !self.config.span_enter.show)
            || (message_text == "\"close\"" && !self.config.span_close.show);
        if should_skip {
            return Ok(());
        }

        // Determine decoration based on event type
        let decoration = if is_span_event && self.config.enable_indentation {
            match message_text.as_str() {
                "\"enter\"" => "\u{252c}\u{2500}", // ┬─ Opening span
                "\"close\"" => "\u{2514}\u{2500}", // └─ Closing span
                _ => "  ",
            }
        } else if is_span_event {
            "  " // No decoration if indentation disabled
        } else if self.config.enable_indentation {
            "\u{25cf} " // ● Regular event (filled circle)
        } else {
            "  "
        };

        // Create indentation with visual gutters, replacing last gutter with decoration
        let (gutter_indent, indent) = if self.config.enable_indentation {
            let gutter = if span_count > 0 {
                "│ ".repeat(span_count)
            } else {
                String::new()
            };

            let indent_str = if span_count > 0 {
                let parent_gutters = "│ ".repeat(span_count - 1);
                format!("{}{}", parent_gutters, decoration)
            } else {
                decoration.to_string()
            };
            (gutter, indent_str)
        } else {
            // No indentation - just use spaces
            let spaces = "  ".repeat(span_count);
            (spaces.clone(), spaces)
        };

        // Write indentation and timestamp
        write!(writer, "{}", indent)?;
        self.timer.format_time(&mut writer)?;

        // Write level
        let level = *event.metadata().level();
        if self.config.enable_ansi {
            let level_str = match level {
                Level::ERROR => "\x1b[31mERROR\x1b[0m",
                Level::WARN => "\x1b[33m WARN\x1b[0m",
                Level::INFO => "\x1b[32m INFO\x1b[0m",
                Level::DEBUG => "\x1b[34mDEBUG\x1b[0m",
                Level::TRACE => "\x1b[35mTRACE\x1b[0m",
            };
            write!(writer, " {}", level_str)?;
        } else {
            write!(writer, " {:5}", level)?;
        }

        // Write the message - separate handling for message field and other fields
        write!(writer, "  ")?;

        // For span events, write custom messages instead of quoted strings
        if is_span_event {
            let span =
                ctx.event_scope().and_then(|scope| scope.from_root().last());

            let span_name = span.as_ref().map(|s| s.name()).unwrap_or("?");
            let target = span.as_ref().map(|s| s.metadata().target()).unwrap_or("");

            let event_name = match message_text.as_str() {
                "\"new\"" => format!("SPAN CREATED: {}", span_name),
                "\"enter\"" => {
                    // Try to extract fn_sig from the span's formatted fields if enabled
                    let fn_sig = if self.config.span_enter.show_fn_signature {
                        span.as_ref().and_then(|s| {
                            // Try to get from FormattedFields extension first
                            if let Some(fields) = s.extensions().get::<tracing_subscriber::fmt::FormattedFields<N>>() {
                                // Strip ANSI codes and newlines from formatted fields
                                let fields_str = strip_ansi_codes(fields.as_str()).replace('\n', "");
                                
                                // Parse fn_sig from formatted fields (format: fn_sig="...")
                                if let Some(idx) = fields_str.find("fn_sig=\"") {
                                    let start = idx + 8; // Skip 'fn_sig="'
                                    if let Some(end_offset) = fields_str[start..].find('"') {
                                        return Some(fields_str[start..start + end_offset].to_string());
                                    }
                                }
                                
                                // Also try without quotes (format: fn_sig=...)
                                if let Some(idx) = fields_str.find("fn_sig=") {
                                    let start = idx + 7; // Skip 'fn_sig='
                                    let remaining = &fields_str[start..];
                                    let end = remaining.find(char::is_whitespace).unwrap_or(remaining.len());
                                    if end > 0 {
                                        let value = &remaining[..end];
                                        return Some(value.trim_matches('"').to_string());
                                    }
                                }
                            }
                            None
                        })
                    } else {
                        None
                    };

                    // Show module path, function name, and highlighted signature if available
                    if let Some(sig) = fn_sig {
                        let highlighted = highlight_rust_signature(&sig, self.config.enable_ansi);
                        format!("SPAN ENTERED: {}::{} - {}", target, span_name, highlighted)
                    } else {
                        format!("SPAN ENTERED: {}::{}", target, span_name)
                    }
                },
                "\"exit\"" => format!("SPAN EXITED: {}", span_name),
                "\"close\"" => {
                    // Show timing info if enabled
                    if self.config.span_close.show_timing {
                        // The timing fields will be shown by the field visitor below
                        format!("SPAN CLOSED: {}", span_name)
                    } else {
                        format!("SPAN CLOSED: {}", span_name)
                    }
                },
                _ => message_text.clone(),
            };
            write!(writer, "{}", event_name)?;
        } else {
            // Regular event - write "EVENT:" prefix for clarity
            write!(writer, "EVENT: {}", message_text.trim_matches('"'))?;
        }

        // Then write all non-message fields on separate lines
        let mut visitor = FieldVisitor::new(
            &mut writer as &mut dyn std::fmt::Write,
            gutter_indent.clone(),
        );
        event.record(&mut visitor);
        visitor.result?;
        
        // For SPAN ENTERED events, also display the span's fields if enabled
        if self.config.span_enter.show_fields
            && message_text == "\"enter\""
            && let Some(span) = ctx.event_scope().and_then(|scope| scope.from_root().last())
                && let Some(fields) = span.extensions().get::<tracing_subscriber::fmt::FormattedFields<N>>() {
                    // Parse and display each field
                    let fields_str = strip_ansi_codes(fields.as_str()).replace('\n', " ");
                    
                    // Split by whitespace and process each field=value pair
                    for part in fields_str.split_whitespace() {
                        if let Some(eq_pos) = part.find('=') {
                            let field_name = &part[..eq_pos];
                            // Skip fn_sig (already displayed inline) and message
                            if field_name != "fn_sig" && field_name != "message" {
                                let field_value = &part[eq_pos + 1..];
                                write!(writer, "\n{}    {}={}", gutter_indent, field_name, field_value)?;
                            }
                        }
                    }
                }

        // For SPAN CLOSED events, filter timing fields if timing is disabled
        if message_text == "\"close\"" && !self.config.span_close.show_timing {
            // Note: timing fields (time.busy, time.idle) are still in the visitor output
            // To fully suppress them, we'd need to filter in the visitor
        }

        // Write file location if enabled
        if self.config.show_file_location {
            if let Some(file) = event.metadata().file()
                && let Some(line) = event.metadata().line() {
                    write!(writer, "\n{}    at {}:{}", gutter_indent, file, line)?;
                }
        }

        writeln!(writer)
    }
}
