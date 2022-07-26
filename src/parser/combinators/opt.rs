use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn opt<P1, R1>(parser1: P1)
                          -> impl Fn(ParseInput)-> ParseResult<Option<R1>>
    where
        P1: Fn(ParseInput)-> ParseResult<R1>
{
    move |(input, index, config)| match parser1((input.clone(), index, config.clone())){
        Ok((input1, index1, config, result1)) => Ok((input1, index1,config, Some(result1))),
        Err(_) => Ok((input, index, config, None))
    }
}
