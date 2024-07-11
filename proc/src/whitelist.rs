use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    ItemImpl, ImplItem, LitStr, Result, Token,
};

/// Struct to hold the whitelist of functions.
pub struct WhitelistArgs {
    pub functions: Vec<String>,
}

impl Parse for WhitelistArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut functions = Vec::new();

        while !input.is_empty() {
            let function_name: LitStr = input.parse()?;
            functions.push(function_name.value());

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(WhitelistArgs { functions })
    }
}

/// Procedural macro implementation to insert the whitelist of functions into the global map.
pub fn whitelist_impl(args: WhitelistArgs, mut input: ItemImpl) -> TokenStream {
    let allowed_functions: Vec<String> = args.functions;

    // Iterate through items in the impl block
    for item in &mut input.items {
        // Check if the item is a method
        if let ImplItem::Fn(method) = item {
            let method_name = method.sig.ident.to_string();
            
            // Check if the method is not in the allowed functions list
            if !allowed_functions.contains(&method_name) {
                // Generate compile-time error if method is not allowed
                let error_message = format!(
                    "Attempted mutation from unauthorized function: {}",
                    method_name
                );
                method.block = syn::parse2(quote!({
                    compile_error!(#error_message);
                })).unwrap();
            }
        }
    }

    TokenStream::from(quote! {
        #input
    })
}
