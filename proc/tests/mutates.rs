#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;


#[derive(Default)]
pub struct MyStruct {
    field: i32,
}

impl MyStruct {
    #[mutates(MyStruct, field: ("self"))]
    pub fn allowed_mutate(&mut self) {
        self.field += 1;
    }
    
    pub fn allowed_mutate_multiple(&mut self) {
        self.field -= 1;
    }

    pub fn unauthorized_mutate(&mut self) {
        self.field = 0;
    }
}

// Your test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_mutate() {
        let mut instance = MyStruct::default();
        instance.allowed_mutate();
        assert_eq!(instance.field, 1);
    }
}
