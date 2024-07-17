use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, ItemFn, Error};
use crate::fnwhitelist;

// Your procedural macro implementation
pub fn assert_function_consumes_impl(input: fnwhitelist::WhitelistArgs) -> TokenStream {
    let fn_name = &input.fn_name;
    let expected_arg_types = &input.arg_types;

    if expected_arg_types.is_empty() {
        return Error::new_spanned(&fn_name, "Expected at least one type").to_compile_error().into();
    }

    // Convert fn_name to a proc_macro::TokenStream
    let fn_name_token_stream: proc_macro::TokenStream = fn_name.to_token_stream().into();

    // Use syn::parse to parse the function
    let fn_item: ItemFn = match parse(fn_name_token_stream) {
        Ok(func) => func,
        Err(err) => {
            return Error::new_spanned(fn_name, format!("Failed to parse function: {}", err))
                .to_compile_error()
                .into();
        }
    };

    let fn_inputs = &fn_item.sig.inputs;

    // Collect the function's argument types
    let mut function_arg_types = Vec::new();
    for input in fn_inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let ty = &*pat_type.ty;
            function_arg_types.push(ty.clone());
        }
    }

    // Check if each expected type is present in the function's argument types
    for expected in expected_arg_types {
        if !function_arg_types.iter().any(|ty| ty.to_token_stream().to_string() == expected.to_token_stream().to_string()) {
            return Error::new_spanned(expected, format!("Type `{}` is not consumed by the function `{}`", expected.to_token_stream(), fn_name))
                .to_compile_error()
                .into();
        }
    }

    // If everything is valid, return an empty TokenStream (you can modify this to return something meaningful)
    quote! {}.into()
}
