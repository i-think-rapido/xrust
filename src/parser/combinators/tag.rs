use crate::parser::combinators::ParseResult;

pub(crate) fn tag<'a>(expected: &'static str)
    -> impl Fn(String, usize) -> ParseResult<()> + '_
{
    move |input, index|
        match input.chars().skip(index).take(expected.chars().count()).collect::<String>(){
            C => {
                if C == expected {
                    Ok((input,index+expected.chars().count(), ()))
                } else {
                    Err(index)
                }
            }
        }
}


#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;

    #[test]
    fn parser_tag_test1() {
        let testdoc = "<doc>".to_string();
        let parse_doc = tag("<");
        assert_eq!(
            Ok(("<doc>".to_string(), 1, ())),
            parse_doc(testdoc, 0)
        );
    }

    #[test]
    fn parser_tag_test2() {
        let testdoc = "<doc>".to_string();
        let parse_doc = tag(">");
        assert_eq!(
            Err(0),
            parse_doc(testdoc, 0)
        );
    }
}