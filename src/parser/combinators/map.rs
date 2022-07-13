use std::any::type_name_of_val;
use crate::parser::combinators::ParseResult;


pub(crate) fn map<P, F, A, B>(parser: P, map_fn: F)
    -> impl Fn(String, usize) -> ParseResult<B>
    //-> impl Fn(String, usize) -> Result<(String, usize, B), usize>
    where
        P: Fn(String, usize) -> ParseResult<A>,
        F: Fn(A) -> B,
{
    move |input, index|
        match parser(input, index) {
            Ok((input2, next_index, result)) =>{
                Ok((input2, next_index, map_fn(result)))
            },
            Err(err) => Err(err),
    }
}