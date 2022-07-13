//! # xdm::parsexml
//!
//! A parser for XML, as a parser combinator.
//! XML 1.1, see <https://www.w3.org/TR/xml11/>
//!

//extern crate nom;

use std::collections::HashSet;
use std::str::FromStr;
use crate::parser::common::{is_char, name, ncname};
use crate::qname::*;
//use crate::parsecommon::*;
use crate::xdmerror::*;

use crate::parser::combinators::ParseResult;
use crate::parser::combinators::alt::{alt2, alt3, alt4, alt6};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::many::many1;
use crate::parser::combinators::map::map;
use crate::parser::combinators::none_of::none_of;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_while, take_while_m_n};
use crate::parser::combinators::tuple::tuple2;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::combinators::tuple::tuple4;
use crate::parser::combinators::tuple::tuple5;
use crate::parser::combinators::tuple::tuple6;
use crate::parser::combinators::tuple::tuple10;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::combinators::validate::validate;
use crate::parser::combinators::value::value;
use crate::Value;


// nom doesn't pass additional parameters, only the input,
// so this is a two-pass process.
// First, use nom to tokenize and parse the input.
// Second, use the internal structure returned by the parser
// to build the document structure.

// This structure allows multiple root elements.
// An XML document will only be well-formed if there is exactly one element.
// However, external general entities may have more than one element.
#[derive(PartialEq)]
pub struct XMLDocument {
    pub prologue: Vec<XMLNode>,
    pub content: Vec<XMLNode>,
    pub epilogue: Vec<XMLNode>,
    pub xmldecl: Option<XMLdecl>
}

#[derive(Clone, PartialEq)]
pub enum XMLNode {
    Element(QualifiedName, Vec<XMLNode>, Vec<XMLNode>), // Element name, attributes, content
    Attribute(QualifiedName, Value),
    Text(Value),
    PI(String, Value),
    Comment(Value),	// Comment value is a string
    DTD(DTDDecl),	// These only occur in the prologue
    Reference(QualifiedName),	// General entity reference. These need to be resolved before presentation to the application
}

#[derive(PartialEq)]
pub struct XMLdecl {
    version: String,
    encoding: Option<String>,
    standalone: Option<String>
}

/// DTD declarations.
/// Only general entities are supported, so far.
/// TODO: element, attribute declarations
#[derive(Clone, PartialEq)]
pub enum DTDDecl {
    GeneralEntity(QualifiedName, String),
}


pub fn parse(e: String) -> Result<XMLDocument, Error> {
    match document(e) {
        Ok((_, I, X)) => {
                Result::Ok(X)
        },
        Err(u) => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("unrecoverable parser error at {}", u))})
    }
}


fn document(input: String) -> ParseResult<XMLDocument> {

    match tuple3(
        opt(prolog()),
        element(),
        opt(misc()),
    )(input, 0) {
        Ok((d, i, (p,c,e))) => {
            if d.chars().count() == i{
                Ok((d, i, XMLDocument {
                    prologue: vec![],
                    content: vec![c],
                    epilogue: vec![],
                    xmldecl: None
                }))
            } else {
                Err(i)
            }
        }
        Err(E) => Err(E)
    }
    /*
    map (
            tuple3(
            opt(prolog()),
            element(),
            opt(misc()),
        ),
        |(p, e, m)| {
            XMLDocument {
                prologue: vec![],
                content: vec![e],
                epilogue: vec![],
                xmldecl: None
            }
        }
    )(input,0)

     */
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog()
    -> impl Fn(String, usize) -> ParseResult<Vec<String>> {
    //-> ParseResult<(Option<XMLdecl>, Option<DocTypeDecl>)>
    map(
        tag("not yet implemented"),
        |_| {
            //vec![Node::new(NodeType::Comment).set_value("not yet implemented".to_string())]
            vec![]
        }
    )
    /*
    map(
        tuple2(
            opt(xmldecl),
            opt(doctypedecl)
        ),
        |(x, dtd)| (x, dtd)
    )

     */
}

fn xmldecl()
    -> impl Fn(String, usize) -> ParseResult<XMLdecl> {
    map(
        tuple10(
            tag("<?xml"),
            whitespace0(),
            map(
                tuple5(
                    tag("version"),
                    whitespace0(),
                    tag("="),
                    whitespace0(),
                    delimited_string
                ), | (_,_,_,_,v) | v
            ),
            whitespace0(),
            opt(
                map(
                    tuple5(
                        tag("encoding"),
                        whitespace0(),
                        tag("="),
                        whitespace0(),
                        delimited_string
                    ), | (_,_,_,_,e) | e
                )),
            whitespace0(),
            opt(
                map(
                    tuple5(
                        tag("standalone"),
                        whitespace0(),
                        tag("="),
                        whitespace0(),
                        delimited_string
                    ), | (_,_,_,_,s) | s
                )),
            whitespace0(),
            tag("?>"),
            whitespace0(),
        ),
        |(_,_, ver, _, enc,_,sta,_,_,_)| {
            XMLdecl{
                version: ver,
                encoding: enc,
                standalone: sta
            }
        }
    )
}

// Element ::= EmptyElemTag | STag content ETag
fn element()
    -> impl Fn(String, usize) -> ParseResult<XMLNode> {
    move |input, index|
    //map(
        alt2(
            emptyelem(),
            taggedelem(),
        )
        //,|e| {
            // TODO: Check for namespace declarations, and resolve URIs in the node tree under 'e'
//            e
//        }
    //)
    (input, index)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem()
    -> impl Fn(String, usize) -> ParseResult<XMLNode> {
        map(
            tuple5(
                tag("<"),
                qualname(),
                attributes(), //many0(attribute),
                whitespace0(),
                tag("/>"),
            ),
            |(_, n, a, _, _)| {
                XMLNode::Element(n, a, vec![])
            }
        )
}

// STag ::= '<' Name (Attribute)* '>'
// ETag ::= '</' Name '>'
// NB. Names must match
fn taggedelem()
    -> impl Fn(String, usize) -> ParseResult<XMLNode> {

    move |input, index| {
        map(
            tuple10(
                tag("<"),
                qualname(),
                attributes(), //many0(attribute),
                whitespace0(),
                tag(">"),
                content(),
                tag("</"),
                qualname(),
                whitespace0(),
                tag(">"),
            ),
            |(_, n, a, _, _, c, _, e, _, _)| {
                // TODO: check that the start tag name and end tag name match (n == e)
                XMLNode::Element(n, a, c)
            }
        )
            (input, index)}
}

// QualifiedName

fn qualname()
    -> impl Fn(String, usize)  -> ParseResult<QualifiedName> {
    alt2(
        prefixed_name(),
        unprefixed_name(),
    )
}
fn unprefixed_name()
    -> impl Fn(String, usize)  -> ParseResult<QualifiedName> {
    map (
        ncname(),
        |localpart| {
            QualifiedName::new(None, None, String::from(localpart))
        }
    )
}
fn prefixed_name()
    -> impl Fn(String, usize)  -> ParseResult<QualifiedName> {
    map (
        tuple3(
            ncname(),
            tag(":"),
            ncname()
        ),
        |(prefix, _, localpart)| {
            QualifiedName::new(None, Some(String::from(prefix)), String::from(localpart))
        }
    )
}

fn attributes()
    -> impl Fn(String, usize)  -> ParseResult<Vec<XMLNode>> {
    //this is just a wrapper around the attribute function, that checks for duplicates.
    validate(
        many0(
            attribute()
        ),
       |v: &Vec<XMLNode>|
            {
                let attrs = v.clone();
                let uniqueattrs: HashSet<_> = attrs.iter()
                    .map(
                        |xmlnode|
                            match xmlnode {
                                XMLNode::Attribute(q, _) => {q.to_string()}
                                _ => "".to_string()
                            }
                    )
                    .collect();
                if &v.len() == &uniqueattrs.len(){
                    true
                } else {
                    false
                }
            }
    )
}
// Attribute ::= Name '=' AttValue
fn attribute()
    -> impl Fn(String, usize)  -> ParseResult<XMLNode> {
    map(
        tuple6(
            whitespace1(),
            qualname(),
            whitespace0(),
            tag("="),
            whitespace0(),
            delimited_string(),
        ),
        |(_, n, _, _, _, s)| {
            XMLNode::Attribute(n, Value::String(s))
        }
    )
}
fn delimited_string()
    -> impl Fn(String, usize)  -> ParseResult<String> {
    alt2(
        string_single(),
        string_double(),
    )
}
fn string_single()
    -> impl Fn(String, usize)  -> ParseResult<String> {
    delimited(
        tag("\'"),
        map(
            many0(
                alt3(
                    chardata_escapes(),
                    chardata_unicode_codepoint(),
                    take_while(|c| !"&\'<".contains(c))
                )
            ),
            |v| v.concat()
        ),
        tag("\'")
    )
    /*
    delimited(
        tag("\'"),
        map(
            many0(none_of("'")),
            |v| v.concat()
        ),
        tag("\'"),
    )

     */
}
fn string_double()
    -> impl Fn(String, usize)  -> ParseResult<String> {
    delimited(
        tag("\""),
        map(
            many0(
                alt2(
                    chardata_escapes(),
                    take_while(|c| !"&\"<".contains(c))
                )
            ),
            |v| v.concat()
        ),
        tag("\"")
    )
    /*
    delimited(
        tag("\""),
        map(
            many0(none_of("\"")),
            |v| v.concat()
        ),
        tag("\""),
    )

     */
}



// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
fn content()
    -> impl Fn(String, usize) -> ParseResult<Vec<XMLNode>> {
    map(
        tuple2(
            opt(chardata()),
            many0(
                tuple2(
                    alt4(
                        element(),
                        reference(),
                        // TODO: CData Section
                        processing_instruction(),
                        comment(),
                    ),
                    opt(chardata()),
                )
            ),
        ),
        |(c, v)| {
            let mut new: Vec<XMLNode> = Vec::new();
            if c.is_some() {
                new.push(XMLNode::Text(Value::String(c.unwrap())));
            }
            if v.len() != 0 {
                for (w, d) in v {
                    new.push(w);
                    if d.is_some() {
                        new.push(XMLNode::Text(Value::String(d.unwrap())));
                    }
                }
            }
            new
        }
    )
}

// Reference ::= EntityRef | CharRef
// TODO
fn reference()
    -> impl Fn(String, usize)  -> ParseResult<XMLNode> {
    map(
        tag("not yet implemented"),
        |_| {
            XMLNode::Text(Value::String("not yet implemented".to_string()))
        }
    )
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction()
    -> impl Fn(String, usize)  -> ParseResult<XMLNode> {
    validate(
        map(
        delimited(
            tag("<?"),
            tuple4(
                whitespace0(),
                name(),
                whitespace0(),
                take_until("?>"),
            ),
            tag("?>"),
        ),
        |(_, n, _, v)| {
            XMLNode::PI(String::from(n), Value::String(v.to_string()))
        }
    ), | v| match v {
            XMLNode::PI(_, Value::String(v)) => {
                if v.contains(|c: char| !is_char(&c.to_string())){
                    false
                } else {
                    true
                }
            }
            _ => false
        }
    )
}


// Comment ::= '<!--' (char* - '--') '-->'
fn comment()
    -> impl Fn(String, usize)  -> ParseResult<XMLNode> {
    validate(
        map(
        delimited(
            tag("<!--"),
            take_until("--"),
            tag("-->"),
        ),
        |v: String| {
            XMLNode::Comment(Value::String(v))
        }
    ), | v| match v {
            XMLNode::Comment(Value::String(com)) => {
                if com.contains(|c: char| !is_char(&c.to_string())){
                    false
                } else {
                    true
                }
            }
            _ => false
        }
    )
}

// Misc ::= Comment | PI | S
fn misc()
    -> impl Fn(String, usize)  -> ParseResult<Vec<XMLNode>> {
        many0(
            alt2(
                comment(),
                processing_instruction()
            )
        )
}

// CharData ::= [^<&]* - (']]>')
fn chardata()
    -> impl Fn(String, usize)  -> ParseResult<String> {
    map(
        many1(
            alt3(
                chardata_cdata(),
                chardata_escapes(),
                chardata_literal()
            )
        ),
        |v| {
            v.concat()
        }
    )
}

fn chardata_cdata()
    -> impl Fn(String, usize)  -> ParseResult<String> {
        delimited(
            tag("<![CDATA["),take_until("]]>"),tag("]]>")
    )
}

fn chardata_escapes()
    -> impl Fn(String, usize)  -> ParseResult<String> {
    alt6(
        chardata_unicode_codepoint(),
        value(tag("&gt;"),">".to_string()),
        value(tag("&lt;"),"<".to_string()),
        value(tag("&amp;"),"&".to_string()),
        value(tag("&quot;"),"\"".to_string()),
        value(tag("&apos;"),"\'".to_string())
    )
}

fn chardata_unicode_codepoint()
    -> impl Fn(String, usize) -> ParseResult<String> {
    map(
        alt2(
            delimited(tag("&#x"),parse_hex(),tag(";")),
            delimited(tag("&#"),parse_decimal(),tag(";")),
        ),
        |value| std::char::from_u32(value).unwrap().to_string()
    )
}
fn parse_hex()
    -> impl Fn(String, usize) -> ParseResult<u32>{
    map (
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
        |hex| {
            u32::from_str_radix(&*hex, 16).unwrap()
        }
    )
}
fn parse_decimal()
    -> impl Fn(String, usize) -> ParseResult<u32>{
    map (
        take_while_m_n(1, 6, |c: char| c.is_ascii_digit()),
        |dec| {
            u32::from_str(&*dec).unwrap()
        }
    )
}


fn chardata_literal()
    -> impl Fn(String, usize)  -> ParseResult<String> {
    map(
        validate(many1(none_of("<&")),
               |v: &Vec<String>|
                   {
                       // chardata cannot contain ]]>
                       let mut w = v.clone();

                       let cd_end = &["]".to_string(),"]".to_string(),">".to_string()][..];

                       while !w.is_empty() {
                           if w.ends_with(cd_end) {return false;}
                           if !is_char(&w.last().unwrap()) {return false;}
                           w.pop();
                       }
                       true
                   }
        ),
        |c| c.concat()
    )
}