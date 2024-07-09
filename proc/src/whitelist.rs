use syn::{ItemImpl, Result, Token, LitStr};
use syn::parse::{Parse, ParseStream};
use proc_macro::TokenStream;
use quote::quote;

// This module provides a procedural macro attribute `#[whitelist(functions = "...")]`
// to specify a whitelist of functions allowed to mutate fields of a struct.

pub struct WhitelistArgs {
    pub functions: Vec<String>,
}

impl Parse for WhitelistArgs {
    // Custom implementation for parse_macro_input!.
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
pub fn whitelist_impl(args: WhitelistArgs, input: ItemImpl) -> TokenStream {
    let self_ty = &input.self_ty;
    let whitelist_functions = args.functions;

    // Convert whitelist functions to a form usable in the quote! macro.
    let whitelist_functions_iter = whitelist_functions.iter().map(|f| quote! { #f }).collect::<Vec<_>>();

    // Generate AST code to enforce the whitelist on struct fields restrictions.
    let generated_code = quote! {
        impl #self_ty {
            fn check_whitelist() {
                let caller_function = std::any::type_name::<Self>();
                let whitelist: &[&str] = &[#(#whitelist_functions_iter),*];

                if !whitelist.contains(&caller_function) {
                    panic!("Attempted mutation from unauthorized function!");
                }
            }
        }
    };

    generated_code.into()
}
