#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

#[derive(Default)]
struct MyStruct {
    field: i32,
}

#[mutatedby("allowed_mutate", "allowed_mutate2")]
impl MyStruct {
    pub fn allowed_mutate(&mut self) {
        self.field += 1;
        Self::__MyStruct_field_mutate_check("allowed_mutate");
    }
    
    pub fn allowed_mutate_multiple(&mut self) {
        self.field -= 1;
        Self::__MyStruct_field_mutate_check("allowed_mutate");
    }

    pub fn unauthorized_mutate(&mut self) {
        self.field = 0;
        Self::__MyStruct_field_mutate_check("unauthorized_mutate");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_mutate() {
        let mut instance = MyStruct::default();
        instance.allowed_mutate();
        assert_eq!(instance.field, 1);
    }

    #[test]
    fn test_allowed_mutate_multiple() {
        let mut instance = MyStruct::default();
        instance.allowed_mutate_multiple();
        assert_eq!(instance.field, -1);
    }

    #[test]
    #[should_panic(expected = "Unauthorized function trying to mutate fields in MyStruct: unauthorized_mutate")]
    fn test_unauthorized_mutate() {
        let mut instance = MyStruct::default();
        instance.unauthorized_mutate();
    }
}
