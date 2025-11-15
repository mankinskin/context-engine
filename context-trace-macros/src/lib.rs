use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input,
    ItemFn,
};

#[proc_macro_attribute]
pub fn instrument_sig(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Convert args to string to check for fields
    let args_str = args.to_string();
    let has_fields = args_str.contains("fields");

    // Extract the function signature as a string
    let fn_sig = extract_function_signature(&input_fn);

    // Parse args as TokenStream2 for manipulation
    let args: TokenStream2 = args.into();

    // Build the new attribute arguments with fn_sig added to or merged with fields
    let new_args = if has_fields {
        // Need to merge with existing fields
        // We'll parse the tokens and insert fn_sig into the fields argument
        merge_fn_sig_into_fields(args, &fn_sig)
    } else {
        // No existing fields, just add fields(fn_sig = ...)
        if args.is_empty() {
            quote! { fields(fn_sig = #fn_sig) }
        } else {
            quote! { #args, fields(fn_sig = #fn_sig) }
        }
    };

    // Generate the output with #[instrument] attribute
    let output = quote! {
        #[tracing::instrument(#new_args)]
        #input_fn
    };

    output.into()
}

/// Merge fn_sig field into existing fields() argument
fn merge_fn_sig_into_fields(
    args: TokenStream2,
    fn_sig: &str,
) -> TokenStream2 {
    use proc_macro2::{
        Delimiter,
        Group,
        TokenTree,
    };

    let mut result = Vec::new();
    let mut iter = args.into_iter().peekable();

    while let Some(token) = iter.next() {
        match &token {
            TokenTree::Ident(ident) if ident.to_string() == "fields" => {
                // Found fields, add it to result
                result.push(token.clone());

                // Next should be the parenthesized group
                if let Some(TokenTree::Group(group)) = iter.next() {
                    if group.delimiter() == Delimiter::Parenthesis {
                        // Create new group with fn_sig prepended
                        let mut new_stream = TokenStream2::new();

                        // Add fn_sig = "..."
                        new_stream.extend(quote! { fn_sig = #fn_sig });

                        // Add comma if group is not empty
                        if !group.stream().is_empty() {
                            new_stream.extend(quote! { , });
                            new_stream.extend(group.stream());
                        }

                        // Create new group
                        let new_group =
                            Group::new(Delimiter::Parenthesis, new_stream);
                        result.push(TokenTree::Group(new_group));
                    } else {
                        result.push(TokenTree::Group(group));
                    }
                }
            },
            _ => result.push(token),
        }
    }

    result.into_iter().collect()
}

/// An attribute macro that wraps `#[instrument]` and automatically adds the function signature.
///
/// This macro extracts the function signature at compile time and adds it as a `fn_sig` field
/// to the tracing span, then delegates to the standard `#[instrument]` macro.
///
/// # Usage
///
/// ```rust,ignore
/// use context_trace_macros::instrument_sig;
///
/// #[instrument_sig]
/// fn my_function(x: i32, y: &str) -> Result<bool, Error> {
///     // function body
/// }
///
/// // With additional instrument parameters:
/// #[instrument_sig(skip(y), level = "debug")]
/// fn my_function(x: i32, y: &str) -> Result<bool, Error> {
///     // function body
/// }
/// ```
///
/// The generated code will include `fn_sig = "fn my_function(x: i32, y: &str) -> Result<bool, Error>"`
/// as a field in the span.

/// Extract the function signature as a readable string
fn extract_function_signature(func: &ItemFn) -> String {
    let sig = &func.sig;
    let fn_name = &sig.ident;

    // Build parameter list - normalize whitespace aggressively
    let mut params = Vec::new();
    for input in &sig.inputs {
        let param_str = quote!(#input).to_string();
        params.push(normalize_spacing(&param_str));
    }
    let params_str = params.join(", ");

    // Build return type - normalize whitespace
    let return_type = match &sig.output {
        syn::ReturnType::Default => String::new(),
        syn::ReturnType::Type(_, ty) => {
            let ty_str = quote!(#ty).to_string();
            format!(" -> {}", normalize_spacing(&ty_str))
        },
    };

    // Handle async
    let async_str = if sig.asyncness.is_some() {
        "async "
    } else {
        ""
    };

    // Handle const
    let const_str = if sig.constness.is_some() {
        "const "
    } else {
        ""
    };

    // Handle unsafe
    let unsafe_str = if sig.unsafety.is_some() {
        "unsafe "
    } else {
        ""
    };

    format!(
        "{}{}{}fn {}({}){}",
        const_str, async_str, unsafe_str, fn_name, params_str, return_type
    )
}

/// Normalize spacing in type signatures - remove extra whitespace around punctuation
fn normalize_spacing(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        // Remove spaces around operators
        .replace("& ", "&")
        .replace("* ", "*")
        .replace(" &", "&")
        .replace(" *", "*")
        // Handle colons
        .replace(" :", ":")
        .replace(" ::", "::")
        .replace(":: ", "::")
        // Handle angle brackets
        .replace(" <", "<")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace("> ", ">")
        // Handle commas
        .replace(" ,", ",")
}
