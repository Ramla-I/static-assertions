#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

#[allow(dead_code)]
enum Foo {
    A { x: u8, y: u8 },
    B(u8),
}

assert_fields!(Foo::A: x);
assert_fields!(Foo::A: x, x);
assert_fields!(Foo::A: x, y, x);

// TODO: Make tuple field access possible
// assert_fields!(Foo::B, 0);

mod m {
    #[allow(dead_code)]
    pub struct Bar<T: ?Sized> {
        pub nul: (),
        pub inner: T,
    }
}

#[allow(dead_code)]
use m::Bar as Baz;

assert_fields!(m::Bar<str>: inner, nul);
assert_fields!(Baz<dyn Send>: inner, nul);

#[allow(dead_code)]
struct A {
    x: u32,
    y: u32,
}
assert_fields_type!(A: x: u32, y: u32);
assert_fields_type!(A: x: u32);

// TO DO: make this work with tuple structs
// struct B(A);
// assert_fields_type!(B: 0: A);
