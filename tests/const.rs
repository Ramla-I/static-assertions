#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


#[cfg(test)]
mod simple_tests {
    #[test]
    fn test_const_assert() {
        const_assert!(true && (true != false));
        const_assert!((true && true) != false);
        
        #[allow(dead_code)]
        const FIVE: usize = 5;

        const_assert!(FIVE * 2 == 10);
        const_assert!(FIVE > 2);
    }
    
    #[test]
    fn test_const_assert_eq() {
        const_assert_eq!(false, false);
    }
}
