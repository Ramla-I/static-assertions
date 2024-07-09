/// Asserts that a function can consume one specified type with correct lifetime.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// fn consume_bool(_x: bool) {}
/// assert_function_consumes_one!(consume_bool: bool);
/// ```
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// fn consume_string(_x: String) {}
/// assert_function_consumes_one!(consume_string: String);
/// ```
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// struct Point {
///     x: f64,
///     y: f64,
/// }
/// fn consume_point(_x: Point) {}
/// assert_function_consumes_one!(consume_point: Point);
/// ```
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// fn consume_char(_x: char) {}
/// // This should fail to compile because consume_char expects char but is provided String.
/// assert_function_consumes_one!(consume_char: String);
/// ```
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// fn consume_option(_x: Option<i32>) {}
/// // This should fail to compile because consume_option expects Option<i32> but is provided i32.
/// assert_function_consumes_one!(consume_option: i32);
/// ```
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// fn consume_reference(_x: &i32) {}
/// // This should fail to compile because consume_reference expects &i32 but is provided i32.
/// assert_function_consumes_one!(consume_reference: i32);
/// ```
#[macro_export]
macro_rules! assert_function_consumes_one {
    // Ensure $f can consume each specified type $ti
    ($f:ident: $($ti:ty),*) => {
        #[allow(unused)]
        const _: () = {
            $(fn dummy(v: $ti) {
                let _ = $f(v);
            })*
        };
    };

    // Extends to test if $f can consume references to $ti types
    ($f:ident: & $($ti:ty),*) => {
        #[allow(unused)]
        const _: () = {
            $(fn dummy(v: &$ti) {
                let _ = $f(v);
            })*
        };
    };

    // Introduces a lifetime parameter $lt and tests $f with multiple $ti types
    ($f:ident: $lt:lifetime, $($ti:ty),*) => {
        #[allow(unused)]
        const _: () = {
            $(fn dummy<$lt>(v: $ti) {
                let _ = $f(v);
            })*
        };
    };

    // Combines lifetime $lt and reference $ti type checking for $f
    ($f:ident: $lt:lifetime, & $($ti:ty),*) => {
        #[allow(unused)]
        const _: () = {
            $(fn dummy<$lt>(v: &$ti) {
                let _ = $f(v);
            })*
        };
    };
}
