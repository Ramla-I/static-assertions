#![allow(dead_code)]
#![allow(unused)]

use syn::{
    Block, Local, ExprIf, ExprWhile, Attribute,
    ExprForLoop, ExprMethodCall, ExprBlock, FnArg,
    punctuated::Punctuated, token::Comma,
    ItemFn, Pat, Type, Stmt, TypePath};
use proc_macro2::TokenStream as ProcTokenStream;
use crate::field_whitelist::WhitelistArgs;
use proc_macro::TokenStream;
use quote::quote;


pub fn assert_mutate_impl(macro_data: &WhitelistArgs, function: &ItemFn) -> ProcTokenStream {
    // NOTE: For better code layout, we will require separate proc-macro for each field
    // to be whitelisted type of #[struct_name, field_name: (func1, func2, func3, ...)].
    
    let whitelist = &macro_data.values;
    let field_name = &macro_data.field_name;
    let struct_name = &macro_data.struct_name;

    let mut errors: Vec<Error> = Vec::new();
    let inputs: &Punctuated<FnArg, Comma> = &function.sig.inputs;
    let block: &Box<Block> = &function.block;

    // Entry point: figure out the instance name by
    // exploring the function input arguments.
    let instance_name = extract_instance_name(inputs, struct_name);
    println!("=============== {:?}", instance_name);
    if instance_name.is_none() {
        // No need to handle check if input is not mutable since
        // this will be automatically checked by Rust compiler.
        return TokenStream::from(quote! { #function }).into();
    }

    // Start the recursive checking from the function body.
    check_block_for_calls(block, whitelist, &mut errors);

    if !errors.is_empty() {
        let mut error_message = String::from("Function contains mutations to a non-whitelisted struct fields:\n");

        for error in &errors {
            error_message.push_str(&format!(" - {}\n", error.message));
        }

        return TokenStream::from(quote! {
            compile_error(#error_message);
        }).into();
    }

    TokenStream::from(quote! { #function }).into()
}

/// Extracts the instance name from function arguments if it matches the specified struct name.
fn extract_instance_name(inputs: &Punctuated<FnArg, Comma>, struct_name: &str) -> Option<String> {
    for arg in inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let pat = &*pat_type.pat;
                let ty = &*pat_type.ty;

                println!("Found argument with type: {:?}", quote::quote! { #ty });

                match pat {
                    Pat::Ident(pat_ident) => {
                        println!("Argument pattern: {:?}", pat_ident.ident);
                        if let Type::Path(TypePath { path, .. }) = ty {
                            if path.is_ident(struct_name) {
                                return Some(pat_ident.ident.to_string());
                            }
                        }
                    }
                    _ => {
                        // Provide a description for non-identifier patterns
                        println!("Non-identifier pattern detected");
                    }
                }
            }
            _ => {
                // Handle non-typed arguments
                println!("Non-typed argument: {:?}", quote::quote! { #arg });
            }
        }
    }
    None
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

fn check_whitelist(
    name: &str, 
    whitelist: &[String], 
    errors: &mut Vec<Error>, 
    message: &str
) {
    if !whitelist.contains(&name.to_string()) {
        // Custom assetion based on whitelist data and found AST calls.
        errors.push(Error::new(format!("{}: `{}`", message, name)));
    }
}


fn print_ast<T>(item: &T, label: &str)
where
    T: quote::ToTokens,
{
    // A helper function to print AST tokens;
    let tokens: ProcTokenStream = quote! { #item };
    let item_string = tokens.to_string();
    println!("{}: {}", label, item_string);
}

// Recursive check all statements in the block.
fn check_block_for_calls(block: &Block, _whitelist: &[String], _errors: &mut Vec<Error>) {
    for smts in &block.stmts {
        match smts {
            Stmt::Expr(expr, _) => {
                print_ast(expr, "Found Expression");
                // Explore Netsted Expression for new calls.
                // check_expr_for_calls(expr, whitelist, errors);
            }
            Stmt::Local(Local { init, .. }) => {
                // Handle variable definitions.
                if let Some(init) = init {
                    print_ast(&init.expr, "Found Initialization Expression");
                    // Explore the local `let i = {__callsite__};` initialization.
                    // check_expr_for_calls(&init.expr, whitelist, errors);
                }
            }
            _ => {}
        }
    }
}