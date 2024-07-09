#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


#[allow(dead_code)]
type X = u8;

#[allow(unused_parens)]
mod m {
    assert_type_eq_all!(super::X, u8, (super::X));
}


#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn test_assert_type_eq_all() {
        
        assert_type_eq_all!([u8], [u8]);
        assert_type_eq_all!(X, u8);
    }
}