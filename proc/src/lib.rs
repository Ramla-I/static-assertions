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

// Procedural macros operate on the abstract syntax tree (AST) of the code.
// The quote crate helps in constructing these syntax trees. Using `quote`
// allows us to write code almost as if we're directly writing Rust code,
// facilitating seamless injection of variables like #input, #size, or #align.
// Without quote, we would have to manually construct token streams and manage
// all syntactic intricacies ourselves, which is prone to errors and cumbersome.
extern crate quote;
extern crate proc_macro;
extern crate syn;

mod private_fields;
mod size_align;
mod whitelist;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct, ItemImpl};


// Function-like macros in Rust take only one TokenStream parameter and return a TokenStream.
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

/// A procedural macro to assert that all fields in a struct are private.
#[proc_macro_attribute]
pub fn assert_private_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    private_fields::assert_private_fields_impl(input).into()
}

/// A procedural macro attribute to assert the size and alignment of a struct.
#[proc_macro_attribute]
pub fn assert_align_size(attr: TokenStream, item: TokenStream) -> TokenStream {
    // The release notes for syn v.2 say that AttributeArgs was removed, 
    // and says to ither use Punctuated<Meta, Token![,]> or build you own 
    // parse_macro_input! implementation. Follow the link for an example:
    // https://docs.rs/syn/latest/syn/meta/fn.parser.html#example
    let size_align = parse_macro_input!(attr as size_align::SizeAlign);
    let size_align::SizeAlign { size, align } = size_align;

    let input = parse_macro_input!(item as DeriveInput);

    size_align::assert_align_size_impl(size, align, &input).into()
}

/// A procedural macro attribute to hold the whitelist of functions.
/// Checks if a field of a type is only mutated in certain functions.
#[proc_macro_attribute]
pub fn mutatedby(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let fns = parse_macro_input!(attr as whitelist::WhitelistArgs);

    whitelist::assert_mutatedby_impl(&fns.functions, input).into()
}
