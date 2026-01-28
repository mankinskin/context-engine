use std::path::{Path, PathBuf};

use syn::spanned::Spanned;
use syn::{Attribute, File, ImplItem, Item, ItemFn, ItemImpl, ItemMod, TraitItem};

use crate::analyzer::FunctionInfo;

/// Collects all function definitions from a Rust file
pub struct FunctionCollector {
    file_path: PathBuf,
    /// Current module path stack
    module_stack: Vec<String>,
    /// Collected functions
    pub functions: Vec<FunctionInfo>,
}

impl FunctionCollector {
    pub fn new(file_path: &Path) -> Self {
        Self {
            file_path: file_path.to_path_buf(),
            module_stack: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn visit_file(&mut self, file: &File) {
        // Extract crate/module name from file path
        let module_name = self.extract_module_name();
        if !module_name.is_empty() {
            self.module_stack.push(module_name);
        }

        for item in &file.items {
            self.visit_item(item);
        }
    }

    fn extract_module_name(&self) -> String {
        // Try to extract module name from file path
        self.file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| {
                if s == "mod" || s == "lib" || s == "main" {
                    // Use parent directory name instead
                    self.file_path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string()
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_default()
    }

    fn current_module_path(&self) -> String {
        self.module_stack.join("::")
    }

    fn has_instrument_attr(attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|seg| seg.ident == "instrument")
        })
    }

    fn add_function(&mut self, name: &str, attrs: &[Attribute], start_line: usize, end_line: usize) {
        let has_instrument = Self::has_instrument_attr(attrs);

        self.functions.push(FunctionInfo {
            file: self.file_path.clone(),
            module_path: self.current_module_path(),
            name: name.to_string(),
            start_line,
            end_line,
            tracing_count: 0, // Will be filled in later
            has_instrument,
        });
    }

    fn visit_item(&mut self, item: &Item) {
        match item {
            Item::Fn(item_fn) => {
                self.visit_item_fn(item_fn);
            }
            Item::Mod(item_mod) => {
                self.visit_item_mod(item_mod);
            }
            Item::Impl(item_impl) => {
                self.visit_item_impl(item_impl);
            }
            Item::Trait(item_trait) => {
                // Visit trait with default implementations
                self.module_stack.push(item_trait.ident.to_string());
                for item in &item_trait.items {
                    if let TraitItem::Fn(method) = item {
                        if method.default.is_some() {
                            // Only count methods with default implementations
                            let start = method.attrs.first().map(|a| a.span().start().line).unwrap_or_else(|| method.sig.span().start().line);
                            let end = method.default.as_ref().map(|b| b.span().end().line).unwrap_or(start);
                            self.add_function(
                                &method.sig.ident.to_string(),
                                &method.attrs,
                                start,
                                end,
                            );
                        }
                    }
                }
                self.module_stack.pop();
            }
            _ => {}
        }
    }

    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        let start = item_fn
            .attrs
            .first()
            .map(|a| a.span().start().line)
            .unwrap_or_else(|| item_fn.sig.span().start().line);
        let end = item_fn.block.span().end().line;

        self.add_function(&item_fn.sig.ident.to_string(), &item_fn.attrs, start, end);

        // Visit nested functions
        for stmt in &item_fn.block.stmts {
            if let syn::Stmt::Item(Item::Fn(nested_fn)) = stmt {
                self.visit_item_fn(nested_fn);
            }
        }
    }

    fn visit_item_mod(&mut self, item_mod: &ItemMod) {
        self.module_stack.push(item_mod.ident.to_string());

        if let Some((_, items)) = &item_mod.content {
            for item in items {
                self.visit_item(item);
            }
        }

        self.module_stack.pop();
    }

    fn visit_item_impl(&mut self, item_impl: &ItemImpl) {
        // Get the type name being implemented
        let type_name = quote::quote!(#item_impl.self_ty).to_string().replace(' ', "");

        // If it's a trait impl, include trait name
        let impl_name = if let Some((_, trait_path, _)) = &item_impl.trait_ {
            let trait_name = quote::quote!(#trait_path).to_string().replace(' ', "");
            format!("<{}as{}>", type_name, trait_name)
        } else {
            type_name
        };

        self.module_stack.push(impl_name);

        for item in &item_impl.items {
            if let ImplItem::Fn(method) = item {
                let start = method
                    .attrs
                    .first()
                    .map(|a| a.span().start().line)
                    .unwrap_or_else(|| method.sig.span().start().line);
                let end = method.block.span().end().line;

                self.add_function(&method.sig.ident.to_string(), &method.attrs, start, end);
            }
        }

        self.module_stack.pop();
    }
}
