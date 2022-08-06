use crate::parser::xml::DTD;

pub mod xml;
pub(crate) mod combinators;
pub(crate) mod common;


#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParserConfig{
    dtd: DTD,
    /*
    The below will track Entity Expansion, ensuring that there are no recursive entities and
    some protections from zip bombs
     */
    maxentitydepth: usize,
    currententitydepth: usize,
    entityindex: usize
}

impl ParserConfig {
    pub fn new() -> ParserConfig {
        return ParserConfig{
            dtd: DTD::new(),
            maxentitydepth: 4,
            currententitydepth: 0,
            entityindex: 0,
        }
    }
}