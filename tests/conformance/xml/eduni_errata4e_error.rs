/*
University of Edinburgh XML 1.0 4th edition errata test suite.
*/

use std::fs;
use xrust::parser;

#[test]
fn invalidbo7() {
    /*
        Test ID:invalid-bo-7
        Test URI:inclbomboom_be.xml
        Spec Sections:4.3.3
        Description:A byte order mark and a backwards one in general entity cause an illegal char. error (big-endian)
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbomboom_be.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
fn invalidbo8() {
    /*
        Test ID:invalid-bo-8
        Test URI:inclbomboom_le.xml
        Spec Sections:4.3.3
        Description:A byte order mark and a backwards one in general entity cause an illegal char. error (little-endian)
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/inclbomboom_le.xml").unwrap());

    assert!(testxml.is_err());

}

#[test]
fn invalidbo9() {
    /*
        Test ID:invalid-bo-9
        Test URI:incl8bomboom.xml
        Spec Sections:4.3.3
        Description:A byte order mark and a backwards one in general entity cause an illegal char. error (utf-8)
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/incl8bomboom.xml").unwrap());

    assert!(testxml.is_err());

}



#[test]
fn xrmt008() {
    /*
        Test ID:x-rmt-008
        Test URI:008.xml
        Spec Sections:2.8 4.3.4
        Description:a document with version=1.7, illegal in XML 1.0 through 4th edition
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-4e/008.xml").unwrap());

    assert!(testxml.is_err());

}