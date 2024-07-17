#![no_std]
#![deny(unsafe_code)]

extern crate proc_static_assertions;
use proc_static_assertions::assert_function_consumes;

pub struct ConsumedStruct;

mod simple_tests {
    use super::*;

    // Define your function within the module
    pub fn test_function(_arg1: i32, _arg2: ConsumedStruct, _arg3: &str) {}

    // Invoke the procedural macro with the correct syntax
    // Note: Use `crate::` to correctly reference the procedural macro from the root of your crate.
    assert_function_consumes!(simple_tests::test_function: i32, ConsumedStruct, &str);
}
