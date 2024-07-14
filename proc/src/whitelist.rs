use syn::{
    parse::{Parse, ParseStream},
    LitStr, Result, Token,
};

pub struct WhitelistArgs {
    pub functions: Vec<String>,
}

impl Parse for WhitelistArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut functions = Vec::new();

        while !input.is_empty() {
            let function_name: LitStr = input.parse()?;
            functions.push(function_name.value());

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(WhitelistArgs { functions })
    }
}
