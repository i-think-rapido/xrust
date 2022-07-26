use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn delimited<P1, P2, P3, R1, R2, R3>(parser1: P1,
                                                parser2: P2,
                                                parser3: P3)
-> impl Fn(ParseInput)-> ParseResult<R2>
    where
        P1: Fn(ParseInput)-> ParseResult<R1>,
        P2: Fn(ParseInput)-> ParseResult<R2>,
        P3: Fn(ParseInput)-> ParseResult<R3>,
{
    move |input|
        match parser1(input) {
        Ok((input1, index1, config, _))
        => match parser2((input1, index1, config)) {
            Ok((input2, index2, config, result2))
            => match parser3((input2, index2, config)){
                Ok((input3, index3, config, _))=> Ok((input3, index3, config, result2)),
                Err(err) => Err(err)
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}