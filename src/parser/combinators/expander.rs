use std::collections::HashMap;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::{ParseInput, ParseResult};
use crate::parser::xml::DTDDecl;

pub(crate) fn genentityexpander()
    -> impl Fn(ParseInput)-> ParseResult<String> +'static
{
    move |(input, index, config)| {
        let e = delimited(tag("&"), take_until(";"), tag(";"))((input, index, config));

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, newindex, mut config, entitykey)) => {
                match config.dtd.generalentities.get(&entitykey as &str) {
                    Some(DTDDecl::GeneralEntity(n, v)) => {
                        if config.currententitydepth >= config.maxentitydepth {
                            //attempting to exceed expansion depth
                            return Err(index)
                        } else{
                            input1.replace_range(index..newindex, v);
                            if index > config.entityindex {
                                config.currententitydepth = 0;
                                config.entityindex = index + v.chars().count();
                            } else {
                                config.currententitydepth += 1;
                            }
                            Ok((input1, index, config, "".to_string()))
                        }
                    }
                    _ => Err(index)
                }
            }
        }
    }
}



pub(crate) fn paramentityexpander()
    -> impl Fn(ParseInput)-> ParseResult<String> +'static
{
    move |(input, index, config)| {
        let e = delimited(tag("%"), take_until(";"), tag(";"))((input, index, config));

        match e {
            Err(usize) => Err(usize),
            Ok((mut input1, newindex, mut config, entitykey)) => {
                match config.dtd.paramentities.get(&entitykey as &str) {
                    Some(DTDDecl::ParamEntity(n, v)) => {
                        if config.currententitydepth >= config.maxentitydepth {
                            //attempting to exceed expansion depth
                            return Err(index)
                        } else{
                            input1.replace_range(index..newindex, v);
                            if index > config.entityindex {
                                config.currententitydepth = 0;
                                config.entityindex = index + v.chars().count();
                            } else {
                                config.currententitydepth += 1;
                            }
                            Ok((input1, index, config, "".to_string()))
                        }
                    }
                    _ => Err(index)
                }
            }
        }
    }
}
