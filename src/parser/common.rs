use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::{ParseInput, ParseResult};
use crate::parser::combinators::take::{take_while, take_while_m_n};
use crate::parser::combinators::tuple::tuple2;

// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub(crate) fn ncname<'a>()
    -> impl Fn(ParseInput)-> ParseResult<String> + 'a
{
    //move |input, index|
        map(
        tuple2(
            take_while_m_n(1, 1, is_ncnamestartchar),
            opt(take_while(is_ncnamechar)),
        ), |(a, b)| {
                [a, b.unwrap_or("".to_string())].concat()
            }
    )
            //(input, index)
}

pub(crate) fn name()
    -> impl Fn(ParseInput)-> ParseResult<String> {
    //move |input, index|
        map(
        tuple2(
            take_while_m_n(1, 1, is_namestartchar),
            take_while(is_namechar),
        ), |(a, b)| [a, b].concat()
    )
            //(input, index)
}

pub(crate) fn is_namechar(ch: char) -> bool {
    if is_namestartchar(ch) {
        true
    } else {
        match ch {
            '.' => true,
            '-' => true,
            '0'..='9' => true,
            '\u{B7}' => true,
            '\u{0300}'..='\u{036F}' => true,
            '\u{203F}'..='\u{2040}' => true,
            _ => false
        }
    }
}

fn is_ncnamechar(ch: char) -> bool {
    if is_ncnamestartchar(ch) {
        true
    } else {
        match ch {
            '.' |
            '-' |
            '0'..='9' |
            '\u{B7}' |
            '\u{0300}'..='\u{036F}' |
            '\u{203F}'..='\u{2040}' => {
                true
            },
            _ => false
        }
    }
}

fn is_namestartchar(ch: char) -> bool {
    match ch {
        ':' => true,
        _ => is_ncnamestartchar(ch)
    }
}
fn is_ncnamestartchar(ch: char) -> bool {
    match ch {
        '\u{0041}'..='\u{005A}' // A-Z
        | '\u{005F}' // _
        | '\u{0061}'..='\u{007A}' // a-z
        | '\u{00C0}'..='\u{00D6}' //  [#xC0-#xD6]
        | '\u{00D8}'..='\u{00F6}' //  [#xD8-#xF6]
        | '\u{00F8}'..='\u{02FF}' //  [#xF8-#x2FF]
        | '\u{0370}'..='\u{037D}' //  [#x370-#x37D]
        | '\u{037F}'..='\u{1FFF}' //  [#x37F-#x1FFF]
        | '\u{200C}'..='\u{200D}' //  [#x200C-#x200D]
        | '\u{2070}'..='\u{218F}' //  [#x2070-#x218F]
        | '\u{2C00}'..='\u{2FEF}' //  [#x2C00-#x2FEF]
        | '\u{3001}'..='\u{D7FF}' //  [#x3001-#xD7FF]
        | '\u{F900}'..='\u{FDCF}' //  [#xF900-#xFDCF]
        | '\u{FDF0}'..='\u{FFFD}' //  [#xFDF0-#xFFFD]
        | '\u{10000}'..='\u{EFFFF}' //  [#x10000-#xEFFFF]
        => {
            true
        },
        // etc
        _ => false
    }
}

pub fn is_char(ch: &String) -> bool {
    match ch.chars().next().unwrap() {
        '\u{0009}' // #x9
        | '\u{000A}' // #xA
        | '\u{000D}' // #xD
        | '\u{0020}'..='\u{D7FF}' //  [#x0020-#xD7FF]
        | '\u{E000}'..='\u{FFFD}' //  [#xE000-#xFFFD]
        | '\u{10000}'..='\u{10FFFF}' //  [#x10000-#10FFFF]
        => {
            true
        },
        // etc
        _ => false
    }
}