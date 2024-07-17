use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token, Type,
    Result
};

pub struct WhitelistArgs {
    pub fn_name: Ident,
    pub arg_types: Vec<Type>,
}

// Implement parsing for the macro invocation structure
impl Parse for WhitelistArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fn_name: Ident = input.parse()?;
        let _colon_token: Token![:] = input.parse()?;
        let arg_types: Punctuated<Type, Token![,]> = input.parse_terminated(Type::parse, Token![,])?;
        let arg_types: Vec<Type> = arg_types.into_iter().collect();

        Ok(WhitelistArgs { fn_name, arg_types })
    }
}
