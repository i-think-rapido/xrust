use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn validate<P, F, A>(parser: P, validate_fn: F)
    -> impl Fn(ParseInput)-> ParseResult<A>
    where
        P: Fn(ParseInput)-> ParseResult<A>,
        F: Fn(&A) -> bool,
{
    move |(input, index, config)| match parser((input, index, config)) {
        Ok((input2, next_index, config, result)) => {
            if validate_fn(&result) {
                Ok((input2, next_index,config, result))
            } else {
                Err(index)
            }
        }
        Err(err) => Err(err),
    }
}