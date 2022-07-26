use crate::parser::combinators::{ParseInput, ParseResult};


pub(crate) fn map<P, F, A, B>(parser: P, map_fn: F)
    -> impl Fn(ParseInput)-> ParseResult<B>
    //-> impl Fn(ParseInput)-> Result<(String, usize, B), usize>
    where
        P: Fn(ParseInput)-> ParseResult<A>,
        F: Fn(A) -> B,
{
    move |input|
        match parser(input) {
            Ok((input2, next_index,config, result)) =>{
                Ok((input2, next_index,config, map_fn(result)))
            },
            Err(err) => Err(err),
    }
}