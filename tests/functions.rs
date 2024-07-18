#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


mod simple_tests {
    #[allow(dead_code)]
    struct A {
        x: u32,
        y: u32,
    }

    fn consume(_x: A) {}
    
    #[test]
    fn test_assert_fields_type() {
        assert_fields_type!(A: x: u32, y: u32);
    }

    #[test]
    fn test_assert_function_consumes_one() {
        assert_function_consumes_one!(consume: A);
    }
}
