#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

pub struct MyStruct;

impl MyStruct {
    #[calledby("allowed_caller", "allowed_caller_multiple", "outside_caller")]
    pub fn target_function(&self) -> &str {
        return "Hello World";
    }

    pub fn allowed_caller(&self) {
        Self::__callsite("allowed_caller");
        self.target_function();
    }

    pub fn allowed_caller_multiple(&self) {
        Self::__callsite("allowed_caller_multiple");
        self.target_function();
    }

    pub fn unauthorized_caller(&self) {
        Self::__callsite("unauthorized_caller");
        self.target_function();
    }
}

pub fn outside_caller(my_struct: &MyStruct) {
    MyStruct::__callsite("outside_caller");
    my_struct.target_function();
}

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test] 
    fn test_allowed_caller() {
        let my_struct = MyStruct;
        my_struct.allowed_caller();
    }

    #[test]
    fn test_allowed_caller_multiple() {
        let my_struct = MyStruct;
        my_struct.allowed_caller_multiple();
    }

    #[test]
    fn test_outside_caller() {
        let my_struct = MyStruct;
        outside_caller(&my_struct);
    }

    #[test]
    #[should_panic(expected = "Unauthorized function trying to call target_function: unauthorized_caller")]
    fn test_unauthorized_caller() {
        let my_struct = MyStruct;
        my_struct.unauthorized_caller();
    }
}
