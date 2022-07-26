use crate::parser::xml::DTD;

pub mod xml;
pub(crate) mod combinators;
pub(crate) mod common;


#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParserConfig{
    dtd :DTD
}

impl ParserConfig {
    pub fn new() -> ParserConfig {
        return ParserConfig{
            dtd: DTD::new()
        }
    }
}