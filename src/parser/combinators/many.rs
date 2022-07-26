use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn many0<P, R>(parser: P)
-> impl Fn(ParseInput)-> ParseResult<Vec<R>>
    where
        P: Fn(ParseInput)-> ParseResult<R>
{
    move |(mut input, mut index, mut config)| {
        let mut result = Vec::new();

        while let Ok((input2, next_index, next_config, next_item)) = parser((input.clone(), index, config.clone())) {
            index = next_index;
            result.push(next_item);
            input = input2;
            config = next_config;
        }

        Ok((input, index, config, result))
    }
}

pub(crate) fn many1<P, R>(parser: P)
-> impl Fn(ParseInput)-> ParseResult<Vec<R>>
    where
        P: Fn(ParseInput)-> ParseResult<R>
{
    move |(mut input, mut index, mut config)| {
        let mut result = Vec::new();

        match parser((input, index, config)){
            Err(err) => Err(err),
            Ok((input1, index1, config1, result1)) => {
                input = input1;
                index = index1;
                config = config1;
                result.push(result1);
                while let Ok((input2, index2, config2, next_item)) = parser((input.clone(), index, config.clone())) {
                    input = input2;
                    index = index2;
                    config = config2;
                    result.push(next_item);
                }
                Ok((input, index, config, result))
            }
        }
    }
}