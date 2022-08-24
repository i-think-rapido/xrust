/*
Richard Tobin's XML 1.0 3rd edition errata test suite 1 June 2006
 */

use std::fs;
use xrust::parser;

#[test]
fn rmte3e06a() {
    /*
        Test ID:rmt-e3e-06a
        Test URI:E06a.xml
        Spec Sections:E06
        Description:Default values for IDREF attributes must match Name.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06a.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06b() {
    /*
        Test ID:rmt-e3e-06b
        Test URI:E06b.xml
        Spec Sections:E06
        Description:Default values for ENTITY attributes must match Name.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06b.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06c() {
    /*
        Test ID:rmt-e3e-06c
        Test URI:E06c.xml
        Spec Sections:E06
        Description:Default values for IDREFS attributes must match Names.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06c.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06d() {
    /*
        Test ID:rmt-e3e-06d
        Test URI:E06d.xml
        Spec Sections:E06
        Description:Default values for ENTITIES attributes must match Names.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06d.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06e() {
    /*
        Test ID:rmt-e3e-06e
        Test URI:E06e.xml
        Spec Sections:E06
        Description:Default values for NMTOKEN attributes must match Nmtoken.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06e.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06f() {
    /*
        Test ID:rmt-e3e-06f
        Test URI:E06f.xml
        Spec Sections:E06
        Description:Default values for NMTOKENS attributes must match Nmtokens.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06f.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06g() {
    /*
        Test ID:rmt-e3e-06g
        Test URI:E06g.xml
        Spec Sections:E06
        Description:Default values for NOTATION attributes must match one of the enumerated values.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06g.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e06h() {
    /*
        Test ID:rmt-e3e-06h
        Test URI:E06h.xml
        Spec Sections:E06
        Description:Default values for enumerated attributes must match one of the enumerated values.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E06h.xml").unwrap());

    assert!(testxml.is_err());
}


#[test]
fn rmte3e13() {
    /*
        Test ID:rmt-e3e-13
        Test URI:E13.xml
        Spec Sections:E13
        Description:Even internal parameter entity references are enough to make undeclared entities into mere validity errors rather than well-formedness errors.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/eduni/errata-3e/E13.xml").unwrap());

    assert!(testxml.is_err());
}


