//! Tracing setup utilities for tests
//!
//! Provides per-test logging to files with automatic cleanup on success.
//! Logs are written to `<target-dir>/test-logs/<test_name>.log` and deleted if the test passes.
//! The target directory is automatically detected from the Cargo build environment.
//!
//! # Log Format Features
//!
//! The logging system provides:
//! - **Compact format** with timestamps and file locations
//! - **Span tracking** showing NEW, ENTER, EXIT, and CLOSE events for spans
//! - **Visual indentation** in the compact format for nested spans
//! - **Color coding** in stdout (when terminal supports it)
//!
//! # Enabling stdout logging
//!
//! By default, logs are only written to files. To enable stdout logging for debugging,
//! set the `LOG_STDOUT` environment variable:
//!
//! ```bash
//! # Enable stdout logging
//! LOG_STDOUT=1 cargo test
//!
//! # Or with true/yes
//! LOG_STDOUT=true cargo test
//! LOG_STDOUT=yes cargo test
//!
//! # Run specific test with stdout logging
//! LOG_STDOUT=1 cargo test my_test_name -- --nocapture
//! ```
//!
//! You can also combine with `RUST_LOG` or `LOG_FILTER` to control log levels:
//!
//! ```bash
//! # Using LOG_FILTER (recommended)
//! LOG_STDOUT=1 LOG_FILTER=debug cargo test
//! LOG_STDOUT=1 LOG_FILTER=context_search::search=trace cargo test
//!
//! # Using RUST_LOG (fallback)
//! LOG_STDOUT=1 RUST_LOG=debug cargo test
//! ```

use std::{
    env,
    fmt::Write as _,
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
    sync::Once,
    time::SystemTime as StdSystemTime,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{
        Style,
        ThemeSet,
    },
    parsing::SyntaxSet,
    util::as_24_bit_terminal_escaped,
};
use tracing::{
    Level,
    Subscriber,
    field::{
        Field,
        Visit,
    },
};
use tracing_subscriber::{
    EnvFilter,
    Layer,
    fmt::{
        FmtContext,
        FormatEvent,
        FormatFields,
        format,
        format::FmtSpan,
        time::FormatTime,
    },
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};

/// Syntax highlight a Rust function signature
fn highlight_rust_signature(
    signature: &str,
    with_ansi: bool,
) -> String {
    if !with_ansi {
        return signature.to_string();
    }

    lazy_static::lazy_static! {
        static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    }

    let syntax = SYNTAX_SET
        .find_syntax_by_extension("rs")
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
    let theme = &THEME_SET.themes["base16-ocean.dark"];

    let mut highlighted = String::new();
    let mut highlighter = HighlightLines::new(syntax, theme);
    for line in syntect::util::LinesWithEndings::from(signature) {
        let ranges: Vec<(Style, &str)> = highlighter
            .highlight_line(line, &SYNTAX_SET)
            .unwrap_or_default();
        
        highlighted.push_str(&as_24_bit_terminal_escaped(&ranges, false));
    }

    highlighted
}

/// Field visitor to extract a specific field value
struct FieldExtractor {
    field_name: &'static str,
    value: Option<String>,
}

impl FieldExtractor {
    fn new(field_name: &'static str) -> Self {
        Self {
            field_name,
            value: None,
        }
    }
}

impl Visit for FieldExtractor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == self.field_name {
            self.value = Some(format!("{:?}", value));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == self.field_name {
            self.value = Some(value.to_string());
        }
    }
}


/// Strip ANSI escape codes from a string
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip escape sequence
            if let Some('[') = chars.next() {
                // Skip until we find a letter (the command character)
                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}


/// Compact timer that shows milliseconds since start
struct CompactTimer {
    start: StdSystemTime,
}

impl CompactTimer {
    fn new() -> Self {
        Self {
            start: StdSystemTime::now(),
        }
    }
}

impl FormatTime for CompactTimer {
    fn format_time(
        &self,
        w: &mut format::Writer<'_>,
    ) -> std::fmt::Result {
        let elapsed = StdSystemTime::now()
            .duration_since(self.start)
            .unwrap_or_default();

        let millis = elapsed.as_millis();

        // Format as seconds.milliseconds (e.g., "1.234s" or "0.056s")
        if millis < 1000 {
            write!(w, "{:3}ms", millis)
        } else if millis < 60_000 {
            write!(w, "{:5.2}s", millis as f64 / 1000.0)
        } else {
            let minutes = millis / 60_000;
            let remaining_ms = millis % 60_000;
            write!(w, "{}m{:05.2}s", minutes, remaining_ms as f64 / 1000.0)
        }
    }
}

/// Custom field visitor that formats each field on its own line
struct FieldVisitor<'a> {
    writer: &'a mut dyn std::fmt::Write,
    indent: String,
    result: std::fmt::Result,
}

impl<'a> FieldVisitor<'a> {
    fn new(
        writer: &'a mut dyn std::fmt::Write,
        indent: String,
    ) -> Self {
        Self {
            writer,
            indent,
            result: Ok(()),
        }
    }

    fn record_field(
        &mut self,
        field: &Field,
        value: &dyn std::fmt::Debug,
    ) {
        // Skip the message field - it's already been written
        if field.name() == "message" {
            return;
        }

        if self.result.is_err() {
            return;
        }

        // Format the value into a string first using alternate Debug (pretty print)
        let value_str = format!("{:#?}", value);

        // Calculate the base indentation for this field
        // This is where the field key starts
        let field_indent = format!("{}    ", self.indent);

        // Write field name on new line
        self.result =
            write!(self.writer, "\n{}{}=", field_indent, field.name());
        if self.result.is_err() {
            return;
        }

        // All lines of the value should be indented relative to where the field key is
        // The value's own Debug formatting provides nesting structure,
        // we just shift it all by the field's indentation
        let mut first_line = true;
        for line in value_str.lines() {
            if first_line {
                // First line goes right after the '='
                self.result = write!(self.writer, "{}", line);
                first_line = false;
            } else {
                // Subsequent lines maintain the Debug output's indentation
                // but shifted by the field's base indentation
                self.result = write!(self.writer, "\n{}{}", field_indent, line);
            }
            if self.result.is_err() {
                return;
            }
        }
    }
}

impl<'a> Visit for FieldVisitor<'a> {
    fn record_debug(
        &mut self,
        field: &Field,
        value: &dyn std::fmt::Debug,
    ) {
        // For Debug format (?), use standard pretty-print Debug
        self.record_field(field, value);
    }

    // Override record_str to handle Display values
    // This is called when using % format specifier in tracing macros
    fn record_str(
        &mut self,
        field: &Field,
        value: &str,
    ) {
        // Skip the message field
        if field.name() == "message" {
            return;
        }

        if self.result.is_err() {
            return;
        }

        let field_indent = format!("{}    ", self.indent);

        // Write field name on new line
        self.result =
            write!(self.writer, "\n{}{}=", field_indent, field.name());
        if self.result.is_err() {
            return;
        }

        // For Display string values (from %), indent all lines appropriately
        let mut first_line = true;
        for line in value.lines() {
            if first_line {
                self.result = write!(self.writer, "{}", line);
                first_line = false;
            } else {
                self.result = write!(self.writer, "\n{}{}", field_indent, line);
            }
            if self.result.is_err() {
                return;
            }
        }
    }
}

/// Custom event formatter that puts each field on its own line,
/// with multi-line values indented on subsequent lines.
struct CompactFieldsFormatter {
    timer: CompactTimer,
    with_ansi: bool,
}

impl CompactFieldsFormatter {
    fn new(with_ansi: bool) -> Self {
        Self {
            timer: CompactTimer::new(),
            with_ansi,
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
        let should_skip =
            message_text == "\"new\"" || message_text == "\"exit\"";
        if should_skip {
            return Ok(());
        }

        // Determine decoration based on event type
        let decoration = if is_span_event {
            match message_text.as_str() {
                "\"enter\"" => "\u{252c}\u{2500}", // ┬─ Opening span
                "\"close\"" => "\u{2514}\u{2500}", // └─ Closing span
                _ => "  ",
            }
        } else {
            "\u{25cf} " // ● Regular event (filled circle)
        };

        // Create indentation with visual gutters, replacing last gutter with decoration
        let gutter_indent = if span_count > 0 {
            "│ ".repeat(span_count)
        } else {
            String::new()
        };

        let indent = if span_count > 0 {
            let parent_gutters = "│ ".repeat(span_count - 1);
            format!("{}{}", parent_gutters, decoration)
        } else {
            decoration.to_string()
        };

        // Write indentation and timestamp
        write!(writer, "{}", indent)?;
        self.timer.format_time(&mut writer)?;

        // Write level
        let level = *event.metadata().level();
        if self.with_ansi {
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
                    // Try to extract fn_sig from the span's formatted fields
                    let fn_sig = span.as_ref().and_then(|s| {
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
                    });

                    // Show module path, function name, and highlighted signature if available
                    if let Some(sig) = fn_sig {
                        let highlighted = highlight_rust_signature(&sig, self.with_ansi);
                        format!("SPAN ENTERED: {}::{} - {}", target, span_name, highlighted)
                    } else {
                        format!("SPAN ENTERED: {}::{}", target, span_name)
                    }
                },
                "\"exit\"" => format!("SPAN EXITED: {}", span_name),
                "\"close\"" => format!("SPAN CLOSED: {}", span_name),
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
        
        // For SPAN ENTERED events, skip displaying fn_sig again since we showed it inline
        // (The old code that displayed all fields is removed)

        // Write file location
        if let Some(file) = event.metadata().file() {
            if let Some(line) = event.metadata().line() {
                write!(writer, "\n{}    at {}:{}", gutter_indent, file, line)?;
            }
        }

        writeln!(writer)
    }
}

static GLOBAL_INIT: Once = Once::new();

/// Get the target directory used by Cargo
///
/// This respects the workspace structure by checking:
/// 1. CARGO_TARGET_DIR environment variable (if set by user/CI)
/// 2. CARGO_MANIFEST_DIR at runtime to find workspace root
/// 3. Falls back to "target" relative to current directory
fn get_target_dir() -> PathBuf {
    // First check if CARGO_TARGET_DIR is set (user override or CI)
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(target_dir);
    }

    // During test execution, CARGO_MANIFEST_DIR points to the crate being tested
    // For workspace, we want to use the workspace root's target directory
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(&manifest_dir);
        let mut current = manifest_path.clone();

        // Walk up to find workspace root (has Cargo.toml with [workspace])
        while let Some(parent) = current.parent() {
            let workspace_toml = parent.join("Cargo.toml");
            if workspace_toml.exists() {
                // Check if this is a workspace root by looking for [workspace] section
                if let Ok(contents) = fs::read_to_string(&workspace_toml)
                    && contents.contains("[workspace]")
                {
                    return parent.join("target");
                }
            }
            current = parent.to_path_buf();
        }

        // If no workspace found, use the manifest dir's target
        return manifest_path.join("target");
    }

    // Fallback to relative "target" directory (current directory)
    PathBuf::from("target")
}

/// Configuration for test tracing
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Directory where log files are stored
    pub log_dir: PathBuf,
    /// Default log level for stdout
    pub stdout_level: Level,
    /// Default log level for file
    pub file_level: Level,
    /// Whether to log to stdout
    pub log_to_stdout: bool,
    /// Whether to log to file
    pub log_to_file: bool,
    /// Custom filter directives for stdout (e.g., "context_search=info,context_trace=error")
    pub stdout_filter_directives: Option<String>,
    /// Custom filter directives for file (e.g., "context_search=trace,context_trace=debug")
    pub file_filter_directives: Option<String>,
    /// Which spans to log
    pub span_events: FmtSpan,
}

impl Default for TracingConfig {
    fn default() -> Self {
        // Check environment variable to enable stdout logging
        // Usage: LOG_STDOUT=1 cargo test
        // or:    LOG_STDOUT=true cargo test
        let log_to_stdout = env::var("LOG_STDOUT")
            .map(|v| {
                v == "1"
                    || v.eq_ignore_ascii_case("true")
                    || v.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false);

        Self {
            log_dir: get_target_dir().join("test-logs"),
            stdout_level: Level::INFO,
            file_level: Level::TRACE,
            log_to_stdout,
            log_to_file: true,
            stdout_filter_directives: None,
            file_filter_directives: None,
            span_events: FmtSpan::ENTER | FmtSpan::CLOSE,
        }
    }
}

impl TracingConfig {
    /// Create config with custom log level for both stdout and file
    pub fn with_level(
        mut self,
        level: Level,
    ) -> Self {
        self.stdout_level = level;
        self.file_level = level;
        self
    }

    /// Create config with custom stdout log level
    pub fn with_stdout_level(
        mut self,
        level: Level,
    ) -> Self {
        self.stdout_level = level;
        self
    }

    /// Create config with custom file log level
    pub fn with_file_level(
        mut self,
        level: Level,
    ) -> Self {
        self.file_level = level;
        self
    }

    /// Create config with custom filter directives for both stdout and file
    ///
    /// Example: `"context_search::search=trace,context_trace=debug"`
    pub fn with_filter(
        mut self,
        filter: impl Into<String>,
    ) -> Self {
        let filter_str = filter.into();
        self.stdout_filter_directives = Some(filter_str.clone());
        self.file_filter_directives = Some(filter_str);
        self
    }

    /// Create config with custom filter directives for stdout only
    ///
    /// Example: `"context_search=info,context_trace=error"`
    pub fn with_stdout_filter(
        mut self,
        filter: impl Into<String>,
    ) -> Self {
        self.stdout_filter_directives = Some(filter.into());
        self
    }

    /// Create config with custom filter directives for file only
    ///
    /// Example: `"context_search=trace,context_trace=debug"`
    pub fn with_file_filter(
        mut self,
        filter: impl Into<String>,
    ) -> Self {
        self.file_filter_directives = Some(filter.into());
        self
    }

    /// Enable/disable stdout logging
    pub fn stdout(
        mut self,
        enabled: bool,
    ) -> Self {
        self.log_to_stdout = enabled;
        self
    }

    /// Enable/disable file logging
    pub fn file(
        mut self,
        enabled: bool,
    ) -> Self {
        self.log_to_file = enabled;
        self
    }

    /// Set which span events to log
    pub fn span_events(
        mut self,
        events: FmtSpan,
    ) -> Self {
        self.span_events = events;
        self
    }

    /// Set custom log directory
    pub fn log_dir(
        mut self,
        dir: impl Into<PathBuf>,
    ) -> Self {
        self.log_dir = dir.into();
        self
    }
}

/// Guard that handles test logging lifecycle
///
/// Automatically cleans up log files when the test succeeds (guard is dropped without panic).
pub struct TestTracing {
    log_file_path: Option<PathBuf>,
}

impl TestTracing {
    /// Initialize tracing for a test
    ///
    /// # Example
    /// ```no_run
    /// use context_trace::logging::tracing_utils::TestTracing;
    ///
    /// #[test]
    /// fn my_test() {
    ///     let _tracing = TestTracing::init("my_test");
    ///     // Test code with automatic tracing
    ///     // Log file will be deleted if test passes
    /// }
    /// ```
    pub fn init(test_name: &str) -> Self {
        Self::init_with_config(test_name, TracingConfig::default())
    }

    /// Initialize tracing with custom configuration
    pub fn init_with_config(
        test_name: &str,
        config: TracingConfig,
    ) -> Self {
        // Initialize global tracing only once
        GLOBAL_INIT.call_once(|| {
            // This is a placeholder - actual subscriber will be set per-test
        });

        // Create log directory
        if config.log_to_file {
            fs::create_dir_all(&config.log_dir).ok();
        }

        let log_file_path = if config.log_to_file {
            Some(config.log_dir.join(format!("{}.log", test_name)))
        } else {
            None
        };

        // Build separate filters for stdout and file
        let stdout_filter = if let Some(directives) =
            &config.stdout_filter_directives
        {
            EnvFilter::try_new(directives).unwrap_or_else(|_| {
                EnvFilter::new(config.stdout_level.as_str())
            })
        } else {
            // Check for LOG_FILTER first (preferred), then RUST_LOG, otherwise use stdout level
            env::var("LOG_FILTER")
                .ok()
                .and_then(|filter| EnvFilter::try_new(&filter).ok())
                .or_else(|| EnvFilter::try_from_default_env().ok())
                .unwrap_or_else(|| EnvFilter::new(config.stdout_level.as_str()))
        };

        let file_filter = if let Some(directives) =
            &config.file_filter_directives
        {
            EnvFilter::try_new(directives)
                .unwrap_or_else(|_| EnvFilter::new(config.file_level.as_str()))
        } else {
            // For file output, also check LOG_FILTER
            env::var("LOG_FILTER")
                .ok()
                .and_then(|filter| EnvFilter::try_new(&filter).ok())
                .unwrap_or_else(|| EnvFilter::new(config.file_level.as_str()))
        };

        // Create the subscriber without a global filter
        let registry = tracing_subscriber::registry();

        // Extract config values to avoid partial move issues
        let span_events = config.span_events;
        let log_to_stdout = config.log_to_stdout;

        // Build layers based on configuration
        match (log_to_stdout, log_file_path.as_ref()) {
            (true, Some(path)) => {
                // Both stdout and file - use multiple layers with separate filters
                let file =
                    fs::File::create(path).expect("Failed to create log file");

                let stdout_layer = tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_span_events(span_events.clone())
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false)
                    .with_level(false) // Level handled by custom formatter
                    .with_ansi(true)
                    .with_timer(CompactTimer::new())
                    .event_format(CompactFieldsFormatter::new(true))
                    .fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
                    .with_filter(stdout_filter);

                let file_layer = tracing_subscriber::fmt::layer()
                    .with_writer(move || {
                        file.try_clone().expect("Failed to clone file")
                    })
                    .with_span_events(span_events)
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false)
                    .with_level(false) // Level handled by custom formatter
                    .with_ansi(false)
                    .with_timer(CompactTimer::new())
                    .event_format(CompactFieldsFormatter::new(false))
                    .with_filter(file_filter);

                registry.with(stdout_layer).with(file_layer).try_init().ok();
            },
            (true, None) => {
                // Only stdout
                let stdout_layer = tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_span_events(span_events)
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false)
                    .with_level(false) // Level handled by custom formatter
                    .with_ansi(true)
                    .with_timer(CompactTimer::new())
                    .event_format(CompactFieldsFormatter::new(true))
                    .fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
                    .with_filter(stdout_filter);

                registry.with(stdout_layer).try_init().ok();
            },
            (false, Some(path)) => {
                // Only file
                let file =
                    fs::File::create(path).expect("Failed to create log file");

                let file_layer = tracing_subscriber::fmt::layer()
                    .with_writer(move || {
                        file.try_clone().expect("Failed to clone file")
                    })
                    .with_span_events(span_events)
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false)
                    .with_level(false) // Level handled by custom formatter
                    .with_ansi(false)
                    .with_timer(CompactTimer::new())
                    .event_format(CompactFieldsFormatter::new(false))
                    .fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
                    .with_filter(file_filter);

                registry.with(file_layer).try_init().ok();
            },
            (false, None) => {
                // No output - minimal subscriber
                registry.try_init().ok();
            },
        }

        tracing::info!(
            test_name = %test_name,
            log_file = ?log_file_path,
            "Test tracing initialized"
        );

        Self { log_file_path }
    }

    /// Get the path to the log file for this test
    pub fn log_file(&self) -> Option<&Path> {
        self.log_file_path.as_deref()
    }

    /// Explicitly keep the log file (don't delete on drop)
    ///
    /// Useful if you want to preserve logs even for passing tests
    pub fn keep_log(mut self) -> Self {
        self.log_file_path = None;
        self
    }
}

impl Drop for TestTracing {
    fn drop(&mut self) {
        // Check if we're unwinding (test panicked/failed)
        let is_panicking = std::thread::panicking();

        if !is_panicking {
            // Test passed - clean up log file
            if let Some(ref path) = self.log_file_path {
                tracing::info!(
                    log_file = %path.display(),
                    "Test passed, removing log file"
                );
                fs::remove_file(path).ok();
            }
        } else {
            // Test failed - keep log file
            if let Some(ref path) = self.log_file_path {
                eprintln!(
                    "\n❌ Test failed! Log file preserved at: {}",
                    path.display()
                );
            }
        }
    }
}

/// Convenience macro to initialize tracing for a test
///
/// # Example
/// ```no_run
/// use context_trace::init_test_tracing;
///
/// #[test]
/// fn my_test() {
///     init_test_tracing!();
///     // Your test code
/// }
/// ```
#[macro_export]
macro_rules! init_test_tracing {
    () => {{
        // Extract test name from function
        let test_name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            // Extract function name from the path
            name.strip_suffix("::f")
                .and_then(|s| s.split("::").last())
                .unwrap_or("unknown")
        };
        $crate::logging::tracing_utils::TestTracing::init(test_name)
    }};
    ($config:expr) => {{
        let test_name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f")
                .and_then(|s| s.split("::").last())
                .unwrap_or("unknown")
        };
        $crate::logging::tracing_utils::TestTracing::init_with_config(
            test_name, $config,
        )
    }};
}

/// Macro to create a tracing span with the function signature captured at compile time.
///
/// This macro wraps the standard `tracing::span!` macro and adds a `fn_sig` field
/// containing the function signature string.
///
/// # Usage
///
/// ```rust,ignore
/// use context_trace::span_with_sig;
///
/// fn my_function(x: i32, y: &str) -> Result<bool, Error> {
///     let _span = span_with_sig!(tracing::Level::INFO, "my_function");
///     // function body
/// }
/// ```
///
/// This will create a span with a field `fn_sig` containing the function signature.
#[macro_export]
macro_rules! span_with_sig {
    ($level:expr, $name:expr, $($fields:tt)*) => {{
        // Capture the function signature by stringifying the surrounding context
        // This is a compile-time operation
        let fn_sig = concat!(
            module_path!(),
            "::",
            $name,
            " @ ",
            file!(),
            ":",
            line!()
        );
        tracing::span!($level, $name, fn_sig = %fn_sig, $($fields)*)
    }};
    ($level:expr, $name:expr) => {{
        let fn_sig = concat!(
            module_path!(),
            "::",
            $name,
            " @ ",
            file!(),
            ":",
            line!()
        );
        tracing::span!($level, $name, fn_sig = %fn_sig)
    }};
}

/// Macro to wrap `#[instrument]` and automatically add function signature.
///
/// This macro generates the function signature string and adds it to the span fields.
///
/// # Usage
///
/// Instead of:
/// ```rust,ignore
/// #[instrument]
/// fn my_function(x: i32, y: &str) -> bool { }
/// ```
///
/// Use:
/// ```rust,ignore
/// #[instrument(fields(fn_sig = %fn_signature!("fn my_function(x: i32, y: &str) -> bool")))]
/// fn my_function(x: i32, y: &str) -> bool { }
/// ```
///
/// Or more simply, provide it as a string:
/// ```rust,ignore
/// #[instrument(fields(fn_sig = "fn my_function(x: i32, y: &str) -> bool"))]
/// fn my_function(x: i32, y: &str) -> bool { }
/// ```
#[macro_export]
macro_rules! fn_signature {
    ($sig:expr) => {
        $sig
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_builder() {
        let config = TracingConfig::default()
            .with_level(Level::TRACE)
            .with_filter("context_search=trace")
            .stdout(false)
            .file(true)
            .log_dir("custom/logs");

        assert_eq!(config.stdout_level, Level::TRACE);
        assert_eq!(config.file_level, Level::TRACE);
        assert_eq!(
            config.stdout_filter_directives,
            Some("context_search=trace".to_string())
        );
        assert_eq!(
            config.file_filter_directives,
            Some("context_search=trace".to_string())
        );
        assert!(!config.log_to_stdout);
        assert!(config.log_to_file);
        assert_eq!(config.log_dir, PathBuf::from("custom/logs"));
    }

    #[test]
    fn test_separate_filters() {
        let config = TracingConfig::default()
            .with_stdout_level(Level::INFO)
            .with_file_level(Level::TRACE)
            .with_stdout_filter("context_search=warn")
            .with_file_filter("context_search=trace,context_trace=debug");

        assert_eq!(config.stdout_level, Level::INFO);
        assert_eq!(config.file_level, Level::TRACE);
        assert_eq!(
            config.stdout_filter_directives,
            Some("context_search=warn".to_string())
        );
        assert_eq!(
            config.file_filter_directives,
            Some("context_search=trace,context_trace=debug".to_string())
        );
    }
}
