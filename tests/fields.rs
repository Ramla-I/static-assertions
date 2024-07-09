#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


mod foo_tests {
    #[allow(dead_code)]
    enum Foo {
        A { x: u8, y: u8 },
        B(u8),
    }

    #[test]
    fn test_assert_fields() {
        assert_fields!(Foo::A: x);
        assert_fields!(Foo::A: x, x);
        assert_fields!(Foo::A: x, y, x);
    }

    // TODO: Make tuple field access possible
    // #[test]
    // fn test_assert_fields() {
    //     assert_fields!(Foo::B, 0);
    // }
}


mod m {
    #[allow(dead_code)]
    pub struct Bar<T: ?Sized> {
        pub nul: (),
        pub inner: T,
    }
}

mod bar_tests {
    use super::*;
    use m::Bar as Baz;

    #[allow(dead_code)]
    struct A {
        x: u32,
        y: u32,
    }

    #[test]
    fn test_assert_fields() {
        assert_fields!(m::Bar<str>: inner, nul);
        assert_fields!(Baz<dyn Send>: inner, nul);
    }

    #[test]
    fn test_assert_fields_type() {
        assert_fields_type!(A: x: u32, y: u32);
    }
}
