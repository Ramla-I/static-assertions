#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

pub fn allowed_function() {}
pub fn disallowed_function() {}

pub struct MyStruct;

impl MyStruct {
    pub fn target_function(&self) {}
    pub fn target_function2(&self) {}

    #[calls("target_function")]
    pub fn allowed_caller(&self) {
        self.target_function();
    }

    #[calls("allowed_function", "target_function2")]
    pub fn allowed_caller_multiple(&self) {
        allowed_function();
        self.target_function2();
    }

    // ``` fails
    // #[calls("allowed_function", "target_function2")]
    // pub fn unauthorized_caller(&self) {
    //     self.target_function();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_function_calls() {
        #[allow(dead_code)]
        #[calls("allowed_function")]
        pub fn my_function() {
            allowed_function();
        }
    }

    // ``` fails
    // fn test_disallowed_function_calls() {
    //     #[allow(dead_code)]
    //     #[calls("allowed_function")]
    //     pub fn my_function() {
    //         disallowed_function();
    //     }
    // }
}

