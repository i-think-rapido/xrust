use std::collections::HashMap;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn genentityexpander()
    -> impl Fn(ParseInput)-> ParseResult<String> +'static
{
    move |(input, index, config)| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))((input, index, config));

        let dtdgenentities = HashMap::from([
            //TODO remove these
            ("ENTITY", "ENTITYRESULT".to_string()),
            ("amp", "&".to_string()),
            ("lt", "<".to_string()),
            ("gt", ">".to_string()),
            ("quot", "\"".to_string()),
        ]);

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, newindex, config, entitykey)) => {
                match dtdgenentities.get(&entitykey as &str) {
                    None => Err(index),
                    Some(entityval) => {
                        input1.replace_range(index..newindex, entityval);
                        Ok((input1, index, config, "".parse().unwrap()))
                    }
                }
            }
        }
    }
}
