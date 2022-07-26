use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn alt2<P1, P2, A>(parser1: P1, parser2: P2)
                              -> impl Fn(ParseInput) -> ParseResult<A>
    where
        P1: Fn(ParseInput)-> ParseResult<A>,
        P2: Fn(ParseInput)-> ParseResult<A>,
{
    move |input| match parser1(input.clone()) {
        Ok(parse_result) => Ok(parse_result),
        Err(_) => match parser2(input){
            Ok(parse_result2) => Ok(parse_result2),
            Err(err) => Err(err)
        }
    }
}

pub(crate) fn alt3<P1, P2, P3, A>(parser1: P1,
                                  parser2: P2,
                                  parser3: P3)
    -> impl Fn(ParseInput)-> ParseResult<A>
    where
        P1: Fn(ParseInput)-> ParseResult<A>,
        P2: Fn(ParseInput)-> ParseResult<A>,
        P3: Fn(ParseInput)-> ParseResult<A>,
{
    move |input| match parser1(input.clone()) {
        Ok(parse_result) => Ok(parse_result),
        Err(_) => match parser2(input.clone()){
            Ok(parse_result2) => Ok(parse_result2),
            Err(_) => match parser3(input){
                Ok(parse_result3) => Ok(parse_result3),
                Err(err) => Err(err)
            }
        }
    }
}

pub(crate) fn alt4<P1, P2, P3, P4, A>(parser1: P1,
                                      parser2: P2,
                                      parser3: P3,
                                      parser4: P4)
    -> impl Fn(ParseInput)-> ParseResult<A>
    where
        P1: Fn(ParseInput)-> ParseResult<A>,
        P2: Fn(ParseInput)-> ParseResult<A>,
        P3: Fn(ParseInput)-> ParseResult<A>,
        P4: Fn(ParseInput)-> ParseResult<A>,
{
    move |input| match parser1(input.clone()) {
        Ok(parse_result) => Ok(parse_result),
        Err(_) => match parser2(input.clone()){
            Ok(parse_result2) => Ok(parse_result2),
            Err(_) => match parser3(input.clone()){
                Ok(parse_result3) => Ok(parse_result3),
                Err(_) => match parser4(input){
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(err) => Err(err)
                }
            }
        }
    }
}


pub(crate) fn alt5<P1, P2, P3, P4, P5, A>(parser1: P1,
                                          parser2: P2,
                                          parser3: P3,
                                          parser4: P4,
                                          parser5: P5)
-> impl Fn(ParseInput)-> ParseResult<A>
    where
        P1: Fn(ParseInput)-> ParseResult<A>,
        P2: Fn(ParseInput)-> ParseResult<A>,
        P3: Fn(ParseInput)-> ParseResult<A>,
        P4: Fn(ParseInput)-> ParseResult<A>,
        P5: Fn(ParseInput)-> ParseResult<A>
{
    move |input| match parser1(input.clone()) {
        Ok(parse_result) => Ok(parse_result),
        Err(_) => match parser2(input.clone()){
            Ok(parse_result2) => Ok(parse_result2),
            Err(_) => match parser3(input.clone()){
                Ok(parse_result3) => Ok(parse_result3),
                Err(_) => match parser4(input.clone()){
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(_) => match parser5(input) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(err) => Err(err)
                    }
                }
            }
        }
    }
}

pub(crate) fn alt6<P1, P2, P3, P4, P5, P6, A>(parser1: P1,
                                      parser2: P2,
                                      parser3: P3,
                                      parser4: P4,
                                      parser5: P5,
                                      parser6: P6)
    -> impl Fn(ParseInput)-> ParseResult<A>
    where
        P1: Fn(ParseInput)-> ParseResult<A>,
        P2: Fn(ParseInput)-> ParseResult<A>,
        P3: Fn(ParseInput)-> ParseResult<A>,
        P4: Fn(ParseInput)-> ParseResult<A>,
        P5: Fn(ParseInput)-> ParseResult<A>,
        P6: Fn(ParseInput)-> ParseResult<A>,
{
    move |input| match parser1(input.clone()) {
        Ok(parse_result) => Ok(parse_result),
        Err(_) => match parser2(input.clone()){
            Ok(parse_result2) => Ok(parse_result2),
            Err(_) => match parser3(input.clone()){
                Ok(parse_result3) => Ok(parse_result3),
                Err(_) => match parser4(input.clone()){
                    Ok(parse_result4) => Ok(parse_result4),
                    Err(_) => match parser5(input.clone()) {
                        Ok(parse_result5) => Ok(parse_result5),
                        Err(_) => match parser6(input) {
                            Ok(parse_result6) => Ok(parse_result6),
                            Err(err) => Err(err)
                        }
                    }
                }
            }
        }
    }
}

