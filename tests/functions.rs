#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;

#[allow(dead_code)]
struct A {
    x: u32,
    y: u32,
}

fn consume(x: A) {}
assert_fields_type!(A: x: u32, y: u32);
assert_function_consumes_one!(consume: A);