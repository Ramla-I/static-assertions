use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, Visibility};

pub fn assert_private_fields_impl(input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let mut all_private = true;

    if let Fields::Named(ref fields) = input.fields {
        for field in fields.named.iter() {
            if let Visibility::Public(_) = field.vis {
                all_private = false;
                break;
            }
        }
    }

    if all_private {
        TokenStream::new()
    } else {
        let expanded = quote! {
            compile_error!("Struct {} has public fields, all fields must be private.", stringify!(#struct_name));
        };
        TokenStream::from(expanded)
    }
}
