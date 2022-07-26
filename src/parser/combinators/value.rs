use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn value<P1, R1, V: Clone>(parser1: P1, val: V)
    -> impl Fn(ParseInput)-> ParseResult<V>
    where
        P1: Fn(ParseInput)-> ParseResult<R1>,
{
    move |input| match parser1(input) {
        Ok((input1, index1, config, _)) => Ok((input1,index1, config, val.clone())),
        Err(err) => Err(err),
    }
}