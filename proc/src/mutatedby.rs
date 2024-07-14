use proc_macro::TokenStream;
use quote::{ToTokens, quote};


pub fn assert_mutatedby_impl(allowed_functions: &[String], input: syn::ItemImpl) -> TokenStream {
    // Get the name of the Struct/Enum being implemented.
    let struct_name = input.self_ty.clone().into_token_stream();
    let struct_name_str = struct_name.to_string();

    // Generate a unique function name for the manual mutation check function at the callsite.
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
    let check_fn_name = format!("__{}_mutates", struct_name_str);
    let check_fn_ident = syn::Ident::new(&check_fn_name, proc_macro2::Span::call_site());

    // Generate injected AST for the mutation check.
    let check_fn_code = quote! {
        impl #struct_name {
            pub fn #check_fn_ident(caller_name: &str) {
                match caller_name {
                    #(#allowed_functions => return,)*
                    _ => panic!("Unauthorized function trying to mutate fields in {}: {}", stringify!(#struct_name), caller_name),
                }
            }
        }
    };

    let mut output = input.into_token_stream();
    output.extend(check_fn_code);

    output.into()
}


