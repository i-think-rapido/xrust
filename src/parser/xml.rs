//! # xdm::parsexml
//!
//! A parser for XML, as a parser combinator.
//! XML 1.1, see <https://www.w3.org/TR/xml11/>
//!

//extern crate nom;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::parser::common::{is_char, is_namechar, name, ncname};
use crate::qname::*;
//use crate::parsecommon::*;
use crate::xdmerror::*;

use crate::parser::combinators::{ParseInput, ParseResult};
use crate::parser::combinators::alt::{alt2, alt3, alt4, alt5, alt6, alt7};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::expander::{genentityexpander, paramentityexpander};
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
use crate::parser::combinators::tuple::tuple7;
use crate::parser::combinators::tuple::tuple8;
use crate::parser::combinators::tuple::tuple9;
use crate::parser::combinators::tuple::tuple10;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::combinators::validate::validate;
use crate::parser::combinators::value::value;
use crate::parser::ParserConfig;
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
#[derive(Clone, Debug, PartialEq)]
pub enum DTDDecl {
    Element(QualifiedName, String),
    Attlist(QualifiedName, String),
    Notation(QualifiedName, String),
    GeneralEntity(QualifiedName, String),
    ParamEntity(QualifiedName, String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct DTD {
    elements: HashMap<String, DTDDecl>,
    attlists: HashMap<String, DTDDecl>,
    notations: HashMap<String, DTDDecl>,
    pub(crate) generalentities: HashMap<String, DTDDecl>,
    pub(crate) paramentities: HashMap<String, DTDDecl>,
    publicid: Option<String>,
    systemid: Option<String>,
    name: Option<String>
}

impl DTD {
    pub fn new() -> DTD {
        return DTD {
            elements: Default::default(),
            attlists: Default::default(),
            notations: Default::default(),
            generalentities: Default::default(),
            paramentities: Default::default(),
            publicid: None,
            systemid: None,
            name: None
        }
    }
}


pub fn parse(e: String) -> Result<XMLDocument, Error> {
    match document(e) {
        Ok((_,_,_, XMLDoc)) => {
                Result::Ok(XMLDoc)
        },
        Err(u) => Result::Err(Error{kind: ErrorKind::Unknown, message: String::from(format!("unrecoverable parser error at {}", u))})
    }
}


fn document(input: String) -> ParseResult<XMLDocument> {
    //TODO ADD CONFIG AND DTD
    match tuple3(
        prolog(),
        element(),
        opt(misc()),
    )((input, 0, ParserConfig::new())) {
        Ok((d, i,pc, ((xmld, pr),c,e))) => {
            if d.chars().count() == i{
                Ok((d, i, pc, XMLDocument {
                    prologue: pr,
                    content: vec![c],
                    epilogue: vec![],
                    xmldecl: xmld
                }))
            } else {
                Err(i)
            }
        }
        Err(err) => Err(err)
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
    -> impl Fn(ParseInput)-> ParseResult<(Option<XMLdecl>, Vec<XMLNode>)> {
    map(
    tuple4(
        opt(xmldecl()),
        misc(),
        opt(doctypedecl()),
        misc()
    ),|(xmld, mut m1,  dtd, mut m2)| {
            m1.append(&mut m2);
            (xmld, m1)
        }
    )
}

fn xmldecl()
    -> impl Fn(ParseInput)-> ParseResult<XMLdecl> {
    map(
        tuple10(
            tag("<?xml"),
            whitespace1(),
            map(
                tuple5(
                    tag("version"),
                    whitespace0(),
                    tag("="),
                    whitespace0(),
                    delimited_string()
                ), | (_,_,_,_,v) | v
            ),
            whitespace1(),
            opt(
                map(
                    tuple5(
                        tag("encoding"),
                        whitespace0(),
                        tag("="),
                        whitespace0(),
                        delimited_string()
                    ), | (_,_,_,_,e) | e
                )),
            whitespace1(),
            opt(
                map(
                    tuple5(
                        tag("standalone"),
                        whitespace0(),
                        tag("="),
                        whitespace0(),
                        delimited_string()
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

fn doctypedecl()
    -> impl Fn(ParseInput) -> ParseResult<()>{
    move |input|
        match tuple8(
        tag("<!DOCTYPE"),
        whitespace1(),
        name(),
        whitespace1(),
        opt(externalid()),
        whitespace0(),
        opt(
            delimited(
                tag("["),
                intsubset(),
                tag("]")
            )),
        tag(">")
    )(input) {
            Ok((d,i, c,(_,_,n,_,e,_,inss,_))) => {
                Ok((d,i,c,()))
            }
            Err(err) => Err(err)
        }
}

fn externalid()
    -> impl Fn(ParseInput) -> ParseResult<(String, Option<String>)> {
    alt2(
        map(
            tuple3(
                tag("SYSTEM"),
                whitespace0(),
                alt2(
                    delimited(tag("'"),take_until("'"),tag("'")),
                    delimited(tag("\""),take_until("\""),tag("\""))
                ) //SystemLiteral
            ),
            |(_,_,sid)| (sid, None)
        ),
        map(
            tuple5(
                tag("PUBLIC"),
                whitespace0(),
                alt2(
                    delimited(tag("'"),take_until("'"),tag("'")),
                    delimited(tag("\""),take_until("\""),tag("\""))
                ), //PubidLiteral TODO validate chars here (PubidChar from spec).
                whitespace1(),
                alt2(
                    delimited(tag("'"),take_until("'"),tag("'")),
                    delimited(tag("\""),take_until("\""),tag("\""))
                ) //SystemLiteral
            ),
            |(_,_,pid,_,sid)| (sid, Some(pid))
        )
    )
}


fn intsubset()
    -> impl Fn(ParseInput) -> ParseResult<Vec<()>> {
    many0(
        alt6(
            elementdecl(),
            attlistdecl(),
            pedecl(),
            gedecl(),
            ndatadecl(),
            whitespace1()
        )
    )
}

//elementdecl	   ::=   	'<!ELEMENT' S Name S contentspec S? '>'
fn elementdecl()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    move |input|
        match tuple7(
                            tag("<!ELEMENT"),
                            whitespace1(),
                            qualname(),
                            whitespace1(),
                            contentspec(), //contentspec - TODO Build out.
                            whitespace0(),
                            tag(">")
                            )(input) {
            Ok((d,i, mut c,(_,_,n,_,s,_,_))) => {
                c.dtd.elements.insert(n.to_string(), DTDDecl::Element(n, s));
                Ok((d, i, c, ()))
            }
            Err(err) => Err(err)
        }
}
fn contentspec()
    -> impl Fn(ParseInput) -> ParseResult<String>
{
    alt4(
        value(tag("EMPTY"),"EMPTY".to_string()),
        value(tag("ANY"),"ANY".to_string()),
        mixed(),
        children()
    )
}

//AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
fn attlistdecl()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    move |input|
        match tuple6(
            tag("<!ATTLIST"),
            whitespace1(),
            qualname(),
            many0(
             attdef()
            ),
            whitespace0(),
            tag(">")
        )(input) {
            Ok((d,i, mut c,(_,_,n,_,_,_))) => {
                c.dtd.attlists.insert(n.to_string(), DTDDecl::Attlist(n, "".to_string()));
                Ok((d, i, c, ()))
            }
            Err(err) => Err(err)
        }
}

//AttDef ::= S Name S AttType S DefaultDecl
fn attdef()
    -> impl Fn(ParseInput) -> ParseResult<String>
{
    map(
        tuple6(
            whitespace1(),
            name(),
            whitespace1(),
            atttype(),
            whitespace1(),
            defaultdecl()
        ),|x|{"".to_string()}
    )
}

//AttType ::= StringType | TokenizedType | EnumeratedType
fn atttype()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    alt3(
        tag("CDATA"), //Stringtype
        alt7( //tokenizedtype
             tag("ID"),
             tag("IDREF"),
             tag("IDREFS"),
             tag("ENTITY"),
             tag("ENTITIES"),
             tag("NMTOKEN"),
             tag("NMTOKENS")
        ),
        enumeratedtype()
    )
}

//EnumeratedType ::= NotationType | Enumeration
fn enumeratedtype()
    -> impl Fn(ParseInput) -> ParseResult<()>{
    alt2(
        notationtype(),
        enumeration()
    )
}

//NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
fn notationtype()
    -> impl Fn(ParseInput) -> ParseResult<()>{
    map(
        tuple8(
            tag("NOTATION"),
            whitespace1(),
            tag("("),
            whitespace0(),
            name(),
            many0(
                tuple4(
                    whitespace0(),
                    tag("|"),
                    whitespace0(),
                    name()
                )
            ),
            whitespace0(),
            tag(")")
        )
        ,|x| {()}
    )
}

//Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
fn enumeration()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    map(
        tuple6(
            tag("("),
            whitespace0(),
            nmtoken(),
            many0(
                tuple4(
                    whitespace0(),
                    tag("|"),
                    whitespace0(),
                    nmtoken()
                )
            ),
            whitespace0(),
            tag(")")
        ),|x|{()}
    )
}

fn nmtoken()
-> impl Fn(ParseInput) -> ParseResult<()>
{
    map(
        many1(
            take_while(|c| is_namechar(c))
        ),|x|{()}
    )
}

//DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
fn defaultdecl()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    map(
        alt3(
            value(tag("#REQUIRED"),"#REQUIRED".to_string()),
            value(tag("#IMPLIED"), "#IMPLIED".to_string()),
            map(
                tuple2(
                    opt(
                        tuple2(
                            value(tag("#FIXED"),"#FIXED".to_string()),
                            whitespace1()
                        )
                    ),
                    attvalue()
                ),|(x,y)|{
                    match x {
                        None => {y}
                        Some((mut f, _)) => {
                           f.push_str(&*y);
                           f
                        }
                    }
                }
            )
        ),
        |x| { () }
    )
}

//AttValue ::= '"' ([^<&"] | Reference)* '"' | "'" ([^<&'] | Reference)* "'"
fn attvalue()
    -> impl Fn(ParseInput) -> ParseResult<String>
{
    alt2(
        delimited(
            tag("\'"),
            map(
                many0(
                    alt3(
                        take_while(|c| !"&\'<".contains(c)),
                        genentityexpander(),
                        paramentityexpander()
                    )
                ),|v|{
                    v.join("")
                }
            ),
            tag("\'")
        ),
        delimited(
            tag("\""),
            map(
                many0(
                    alt3(
                        take_while(|c| !"&\"<".contains(c)),
                        genentityexpander(),
                        paramentityexpander()
                    )
                ),|v|{
                    v.join("")
                }
            ),
            tag("\"")
        )
    )
}

fn pedecl()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    move |input|
        match tuple9(
            tag("<!ENTITY"),
            whitespace1(),
            tag("%"),
            whitespace1(),
            qualname(),
            whitespace1(),
            alt2(
                delimited(tag("'"),take_until("'"),tag("'")),
                delimited(tag("\""),take_until("\""),tag("\""))
            ),
            whitespace0(),
            tag(">")
        )(input) {
            Ok((d,i, mut c,(_,_,_,_,n,_,s,_,_))) => {
                c.dtd.paramentities.insert(n.to_string(), DTDDecl::ParamEntity(n, s));
                Ok((d, i, c, ()))
            }
            Err(err) => Err(err)
        }
}

fn gedecl()
    -> impl Fn(ParseInput) -> ParseResult<()>
{
    move |input|
        match tuple7(
            tag("<!ENTITY"),
            whitespace1(),
            qualname(),
            whitespace1(),
            alt2(
                delimited(tag("'"),take_until("'"),tag("'")),
                delimited(tag("\""),take_until("\""),tag("\""))
            ),
            whitespace0(),
            tag(">")
        )(input) {
            Ok((d,i, mut c,(_,_,n,_,s,_,_))) => {
                c.dtd.generalentities.insert(n.to_string(), DTDDecl::GeneralEntity(n, s));
                Ok((d, i, c, ()))
            }
            Err(err) => Err(err)
        }
}
fn ndatadecl()
    -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input|
        match tuple7(
            tag("<!NOTATION"),
            whitespace1(),
            qualname(),
            whitespace1(),
            take_until(">"), //contentspec - TODO Build out.
            whitespace0(),
            tag(">")
        )(input) {
            Ok((d,i, mut c,(_,_,n,_,s,_,_))) => {
                c.dtd.notations.insert(n.to_string(), DTDDecl::Notation(n, s));
                Ok((d, i, c, ()))
            }
            Err(err) => Err(err)
        }
}

//Mixed	   ::=   	'(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
fn mixed()
    -> impl Fn(ParseInput) -> ParseResult<String>
{
    alt2(
        map(
            tuple6(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                many0(
                    tuple4(
                        whitespace0(),
                        tag("|"),
                        whitespace0(),
                        name()
                    )
                ),
                whitespace0(),
                tag(")*")
            ), |x| {"".to_string()}
        ),
        map(
            tuple5(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                whitespace0(),
                tag(")")
            ), |x| {"".to_string()}
        )
    )
}

// children	   ::=   	(choice | seq) ('?' | '*' | '+')?
fn children()
    -> impl Fn(ParseInput)-> ParseResult<String>
{
    map(
        tuple2(
            alt2(
                choice(),
                seq()
            ),
            opt(
                alt3(
                    tag("?"),
                    tag("*"),
                    tag("+"),
                )
            )
        ),
        |x|{ "".to_string() }
    )
}

// cp	   ::=   	(Name | choice | seq) ('?' | '*' | '+')?
fn cp()
    -> impl Fn(ParseInput)-> ParseResult<String>
{
    move |input|
    map(
        tuple2(
            alt3(
                name(),
                choice(),
                seq()
            ),
            opt(
                alt3(
                    tag("?"),
                    tag("*"),
                    tag("+"),
                )
            )
        ),
        |x| { "".to_string() }
    )
    (input)
}
//choice	   ::=   	'(' S? cp ( S? '|' S? cp )+ S? ')'
fn choice()
    -> impl Fn(ParseInput)-> ParseResult<String>
{
    move |input|
    map(
        tuple6(
            tag("("),
            whitespace0(),
            cp(),
            many0(
                tuple4(
                    whitespace0(),
                    tag("|"),
                    whitespace0(),
                    cp()
                )
            ),
            whitespace0(),
            tag(")")
        ),|x| {"".to_string()}
    )
    (input)
}

//seq	   ::=   	'(' S? cp ( S? ',' S? cp )* S? ')'
fn seq()
    -> impl Fn(ParseInput)-> ParseResult<String>
{
    map(
        tuple6(
            tag("("),
            whitespace0(),
            cp(),
            many0(
                tuple4(
                    whitespace0(),
                    tag(","),
                    whitespace0(),
                    cp()
                )
            ),
            whitespace0(),
            tag(")")
        ),|x|{"".to_string()}
    )
}

// Element ::= EmptyElemTag | STag content ETag
fn element()
    -> impl Fn(ParseInput)-> ParseResult<XMLNode> {
    move |input|
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
    (input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem()
    -> impl Fn(ParseInput)-> ParseResult<XMLNode> {
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
    -> impl Fn(ParseInput)-> ParseResult<XMLNode> {

    //move |input, index| {
        map(
            validate(
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
                |(_, n, a, _, _, c, _, e, _, _)|{
                    if n.to_string() == e.to_string() {
                        true
                    } else {
                        false
                    }
                }
            ),
            |(_, n, a, _, _, c, _, e, _, _)| {
                // TODO: check that the start tag name and end tag name match (n == e)
                XMLNode::Element(n, a, c)
            }
        )
            //(input, index)}
}

// QualifiedName

fn qualname()
    -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    alt2(
        prefixed_name(),
        unprefixed_name(),
    )
}
fn unprefixed_name()
    -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    map (
        ncname(),
        |localpart| {
            QualifiedName::new(None, None, String::from(localpart))
        }
    )
}
fn prefixed_name()
    -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
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
    -> impl Fn(ParseInput) -> ParseResult<Vec<XMLNode>> {
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
    -> impl Fn(ParseInput) -> ParseResult<XMLNode> {
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
    -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(
        string_single(),
        string_double(),
    )
}
fn string_single()
    -> impl Fn(ParseInput) -> ParseResult<String> {
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
    -> impl Fn(ParseInput) -> ParseResult<String> {
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
    -> impl Fn(ParseInput)-> ParseResult<Vec<XMLNode>> {
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
    -> impl Fn(ParseInput) -> ParseResult<XMLNode> {
    map(
        genentityexpander(),
        |_| {
            XMLNode::Text(Value::String("".to_string()))
        }
    )
}

// PI ::= '<?' PITarget (char* - '?>') '?>'
fn processing_instruction()
    -> impl Fn(ParseInput) -> ParseResult<XMLNode> {
    validate(
        map(
            tuple5(
                tag("<?"),
                name(),
                opt(
                    tuple2(
                        whitespace1(),
                        take_until("?>")
                    )
                ),
                whitespace0(),
                tag("?>")
            ),
        |(_, n,vt , _, _)| {
            match vt {
                None => {
                    XMLNode::PI(String::from(n), Value::String("".to_string()))
                },
                Some((_, v)) => {
                    XMLNode::PI(String::from(n), Value::String(v))
                }
            }
        }
    ), | v| match v {
            XMLNode::PI(N, Value::String(v)) => {
                if v.contains(|c: char| !is_char(&c.to_string())){
                    false
                } else if N.to_lowercase() == "xml" {
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
    -> impl Fn(ParseInput) -> ParseResult<XMLNode> {
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
    -> impl Fn(ParseInput) -> ParseResult<Vec<XMLNode>> {
        map(
            tuple2(
                many0(
                    map(
                        alt2(
                            tuple2(whitespace0(),comment()),
                            tuple2(whitespace0(), processing_instruction())
                        ), |(ws,xn)|{ xn }
                    )
                ),
                whitespace0()
            ),|(v,w)|{v}
        )
}

// CharData ::= [^<&]* - (']]>')
fn chardata()
    -> impl Fn(ParseInput) -> ParseResult<String> {
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
    -> impl Fn(ParseInput) -> ParseResult<String> {
        delimited(
            tag("<![CDATA["),take_until("]]>"),tag("]]>")
    )
}

fn chardata_escapes()
    -> impl Fn(ParseInput) -> ParseResult<String> {
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
    -> impl Fn(ParseInput)-> ParseResult<String> {
    map(
        alt2(
            delimited(tag("&#x"),parse_hex(),tag(";")),
            delimited(tag("&#"),parse_decimal(),tag(";")),
        ),
        |value| std::char::from_u32(value).unwrap().to_string()
    )
}
fn parse_hex()
    -> impl Fn(ParseInput)-> ParseResult<u32>{
    map (
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
        |hex| {
            u32::from_str_radix(&*hex, 16).unwrap()
        }
    )
}
fn parse_decimal()
    -> impl Fn(ParseInput)-> ParseResult<u32>{
    map (
        take_while_m_n(1, 6, |c: char| c.is_ascii_digit()),
        |dec| {
            u32::from_str(&*dec).unwrap()
        }
    )
}


fn chardata_literal()
    -> impl Fn(ParseInput) -> ParseResult<String> {
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