/// Asserts that the type has fields with the given types.

//https://stackoverflow.com/questions/64251852/is-there-a-way-to-check-a-struct-has-a-field-and-check-its-type
#[macro_export]
macro_rules! assert_fields_type {
    ($t:ty: $($i:ident: $ti:ty),+) => {
        #[allow(unknown_lints, unneeded_field_pattern)]
        const _: () = {
            fn dummy(v: $t) {
                $(let _: $ti = v.$i;)+
            }
        };
    };
}
