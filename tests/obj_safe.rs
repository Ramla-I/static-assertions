#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


mod simple_tests {
    #[test]
    fn test_assert_obj_safe() {
        assert_obj_safe!(core::fmt::Debug, Send, Sync);
    }

    trait ObjSafe {}
    
    #[test]
    fn test_assert_obj_safe_trait() {
        assert_obj_safe!(ObjSafe);
    }
}
