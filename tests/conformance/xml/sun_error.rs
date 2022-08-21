/*

Sun Microsystems test cases

*/

use std::fs;
use xrust::parser;

#[test]
fn uri01() {
    /*
        Test ID:uri01
        Test URI:not-wf/uri01.xml
        Spec Sections:4.2.2 [75]
        Description:SYSTEM ids may not have URI fragments
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/sun/not-wf/uri01.xml").unwrap());

    assert!(testxml.is_err());

}

