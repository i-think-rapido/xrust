use crate::parser::combinators::ParseResult;

pub(crate) fn opt<P1, R1>(parser1: P1)
                          -> impl Fn(String, usize) -> ParseResult<Option<R1>>
    where
        P1: Fn(String, usize) -> ParseResult<R1>
{
    move |input, index| match parser1(input.clone(), index){
        Ok((input1, index1, result1)) => Ok((input1, index1, Some(result1))),
        Err(err) => Ok((input, index, None))
    }
}
