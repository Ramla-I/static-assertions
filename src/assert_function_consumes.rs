/// function should always consume a type. Only works with function that takes one argument

#[macro_export]
macro_rules! assert_function_consumes_one {
    ($f:ident: $ti:ty) => {
        #[allow(unknown_lints, unneeded_field_pattern)]
        const _: () = {
            fn dummy(v: $ti) {
                let _ = $f(v);
            }
        };
    };
}
