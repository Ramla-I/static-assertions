use proc_macro2::Ident;
use proc_macro::TokenStream;
use quote::{quote_spanned, quote};
use syn::spanned::Spanned;
use syn::{Stmt, visit_mut::{self, VisitMut}};

pub fn assert_calledby_impl(allowed_functions: &[String], input: syn::ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;
    let fn_span = input.span();

    // Generate a strct unique function name for the manual mutation check function at the callsite.
    //
    // NOTE: We cannot automatically check the callsite fn, unless use Rust Lints like Clippy.
    // However, that would require forking the repo and directly contributing there.
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

pub fn assert_callsite_impl(input: syn::ItemFn) -> proc_macro2::TokenStream {
    let fn_name = input.sig.ident.clone();
    let fn_name_str = fn_name.to_string();
    
    // Visitor to find function calls
    struct FnCallVisitor {
        calls: Vec<Ident>,
    }
    
    impl VisitMut for FnCallVisitor {
        fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
            if let Stmt::Expr(expr, _) = stmt {
                if let syn::Expr::MethodCall(method_call) = expr {
                    self.calls.push(method_call.method.clone());
                }
            }
            visit_mut::visit_stmt_mut(self, stmt);
        }
    }
    
    let mut visitor = FnCallVisitor { calls: vec![] };
    let mut input_clone = input.clone();
    visitor.visit_item_fn_mut(&mut input_clone);
    
    let _calls = visitor.calls;
    
    let callsite_code = quote! {
        Self::__callsite(#fn_name_str);
    };
    let injected_code: Stmt = syn::parse_quote!(#callsite_code);
    
    let block = &input.block;
    let stmts = &block.stmts;
    
    let new_block = quote! {
        {
            #injected_code
            #(#stmts)*
        }
    };

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;

    let result = quote! {
        #(#attrs)*
        #vis #sig #new_block
    };

    result.into()
}
