use std::collections::HashMap;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::ParseResult;

pub fn entityexpander()
    -> impl Fn(String, usize) -> ParseResult<String> +'static
{
    move |input,index| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))(input.clone(), index);

        let dtdgenentities = HashMap::from([
        ]);

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, newindex, entitykey)) => {
                match dtdgenentities.get(&entitykey as &str) {
                    None => Err(index),
                    Some(entityval) => {
                        input1.replace_range(index..newindex, entityval);
                        Ok((input1, index, "".parse().unwrap()))
                    },
                    _ => Err(index),
                }
            }
        }
    }
}
