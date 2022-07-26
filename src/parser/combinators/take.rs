use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn take_until<'a>(substr: &'static str)
    -> impl Fn(ParseInput)-> ParseResult<String> + '_
{
    move |(input, index, config)| match input[index..].find(substr){
        None => Err(index),
        Some(foundindex) => Ok((input.clone(), index + foundindex, config, input.chars().skip(index).take(foundindex).collect()))
    }
}

pub(crate) fn take_while<'a, F>(condition: F)
    -> impl Fn(ParseInput)-> ParseResult<String>
    where
        F: Fn(char) -> bool
{
    move |(input, index, config)| {

        match input.chars().skip(index).position(|c|!condition(c)){
            None => {Err(index)},
            Some(0) => {Err(index)},
            Some(pos) => {
                Ok((input.clone(), index + pos, config,  input.chars().skip(index).take(pos).collect()))
            }
        }
    }
}

pub(crate) fn take_while_m_n<'a, F>(min: usize, max: usize, condition: F)
    -> impl Fn(ParseInput)-> ParseResult<String>
    where
        F: Fn(char) -> bool
{
    move |(input, index, config)| {
        match input.chars().skip(index).position(|c|!condition(c)){
            None => {Err(index)},
            Some(pos) => {
                if pos >= min {
                    if pos > max {
                        Ok((input.clone(), index + max, config, input.chars().skip(index).take(max).collect()))
                    } else {
                        Ok((input.clone(), index + pos, config, input.chars().skip(index).take(pos).collect()))
                    }
                } else {
                    Err(index)
                }
            }
        }
    }
}
