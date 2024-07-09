#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions;


#[cfg(test)]
mod simple_tests {
    #[test]
    fn test_assert_size_eq() {
        assert_size_eq!(u8, u8);
        assert_size_eq!([u8; 2], u16);
        assert_size_eq!(u8, u8, (u8,), [u8; 1]);
        assert_size_eq!([u8; 2], u16, (u8, u8));
        assert_size_eq!([u8; 4], u32, (u16, u8, u8), (u16, u16));
    }

    #[test]
    fn test_assert_size_eq_val() {
        assert_size_eq_val!(0u8, 0u8);
        assert_size_eq_val!([0u8; 2], 0u16);
        assert_size_eq_val!([0u8; 2], 0u16, (0u8, 0u8));
        assert_size_eq_val!([0u8; 4], 0u32, (0u16, 0u8, 0u8), (0u16, 0u16));
    }

    #[test]
    fn test_assert_size_eq_ptr() {
        let mut x = 0;
        assert_size_eq_ptr!(&mut x, &0);
        x = 20;
        assert_size_eq_ptr!(&mut x, &0);
    }

    // Should fail to compile (un-comment to test manually):
    // assert_size_eq!(u8, u16);
    // assert_size_eq_val!(0u8, 0u16);
}


#[cfg(test)]
mod assoc_type_tests {
    trait Trait {
        type AssocItem: ?Sized;
    }

    impl<T: ?Sized> Trait for T {
        type AssocItem = Self;
    }

    #[allow(dead_code)]
    struct Value;
    
    #[test]
    fn test_assoc_type_item() {
        assert_size_eq!(<Value as Trait>::AssocItem, Value);
    }
}


// Private DropCounter
mod dc {
    /// A type that acts like somewhat of a reference counter.
    pub struct DropCounter<'a> {
        count: &'a mut i32,
    }

    impl<'a> DropCounter<'a> {
        pub fn new(count: &'a mut i32) -> DropCounter<'a> {
            *count += 1;
            DropCounter { count }
        }

        pub fn count(&self) -> i32 {
            *self.count
        }
    }

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            *self.count -= 1
        }
    }
}


mod dc_tests {
    use dc::*;

    /// A type that panics on drop.
    pub struct PanicDrop<T>(T);

    impl<T> Drop for PanicDrop<T> {
        fn drop(&mut self) {
            panic!("Dropped!");
        }
    }

    
    #[test]
    fn test_eq_size_no_drop() {
        assert_size_eq!(u32, PanicDrop<u32>);
        assert_size_eq!(PanicDrop<u32>, u32);
        assert_size_eq!(PanicDrop<u32>, PanicDrop<u32>);
    }

    #[test]
    fn test_eq_size_drop_count() {
        let mut count = 0;
        {
            let dc = DropCounter::new(&mut count);
            assert_eq!(dc.count(), 1);
            assert_size_eq_val!(dc, 0usize);
            assert_eq!(dc.count(), 1);
            assert_size_eq_val!(dc, 0usize, dc);
            assert_eq!(dc.count(), 1);
        }
        assert_eq!(count, 0);

        assert_size_eq_val!(DropCounter::new(&mut count), 0usize);
        assert_eq!(count, 0);
    }
}
