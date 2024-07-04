//! [![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)
//!
//! <div align="center">
//!     <a href="https://crates.io/crates/proc_static_assertions">
//!         <img src="https://img.shields.io/crates/d/proc_static_assertions.svg" alt="Downloads">
//!     </a>
//!     <a href="https://travis-ci.org/nvzqz/static-assertions-rs">
//!         <img src="https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master" alt="Build Status">
//!     </a>
//!     <br><br>
//! </div>
//!
//! Procedural macro [compile-time] assertions as an extension of
//! [`static_assertions`].
//!
//! # Usage
//!
//! There's two main ways of using this crate: as a direct dependency or
//! indirect dependency (via [`static_assertions`]).
//!
//! ## Direct Dependency
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! proc_static_assertions = "0.0.0"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```
//! #[macro_use]
//! extern crate proc_static_assertions;
//! # fn main() {}
//! ```
//!
//! ## Indirect Dependency
//!
//! Add the following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! static_assertions = { version = "1.1.0", features = ["proc"] }
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```ignore
//! #[macro_use]
//! extern crate static_assertions;
//! ```
//!
//! This will also import all macros in `proc_static_assertions`.
//!
//! # Donate
//!
//! This project is made freely available (as in free beer), but unfortunately
//! not all beer is free! So, if you would like to buy me a beer (or coffee or
//! *more*), then consider supporting my work that's benefited your project
//! and thousands of others.
//!
//! <a href="https://www.patreon.com/nvzqz">
//!     <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
//! </a>
//! <a href="https://www.paypal.me/nvzqz">
//!     <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
//! </a>
//!
//! [`static_assertions`]: https://github.com/nvzqz/static-assertions-rs
//! [crate]: https://crates.io/crates/static_assertions
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [compile-time]: https://en.wikipedia.org/wiki/Compile_time

#![doc(html_root_url = "https://docs.rs/proc_static_assertions/0.0.0")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Icon.png"
)]
#![deny(missing_docs)]

extern crate syn;
extern crate quote;
extern crate proc_macro;

mod private_fields;
mod size_align;

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};


// Define Proc-Macros Below
// Function-like macros in Rust take only one TokenStream parameter and return a TokenStream.

/// A procedural macro to assert that all fields in a struct are private.
/// 
#[proc_macro_attribute]
pub fn assert_private_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    private_fields::assert_private_fields_impl(input)
}

/// A procedural macro attribute to assert the size and alignment of a struct.
/// 
#[proc_macro_attribute]
pub fn assert_align_size(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let size_align = parse_macro_input!(attr as size_align::SizeAlign);

    let size = size_align.size;
    let align = size_align.align;

    // Parse the input tokens and name
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Generate compile-time assertions
    let generated_code = quote! {
        #input

        const _: () = {
            #[test]
            fn size_of_struct() {
                let expected_size = #size;
                let actual_size = std::mem::size_of::<#name>();
                assert_eq!(expected_size, actual_size, "Struct {} does not have the expected size of {} bytes", stringify!(#name), expected_size);
            }

            #[test]
            fn align_of_struct() {
                let expected_align = #align;
                let actual_align = std::mem::align_of::<#name>();
                assert_eq!(expected_align, actual_align, "Struct {} does not have the expected alignment of {} bytes", stringify!(#name), expected_align);
            }
        };
    };

    // Return the generated code as a TokenStream
    generated_code.into()
}
