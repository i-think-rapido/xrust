use crate::parser::combinators::{ParseInput, ParseResult};

pub(crate) fn tag<'a>(expected: &'static str)
    -> impl Fn(ParseInput) -> ParseResult<()> + '_
{
    move |(input, index, config)|
        match input.chars().skip(index).take(expected.chars().count()).collect::<String>(){
            c => {
                if c == expected {
                    Ok((input,index+expected.chars().count(), config, ()))
                } else {
                    Err(index)
                }
            }
        }
}


#[cfg(test)]
mod tests {
    use crate::parser::combinators::tag::tag;
    use crate::parser::ParserConfig;
    use crate::parser::xml::DTD;

    #[test]
    fn parser_tag_test1() {
        let testdoc = "<doc>".to_string();
        let parse_doc = tag("<");
        assert_eq!(
            Ok(("<doc>".to_string(), 1, ParserConfig::new(), ())),
            parse_doc((testdoc, 0, ParserConfig::new()))
        );
    }

    #[test]
    fn parser_tag_test2() {
        let testdoc = "<doc>".to_string();
        let parse_doc = tag(">");
        assert_eq!(
            Err(0),
            parse_doc((testdoc, 0, ParserConfig::new()))
        );
    }
}