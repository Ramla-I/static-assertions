#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

pub struct MyStruct;

impl MyStruct {
    #[calledby("allowed_caller", "allowed_caller_multiple")]
    pub fn target_function(&self) -> &str {
        return "Hello World";
    }

    pub fn allowed_caller(&self) {
        Self::__target_function_callsite("allowed_caller");
        self.target_function();
    }

    pub fn allowed_caller_multiple(&self) {
        Self::__target_function_callsite("allowed_caller_multiple");
        self.target_function();
    }

    pub fn unauthorized_caller(&self) {
        Self::__target_function_callsite("unauthorized_caller");
        self.target_function();
    }
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
    #[should_panic(expected = "Unauthorized function trying to call target_function: unauthorized_caller")]
    fn test_unauthorized_caller() {
        let my_struct = MyStruct;
        my_struct.unauthorized_caller();
    }
}
