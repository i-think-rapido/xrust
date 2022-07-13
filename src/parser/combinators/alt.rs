use crate::parser::combinators::ParseResult;

pub(crate) fn alt2<P1, P2, A>(parser1: P1, parser2: P2)
                              -> impl Fn(String, usize) -> ParseResult<A>
    where
        P1: Fn(String, usize) -> ParseResult<A>,
        P2: Fn(String, usize) -> ParseResult<A>,
{
    move |input, index| match parser1(input.clone(), index) {
        Ok((input1, index1, result1)) => Ok((input1, index1, result1)),
        Err(err) => match parser2(input, index){
            Ok((input2, index2, result2)) => Ok((input2, index2, result2)),
            Err(err) => Err(err)
        }
    }
}

pub(crate) fn alt3<P1, P2, P3, A>(parser1: P1,
                                  parser2: P2,
                                  parser3: P3)
    -> impl Fn(String, usize) -> ParseResult<A>
    where
        P1: Fn(String, usize) -> ParseResult<A>,
        P2: Fn(String, usize) -> ParseResult<A>,
        P3: Fn(String, usize) -> ParseResult<A>,
{
    move |input, index| match parser1(input.clone(), index) {
        Ok((input1, index1, result1)) => Ok((input1, index1, result1)),
        Err(err) => match parser2(input.clone(), index){
            Ok((input2, index2, result2)) => Ok((input2, index2, result2)),
            Err(err) => match parser3(input, index){
                Ok((input3, index3, result3)) => Ok((input3, index3, result3)),
                Err(err) => Err(err)
            }
        }
    }
}

pub(crate) fn alt4<P1, P2, P3, P4, A>(parser1: P1,
                                      parser2: P2,
                                      parser3: P3,
                                      parser4: P4)
                              -> impl Fn(String, usize) -> ParseResult<A>
    where
        P1: Fn(String, usize) -> ParseResult<A>,
        P2: Fn(String, usize) -> ParseResult<A>,
        P3: Fn(String, usize) -> ParseResult<A>,
        P4: Fn(String, usize) -> ParseResult<A>,
{
    move |input, index| match parser1(input.clone(), index) {
        Ok((input1, index1, result1)) => Ok((input1, index1, result1)),
        Err(err) => match parser2(input.clone(), index){
            Ok((input2, index2, result2)) => Ok((input2, index2, result2)),
            Err(err) => match parser3(input.clone(), index){
                Ok((input3, index3, result3)) => Ok((input3, index3, result3)),
                Err(err) => match parser4(input, index){
                    Ok((input4, index4, result4)) => Ok((input4, index4, result4)),
                    Err(err) => Err(err)
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
    -> impl Fn(String, usize) -> ParseResult<A>
    where
        P1: Fn(String, usize) -> ParseResult<A>,
        P2: Fn(String, usize) -> ParseResult<A>,
        P3: Fn(String, usize) -> ParseResult<A>,
        P4: Fn(String, usize) -> ParseResult<A>,
        P5: Fn(String, usize) -> ParseResult<A>,
        P6: Fn(String, usize) -> ParseResult<A>,
{
    move |input, index| match parser1(input.clone(), index) {
        Ok((input1, index1, result1)) => Ok((input1, index1, result1)),
        Err(err) => match parser2(input.clone(), index){
            Ok((input2, index2, result2)) => Ok((input2, index2, result2)),
            Err(err) => match parser3(input.clone(), index){
                Ok((input3, index3, result3)) => Ok((input3, index3, result3)),
                Err(err) => match parser4(input.clone(), index){
                    Ok((input4, index4, result4)) => Ok((input4, index4, result4)),
                    Err(err) => match parser5(input.clone(), index) {
                        Ok((input5, index5, result5)) => Ok((input5, index5, result5)),
                        Err(err) => match parser6(input, index) {
                            Ok((input6, index6, result6)) => Ok((input6, index6, result6)),
                            Err(err) => Err(err)
                        }
                    }
                }
            }
        }
    }
}

