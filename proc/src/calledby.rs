use proc_macro::TokenStream;
use quote::{quote_spanned, quote};
use syn::spanned::Spanned;

pub fn assert_calledby_impl(allowed_functions: &[String], input: syn::ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;
    let fn_span = input.span();

    // Generate a strct unique function name for the manual mutation check function at the callsite.
    //
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
    //
    // INFO: Potential improvement would be automize this process by some global proc-macro #![assert_callsite]
    // that would go through all the functions and check for usage of another struct related functions, so it 
    // could generate injected AST at compile time type of `MyStruct::__callsite("function_name");`.
    let check_fn_ident = syn::Ident::new("__callsite", fn_span);

    // Generate the list of unauthorized checks.
    let unauthorized_check = allowed_functions.iter().map(|allowed_fn| {
        let allowed_fn_ident = syn::Ident::new(allowed_fn, fn_span);
        quote_spanned! { fn_span =>
            if stringify!(#allowed_fn_ident) == caller_name {
                return;
            }
        }
    });

    // Generate the injected AST for the callsite check function.
    let expanded = quote! {
        #input

        #[doc(hidden)]
        pub fn #check_fn_ident(caller_name: &str) {
            #(#unauthorized_check)* // For the DEMO purpose I keep it as panic, fix to compiler_error().
            panic!("Unauthorized function trying to call {}: {}", stringify!(#fn_name), caller_name);
        }
    };

    TokenStream::from(expanded)
}
