/*
Richard Tobin's XML Namespaces 1.0 test suite 14 Feb 2003
 */

use std::fs;
use xrust::parser;

#[test]
fn rmtns10004() {
    /*
        Test ID:rmt-ns10-004
        Test URI:004.xml
        Spec Sections:2
        Description:Namespace name test: a relative URI (deprecated)
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/004.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
fn rmtns10005() {
    /*
        Test ID:rmt-ns10-005
        Test URI:005.xml
        Spec Sections:2
        Description:Namespace name test: a same-document relative URI (deprecated)
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/005.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
fn rmtns10006() {
    /*
        Test ID:rmt-ns10-006
        Test URI:006.xml
        Spec Sections:2
        Description:Namespace name test: an http IRI that is not a URI
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/namespaces/1.0/006.xml").unwrap());

    assert!(testxml.is_err());

}
