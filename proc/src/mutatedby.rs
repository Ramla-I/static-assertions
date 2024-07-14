use proc_macro::TokenStream;
use quote::{ToTokens, quote};


pub fn assert_mutatedby_impl(allowed_functions: &[String], input: syn::ItemImpl) -> TokenStream {
    let struct_name = input.self_ty.clone().into_token_stream();

    // Generate a unique function name for the manual mutation check function at the callsite.
    //
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
    //
    // INFO: Potential improvement would be automize this process by some global proc-macro #![assert_mutates]
    // that would go through all the functions and check its arguments for existance of &mut MyStruct, so
    // it could generate injected AST at compile time type of `MyStruct::__mutates("function_name");`.
    let check_fn_ident = syn::Ident::new("__mutates", proc_macro2::Span::call_site());

    // Generate injected AST for the mutation check.
    let check_fn_code = quote! {
        impl #struct_name {
            pub fn #check_fn_ident(caller_name: &str) {
                match caller_name {
                    #(#allowed_functions => return,)* // For the DEMO purpose I keep it as panic, fix to compiler_error().
                    _ => panic!("Unauthorized function trying to mutate fields in {}: {}", stringify!(#struct_name), caller_name),
                }
            }
        }
    };

    let mut output = input.into_token_stream();
    output.extend(check_fn_code);

    output.into()
}
