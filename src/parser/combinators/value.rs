use crate::parser::combinators::ParseResult;

pub(crate) fn value<P1, R1, V: Clone>(parser1: P1, val: V)
    -> impl Fn(String, usize) -> ParseResult<V>
    where
        P1: Fn(String, usize) -> ParseResult<R1>,
{
    move |input, index| match parser1(input, index) {
        Ok((input1, index1, result)) => Ok((input1,index1, val.clone())),
        Err(err) => Err(err),
    }
}