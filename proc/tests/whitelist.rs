#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

#[derive(Default)]
struct MyStruct {
    field: i32,
}

// The functions allowed to mutate the field in MyStruct
#[whitelist(functions = "allowed_mutate")]
impl MyStruct {
    // Allowed function
    pub fn allowed_mutate(&mut self) {
        self.field += 1;
    }

    // Unauthorized function
    pub fn unauthorized_mutate(&mut self) {
        self.field = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_mutate() {
        let mut my_struct = MyStruct::default();
        my_struct.allowed_mutate();
        assert_eq!(my_struct.field, 1);
    }
}
