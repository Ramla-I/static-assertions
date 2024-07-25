use syn::{ExprMethodCall, ItemFn, Expr, ExprCall, Stmt};
use proc_macro2::TokenStream as ProcTokenStream;
use proc_macro::TokenStream;
use quote::quote;

pub fn assert_call_impl(whitelist: &[String], function: &ItemFn) -> ProcTokenStream {
    let mut errors = Vec::new();
    let block = &function.block;

    for stmt in &block.stmts {
        if let Stmt::Expr(expr, _) = stmt {
            check_expr_for_calls(expr, whitelist, &mut errors);
        }
    }

    if !errors.is_empty() {
        let mut error_message = String::from("Function contains calls to non-whitelisted functions:\n");
        for error in &errors {
            error_message.push_str(&format!(" - {}\n", error.message));
        }

        return TokenStream::from(quote! {
            compile_error!(#error_message);
        }).into();
    }

    TokenStream::from(quote! { #function }).into()
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Error { message }
    }
}

fn check_expr_for_calls(expr: &Expr, whitelist: &[String], errors: &mut Vec<Error>) {
    // NOTE: Assumes that Rustc lint will catch any repetative names between instances.
    
    match expr {
        Expr::Call(ExprCall { func, .. }) => {
            if let Expr::Path(path) = &**func {
                let func_name = path.path.segments.last().map(|seg| seg.ident.to_string());
                if let Some(func_name) = func_name {
                    if !whitelist.contains(&func_name) {
                        errors.push(Error::new(
                            format!("Function calls a non-whitelisted function: `{}`", func_name),
                        ));
                    }
                }
            }
        }
        
        Expr::MethodCall(ExprMethodCall { method, .. }) => {
            let method_name = method.to_string();
            if !whitelist.contains(&method_name) {
                errors.push(Error::new(
                    format!("Method call to a non-whitelisted method: `{}`", method_name),
                ));
            }
        }

        _ => {} // Handle other expression types if I missed anything
        // Currently works for method and simple function calls 
    }
}
