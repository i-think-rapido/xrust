use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn none_of<'a>(chars: &str)
    -> impl Fn(ParseInput)-> ParseResult<String> + '_
{
    move |(input, index, config)|
        match input.chars().skip(index).next(){
        //match input.clone().get(index..index+1) {
        Some(next) => {
            if chars.contains(next) {
                Err(index)
            }else {
                Ok((input,index+1,config, next.to_string()))
            }
        },
        _ => {
            Err(index)
        }
    }
}