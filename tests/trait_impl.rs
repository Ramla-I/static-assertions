#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;
use core::ops::Range;

trait Tri<A: ?Sized, B: ?Sized, C: ?Sized> {}
impl<T, A: ?Sized, B: ?Sized, C: ?Sized> Tri<A, B, C> for T {}


mod simple_tests {
    use super::*;
    
    #[test]
    fn test_assert_impl_all() {
        assert_impl_all!(
            u64: Tri<[&'static u8], dyn Tri<dyn Send, dyn Sync, str>, (u16, u16)>
        );
        assert_impl_all!(u8: Send, Sync);
        assert_impl_all!(&'static [u8]: IntoIterator<Item=&'static u8>);
        assert_impl_all!(Range<u8>: Iterator<Item=u8>);
        assert_impl_all!([u8]: Send, Sync, AsRef<[u8]>);
        assert_impl_all!(str: Send, Sync, AsRef<[u8]>,);
    }

    #[test]
    fn test_assert_impl_any() {
        assert_impl_any!((): Send, Sync);
        assert_impl_any!((): Send, From<u8>);
        assert_impl_any!((): From<u8>, From<u16>, Send);
    }

    #[test]
    fn test_assert_impl_mut_ref() {
        assert_impl!(for(T: ?Sized) T: Clone | !Clone);
        assert_impl!(for('a, T: 'a) &'a mut T: !Copy);
    }

    #[test]
    fn test_assert_impl_phantom() {
        assert_impl!(for(T) PhantomData<T>: Clone);
    }

    #[test]
    fn test_assert_impl_copy() {
        assert_impl!(for(T: Copy) T: Clone);
    }
}

mod foo_tests {
    #[allow(dead_code)]
    struct Foo;
    trait A {}
    trait B {}
    trait C {}

    impl B for Foo {}


    #[test]
    fn test_assert_impl_one() {
        assert_impl_one!(Foo: A, B);
        assert_impl_one!(Foo: B, A);
        assert_impl_one!(Foo: B, C);
        assert_impl_one!(Foo: C, B);
        assert_impl_one!(Foo: A, B, C);
        assert_impl_one!(Foo: B, C, A);
        assert_impl_one!(Foo: C, A, B);
    }
}

mod bar_tests {
    #[derive(Clone)]
    struct Test;

    #[test]
    fn test_assert_impl() {
        assert_impl!(u8: (From<u16>) | (Into<u16>));
        assert_impl!((): (From<u8>) | (From<u16>) | Send);
        assert_impl!((): (!From<u8>) & !(From<u16>) & Send);
        assert_impl!((): Copy | Clone);
        assert_impl!((): Copy & Clone);
        assert_impl!(Test: Copy | Clone);
        assert_impl!(Test: !Copy | Clone);
        assert_impl!(Test: !Copy & Clone);
        assert_impl!(Test: !Copy & (Clone));
        assert_impl!(Test: !(Copy) & Clone);
        assert_impl!(Test: !(!Clone));
        assert_impl!(Test: !(Copy) & !(!Clone));
        assert_impl!(Test: !(Copy & Clone));
        assert_impl!(str: !Copy & !Clone);
    }
}

mod box_tests {
    #[derive(Clone)]
    struct Box<T>(T);

    #[test]
    fn test_assert_impl_box() {
        assert_impl!(for(T: Clone) Box<T>: Clone);
        assert_impl!(for(T: Clone + Send) Box<T>: Clone & Send);
        assert_impl!(for(T) Box<T>: (From<T>) & (Into<T>));
    }
}
