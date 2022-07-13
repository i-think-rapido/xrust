use crate::parser::combinators::ParseResult;

pub(crate) fn delimited<P1, P2, P3, R1, R2, R3>(parser1: P1, parser2: P2, parser3: P3)
                                                -> impl Fn(String, usize) -> ParseResult<R2>
    where
        P1: Fn(String, usize) -> ParseResult<R1>,
        P2: Fn(String, usize) -> ParseResult<R2>,
        P3: Fn(String, usize) -> ParseResult<R3>,
{
    move |input, index|
        match parser1(input, index) {
        Ok((input1, index1, result1))
        => match parser2(input1, index1) {
            Ok((input2, index2, result2))
            => match parser3(input2, index2){
                Ok((input3, index3, result3))=> Ok((input3, index3, result2)),
                Err(err) => Err(err)
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}