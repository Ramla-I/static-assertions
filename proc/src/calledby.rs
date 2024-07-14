use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemFn, Ident,
    spanned::Spanned
};

pub fn assert_calledby_impl(allowed_functions: &Vec<String>, input: ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    // Generate a unique function name for the manual mutation check function at the callsite.
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
    let check_fn_name = format!("__{}_callsite", fn_name);
    let check_fn_ident = Ident::new(&check_fn_name, input.span());

    // Generate injected AST for the callsite check.
    let unauthorized_check = allowed_functions.iter().map(|allowed_fn| {
        let fn_ident = Ident::new(allowed_fn, input.span());
        // Quote is important to keep the correct order.
        quote! {
            if stringify!(#fn_ident) == caller_name {
                // Return before panic.
                return;
            }
        }
    });

    let expanded = quote! {
        #input

        #[doc(hidden)]
        pub fn #check_fn_ident(caller_name: &str) {
            #(#unauthorized_check)* // If not returned.
            panic!("Unauthorized function trying to call {}: {}", stringify!(#fn_name), caller_name);
        }
    };

    TokenStream::from(expanded)
}