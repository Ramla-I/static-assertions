
use syn::{parse, LitInt, Token};

// I should aim for usage without struct,
// but have not found other solutions yet
pub struct SizeAlign {
    pub size: usize,
    pub align: usize,
}

impl parse::Parse for SizeAlign {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let content;
        let _ = syn::parenthesized!(content in input);

        let size_token: LitInt = content.parse()?;
        let _comma: Token![,] = content.parse()?;
        let align_token: LitInt = content.parse()?;

        let size = size_token.base10_parse::<usize>()?;
        let align = align_token.base10_parse::<usize>()?;

        Ok(SizeAlign { size, align })
    }
}
