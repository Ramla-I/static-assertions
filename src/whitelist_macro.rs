/// This macro generates an `is_allowed` method for a struct to check if a field can be mutated
/// only within a whitelist of specified functions.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {
/// #[derive(Debug)]
/// struct MyStruct {
///     field: i32,
/// }
///
/// whitelist!(MyStruct, field, allowed_function);
///
/// impl MyStruct {
///     pub fn allowed_function(&mut self) {
///         if Self::is_allowed("allowed_function") {
///             self.field = 42;
///         }
///     }
///
///     pub fn disallowed_function(&mut self) {
///         if Self::is_allowed("disallowed_function") {
///             self.field = 99; // This will not happen as disallowed_function is not in the whitelist
///         }
///     }
/// }
///
/// let mut instance = MyStruct { field: 0 };
/// instance.allowed_function();
/// assert_eq!(instance.field, 42);
///
/// instance.disallowed_function();
/// assert_eq!(instance.field, 42); // The field remains unchanged
/// # }
/// ```
#[macro_export]
macro_rules! whitelist {
    ($struct_type:ty, $field:ident, $($fn_name:ident),*) => {
        impl $struct_type {
            fn is_allowed(caller: &str) -> bool {
                match caller {
                    $(stringify!($fn_name) => true,)*
                    _ => false,
                }
            }
        }
    };
}
