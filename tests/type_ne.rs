#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


#[cfg(test)]
mod simple_tests {
    #[test]
    fn test_assert_type_ne_all() {
        assert_type_ne_all!(u8, u16, u32);
    }
}
