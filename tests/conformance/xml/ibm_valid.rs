/*

IBM test cases

*/

use std::fs;
use xrust::parser;


#[test]
fn ibmvalidP01ibm01v01xml() {
    /*
        Test ID:ibm-valid-P01-ibm01v01.xml
        Test URI:valid/P01/ibm01v01.xml
        Spec Sections:2.1
        Description:Tests with a xml document consisting of prolog followed by element then Misc
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P01/ibm01v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P01/out/ibm01v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP02ibm02v01xml() {
    /*
        Test ID:ibm-valid-P02-ibm02v01.xml
        Test URI:valid/P02/ibm02v01.xml
        Spec Sections:2.2
        Description:This test case covers legal character ranges plus discrete legal characters for production 02.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P02/ibm02v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP03ibm03v01xml() {
    /*
        Test ID:ibm-valid-P03-ibm03v01.xml
        Test URI:valid/P03/ibm03v01.xml
        Spec Sections:2.3
        Description:Tests all 4 legal white space characters - #x20 #x9 #xD #xA
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P03/ibm03v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP09ibm09v01xml() {
    /*
        Test ID:ibm-valid-P09-ibm09v01.xml
        Test URI:valid/P09/ibm09v01.xml
        Spec Sections:2.3
        Description:Empty EntityValue is legal
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/ibm09v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/out/ibm09v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP09ibm09v02xml() {
    /*
        Test ID:ibm-valid-P09-ibm09v02.xml
        Test URI:valid/P09/ibm09v02.xml
        Spec Sections:2.3
        Description:Tests a normal EnitityValue
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/ibm09v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/out/ibm09v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP09ibm09v03xml() {
    /*
        Test ID:ibm-valid-P09-ibm09v03.xml
        Test URI:valid/P09/ibm09v03.xml
        Spec Sections:2.3
        Description:Tests EnitityValue referencing a Parameter Entity
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/ibm09v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/out/ibm09v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP09ibm09v04xml() {
    /*
        Test ID:ibm-valid-P09-ibm09v04.xml
        Test URI:valid/P09/ibm09v04.xml
        Spec Sections:2.3
        Description:Tests EnitityValue referencing a General Entity
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/ibm09v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/out/ibm09v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP09ibm09v05xml() {
    /*
        Test ID:ibm-valid-P09-ibm09v05.xml
        Test URI:valid/P09/ibm09v05.xml
        Spec Sections:2.3
        Description:Tests EnitityValue with combination of GE, PE and text, the GE used is declared in the student.dtd
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/ibm09v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P09/out/ibm09v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP10ibm10v01xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v01.xml
        Test URI:valid/P10/ibm10v01.xml
        Spec Sections:2.3
        Description:Tests empty AttValue with double quotes as the delimiters
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v02xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v02.xml
        Test URI:valid/P10/ibm10v02.xml
        Spec Sections:2.3
        Description:Tests empty AttValue with single quotes as the delimiters
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v03xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v03.xml
        Test URI:valid/P10/ibm10v03.xml
        Spec Sections:2.3
        Description:Test AttValue with double quotes as the delimiters and single quote inside
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v04xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v04.xml
        Test URI:valid/P10/ibm10v04.xml
        Spec Sections:2.3
        Description:Test AttValue with single quotes as the delimiters and double quote inside
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v05xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v05.xml
        Test URI:valid/P10/ibm10v05.xml
        Spec Sections:2.3
        Description:Test AttValue with a GE reference and double quotes as the delimiters
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v06xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v06.xml
        Test URI:valid/P10/ibm10v06.xml
        Spec Sections:2.3
        Description:Test AttValue with a GE reference and single quotes as the delimiters
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v06.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v06.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v07xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v07.xml
        Test URI:valid/P10/ibm10v07.xml
        Spec Sections:2.3
        Description:testing AttValue with mixed references and text content in double quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v07.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v07.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP10ibm10v08xml() {
    /*
        Test ID:ibm-valid-P10-ibm10v08.xml
        Test URI:valid/P10/ibm10v08.xml
        Spec Sections:2.3
        Description:testing AttValue with mixed references and text content in single quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/ibm10v08.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P10/out/ibm10v08.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP11ibm11v01xml() {
    /*
        Test ID:ibm-valid-P11-ibm11v01.xml
        Test URI:valid/P11/ibm11v01.xml
        Spec Sections:2.3
        Description:Tests empty systemliteral using the double quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/ibm11v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/out/ibm11v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP11ibm11v02xml() {
    /*
        Test ID:ibm-valid-P11-ibm11v02.xml
        Test URI:valid/P11/ibm11v02.xml
        Spec Sections:2.3
        Description:Tests empty systemliteral using the single quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/ibm11v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/out/ibm11v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP11ibm11v03xml() {
    /*
        Test ID:ibm-valid-P11-ibm11v03.xml
        Test URI:valid/P11/ibm11v03.xml
        Spec Sections:2.3
        Description:Tests regular systemliteral using the single quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/ibm11v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/out/ibm11v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP11ibm11v04xml() {
    /*
        Test ID:ibm-valid-P11-ibm11v04.xml
        Test URI:valid/P11/ibm11v04.xml
        Spec Sections:2.3
        Description:Tests regular systemliteral using the double quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/ibm11v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P11/out/ibm11v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP12ibm12v01xml() {
    /*
        Test ID:ibm-valid-P12-ibm12v01.xml
        Test URI:valid/P12/ibm12v01.xml
        Spec Sections:2.3
        Description:Tests empty systemliteral using the double quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/ibm12v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/out/ibm12v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP12ibm12v02xml() {
    /*
        Test ID:ibm-valid-P12-ibm12v02.xml
        Test URI:valid/P12/ibm12v02.xml
        Spec Sections:2.3
        Description:Tests empty systemliteral using the single quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/ibm12v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/out/ibm12v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP12ibm12v03xml() {
    /*
        Test ID:ibm-valid-P12-ibm12v03.xml
        Test URI:valid/P12/ibm12v03.xml
        Spec Sections:2.3
        Description:Tests regular systemliteral using the double quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/ibm12v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/out/ibm12v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP12ibm12v04xml() {
    /*
        Test ID:ibm-valid-P12-ibm12v04.xml
        Test URI:valid/P12/ibm12v04.xml
        Spec Sections:2.3
        Description:Tests regular systemliteral using the single quotes
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/ibm12v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P12/out/ibm12v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP13ibm13v01xml() {
    /*
        Test ID:ibm-valid-P13-ibm13v01.xml
        Test URI:valid/P13/ibm13v01.xml
        Spec Sections:2.3
        Description:Testing PubidChar with all legal PubidChar in a PubidLiteral
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P13/ibm13v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P13/out/ibm13v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP14ibm14v01xml() {
    /*
        Test ID:ibm-valid-P14-ibm14v01.xml
        Test URI:valid/P14/ibm14v01.xml
        Spec Sections:2.4
        Description:Testing CharData with empty string
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P14/ibm14v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P14/out/ibm14v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP14ibm14v02xml() {
    /*
        Test ID:ibm-valid-P14-ibm14v02.xml
        Test URI:valid/P14/ibm14v02.xml
        Spec Sections:2.4
        Description:Testing CharData with white space character
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P14/ibm14v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P14/out/ibm14v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP14ibm14v03xml() {
    /*
        Test ID:ibm-valid-P14-ibm14v03.xml
        Test URI:valid/P14/ibm14v03.xml
        Spec Sections:2.4
        Description:Testing CharData with a general text string
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P14/ibm14v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P14/out/ibm14v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP15ibm15v01xml() {
    /*
        Test ID:ibm-valid-P15-ibm15v01.xml
        Test URI:valid/P15/ibm15v01.xml
        Spec Sections:2.5
        Description:Tests empty comment
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/ibm15v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/out/ibm15v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP15ibm15v02xml() {
    /*
        Test ID:ibm-valid-P15-ibm15v02.xml
        Test URI:valid/P15/ibm15v02.xml
        Spec Sections:2.5
        Description:Tests comment with regular text
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/ibm15v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/out/ibm15v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP15ibm15v03xml() {
    /*
        Test ID:ibm-valid-P15-ibm15v03.xml
        Test URI:valid/P15/ibm15v03.xml
        Spec Sections:2.5
        Description:Tests comment with one dash inside
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/ibm15v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/out/ibm15v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP15ibm15v04xml() {
    /*
        Test ID:ibm-valid-P15-ibm15v04.xml
        Test URI:valid/P15/ibm15v04.xml
        Spec Sections:2.5
        Description:Tests comment with more comprehensive content
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/ibm15v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P15/out/ibm15v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP16ibm16v01xml() {
    /*
        Test ID:ibm-valid-P16-ibm16v01.xml
        Test URI:valid/P16/ibm16v01.xml
        Spec Sections:2.6
        Description:Tests PI definition with only PItarget name and nothing else
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P16/ibm16v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P16/out/ibm16v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP16ibm16v02xml() {
    /*
        Test ID:ibm-valid-P16-ibm16v02.xml
        Test URI:valid/P16/ibm16v02.xml
        Spec Sections:2.6
        Description:Tests PI definition with only PItarget name and a white space
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P16/ibm16v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P16/out/ibm16v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP16ibm16v03xml() {
    /*
        Test ID:ibm-valid-P16-ibm16v03.xml
        Test URI:valid/P16/ibm16v03.xml
        Spec Sections:2.6
        Description:Tests PI definition with PItarget name and text that contains question mark and right angle
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P16/ibm16v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P16/out/ibm16v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP17ibm17v01xml() {
    /*
        Test ID:ibm-valid-P17-ibm17v01.xml
        Test URI:valid/P17/ibm17v01.xml
        Spec Sections:2.6
        Description:Tests PITarget name
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P17/ibm17v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P17/out/ibm17v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP18ibm18v01xml() {
    /*
        Test ID:ibm-valid-P18-ibm18v01.xml
        Test URI:valid/P18/ibm18v01.xml
        Spec Sections:2.7
        Description:Tests CDSect with CDStart CData CDEnd
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P18/ibm18v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P18/out/ibm18v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP19ibm19v01xml() {
    /*
        Test ID:ibm-valid-P19-ibm19v01.xml
        Test URI:valid/P19/ibm19v01.xml
        Spec Sections:2.7
        Description:Tests CDStart
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P19/ibm19v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P19/out/ibm19v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP20ibm20v01xml() {
    /*
        Test ID:ibm-valid-P20-ibm20v01.xml
        Test URI:valid/P20/ibm20v01.xml
        Spec Sections:2.7
        Description:Tests CDATA with empty string
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P20/ibm20v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P20/out/ibm20v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP20ibm20v02xml() {
    /*
        Test ID:ibm-valid-P20-ibm20v02.xml
        Test URI:valid/P20/ibm20v02.xml
        Spec Sections:2.7
        Description:Tests CDATA with regular content
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P20/ibm20v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P20/out/ibm20v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP21ibm21v01xml() {
    /*
        Test ID:ibm-valid-P21-ibm21v01.xml
        Test URI:valid/P21/ibm21v01.xml
        Spec Sections:2.7
        Description:Tests CDEnd
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P21/ibm21v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P21/out/ibm21v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP22ibm22v01xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v01.xml
        Test URI:valid/P22/ibm22v01.xml
        Spec Sections:2.8
        Description:Tests prolog with XMLDecl and doctypedecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP22ibm22v02xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v02.xml
        Test URI:valid/P22/ibm22v02.xml
        Spec Sections:2.8
        Description:Tests prolog with doctypedecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP22ibm22v03xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v03.xml
        Test URI:valid/P22/ibm22v03.xml
        Spec Sections:2.8
        Description:Tests prolog with Misc doctypedecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP22ibm22v04xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v04.xml
        Test URI:valid/P22/ibm22v04.xml
        Spec Sections:2.8
        Description:Tests prolog with doctypedecl Misc
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP22ibm22v05xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v05.xml
        Test URI:valid/P22/ibm22v05.xml
        Spec Sections:2.8
        Description:Tests prolog with XMLDecl Misc doctypedecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP22ibm22v06xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v06.xml
        Test URI:valid/P22/ibm22v06.xml
        Spec Sections:2.8
        Description:Tests prolog with XMLDecl doctypedecl Misc
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v06.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v06.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP22ibm22v07xml() {
    /*
        Test ID:ibm-valid-P22-ibm22v07.xml
        Test URI:valid/P22/ibm22v07.xml
        Spec Sections:2.8
        Description:Tests prolog with XMLDecl Misc doctypedecl Misc
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/ibm22v07.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P22/out/ibm22v07.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP23ibm23v01xml() {
    /*
        Test ID:ibm-valid-P23-ibm23v01.xml
        Test URI:valid/P23/ibm23v01.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with VersionInfo only
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/ibm23v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/out/ibm23v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP23ibm23v02xml() {
    /*
        Test ID:ibm-valid-P23-ibm23v02.xml
        Test URI:valid/P23/ibm23v02.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with VersionInfo EncodingDecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/ibm23v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/out/ibm23v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP23ibm23v03xml() {
    /*
        Test ID:ibm-valid-P23-ibm23v03.xml
        Test URI:valid/P23/ibm23v03.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with VersionInfo SDDecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/ibm23v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/out/ibm23v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP23ibm23v04xml() {
    /*
        Test ID:ibm-valid-P23-ibm23v04.xml
        Test URI:valid/P23/ibm23v04.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with VerstionInfo and a trailing whitespace char
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/ibm23v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/out/ibm23v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP23ibm23v05xml() {
    /*
        Test ID:ibm-valid-P23-ibm23v05.xml
        Test URI:valid/P23/ibm23v05.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with VersionInfo EncodingDecl SDDecl
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/ibm23v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/out/ibm23v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP23ibm23v06xml() {
    /*
        Test ID:ibm-valid-P23-ibm23v06.xml
        Test URI:valid/P23/ibm23v06.xml
        Spec Sections:2.8
        Description:Tests XMLDecl with VersionInfo EncodingDecl SDDecl and a trailing whitespace
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/ibm23v06.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P23/out/ibm23v06.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP24ibm24v01xml() {
    /*
        Test ID:ibm-valid-P24-ibm24v01.xml
        Test URI:valid/P24/ibm24v01.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with single quote
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P24/ibm24v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P24/out/ibm24v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP24ibm24v02xml() {
    /*
        Test ID:ibm-valid-P24-ibm24v02.xml
        Test URI:valid/P24/ibm24v02.xml
        Spec Sections:2.8
        Description:Tests VersionInfo with double quote
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P24/ibm24v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P24/out/ibm24v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP25ibm25v01xml() {
    /*
        Test ID:ibm-valid-P25-ibm25v01.xml
        Test URI:valid/P25/ibm25v01.xml
        Spec Sections:2.8
        Description:Tests EQ with =
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/ibm25v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/out/ibm25v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP25ibm25v02xml() {
    /*
        Test ID:ibm-valid-P25-ibm25v02.xml
        Test URI:valid/P25/ibm25v02.xml
        Spec Sections:2.8
        Description:Tests EQ with = and spaces on both sides
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/ibm25v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/out/ibm25v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP25ibm25v03xml() {
    /*
        Test ID:ibm-valid-P25-ibm25v03.xml
        Test URI:valid/P25/ibm25v03.xml
        Spec Sections:2.8
        Description:Tests EQ with = and space in front of it
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/ibm25v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/out/ibm25v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP25ibm25v04xml() {
    /*
        Test ID:ibm-valid-P25-ibm25v04.xml
        Test URI:valid/P25/ibm25v04.xml
        Spec Sections:2.8
        Description:Tests EQ with = and space after it
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/ibm25v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P25/out/ibm25v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP26ibm26v01xml() {
    /*
        Test ID:ibm-valid-P26-ibm26v01.xml
        Test URI:valid/P26/ibm26v01.xml
        Spec Sections:2.8
        Description:Tests VersionNum 1.0
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P26/ibm26v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P26/out/ibm26v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP27ibm27v01xml() {
    /*
        Test ID:ibm-valid-P27-ibm27v01.xml
        Test URI:valid/P27/ibm27v01.xml
        Spec Sections:2.8
        Description:Tests Misc with comment
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P27/ibm27v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P27/out/ibm27v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP27ibm27v02xml() {
    /*
        Test ID:ibm-valid-P27-ibm27v02.xml
        Test URI:valid/P27/ibm27v02.xml
        Spec Sections:2.8
        Description:Tests Misc with PI
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P27/ibm27v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P27/out/ibm27v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP27ibm27v03xml() {
    /*
        Test ID:ibm-valid-P27-ibm27v03.xml
        Test URI:valid/P27/ibm27v03.xml
        Spec Sections:2.8
        Description:Tests Misc with white spaces
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P27/ibm27v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P27/out/ibm27v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP28ibm28v01xml() {
    /*
        Test ID:ibm-valid-P28-ibm28v01.xml
        Test URI:valid/P28/ibm28v01.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with internal DTD only
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P28/ibm28v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P28/out/ibm28v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP28ibm28v02xml() {
    /*
        Test ID:ibm-valid-P28-ibm28v02.xml
        Test URI:valid/P28/ibm28v02.xml
        Spec Sections:2.8
        Description:Tests doctypedecl with external subset and combinations of different markup declarations and PEReferences
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P28/ibm28v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P28/out/ibm28v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP29ibm29v01xml() {
    /*
        Test ID:ibm-valid-P29-ibm29v01.xml
        Test URI:valid/P29/ibm29v01.xml
        Spec Sections:2.8
        Description:Tests markupdecl with combinations of elementdecl, AttlistDecl,EntityDecl, NotationDecl, PI and comment
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P29/ibm29v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P29/out/ibm29v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP29ibm29v02xml() {
    /*
        Test ID:ibm-valid-P29-ibm29v02.xml
        Test URI:valid/P29/ibm29v02.xml
        Spec Sections:2.8
        Description:Tests WFC: PE in internal subset as a positive test
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P29/ibm29v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P29/out/ibm29v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP30ibm30v01xml() {
    /*
        Test ID:ibm-valid-P30-ibm30v01.xml
        Test URI:valid/P30/ibm30v01.xml
        Spec Sections:2.8
        Description:Tests extSubset with extSubsetDecl only in the dtd file
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P30/ibm30v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P30/out/ibm30v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP30ibm30v02xml() {
    /*
        Test ID:ibm-valid-P30-ibm30v02.xml
        Test URI:valid/P30/ibm30v02.xml
        Spec Sections:2.8
        Description:Tests extSubset with TextDecl and extSubsetDecl in the dtd file
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P30/ibm30v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P30/out/ibm30v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP31ibm31v01xml() {
    /*
        Test ID:ibm-valid-P31-ibm31v01.xml
        Test URI:valid/P31/ibm31v01.xml
        Spec Sections:2.8
        Description:Tests extSubsetDecl with combinations of markupdecls, conditionalSects, PEReferences and white spaces
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P31/ibm31v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P31/out/ibm31v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP32ibm32v01xml() {
    /*
        Test ID:ibm-valid-P32-ibm32v01.xml
        Test URI:valid/P32/ibm32v01.xml
        Spec Sections:2.9
        Description:Tests VC: Standalone Document Declaration with absent attribute that has default value and standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/ibm32v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/out/ibm32v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP32ibm32v02xml() {
    /*
        Test ID:ibm-valid-P32-ibm32v02.xml
        Test URI:valid/P32/ibm32v02.xml
        Spec Sections:2.9
        Description:Tests VC: Standalone Document Declaration with external entity reference and standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/ibm32v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/out/ibm32v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP32ibm32v03xml() {
    /*
        Test ID:ibm-valid-P32-ibm32v03.xml
        Test URI:valid/P32/ibm32v03.xml
        Spec Sections:2.9
        Description:Tests VC: Standalone Document Declaration with attribute values that need to be normalized and standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/ibm32v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/out/ibm32v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP32ibm32v04xml() {
    /*
        Test ID:ibm-valid-P32-ibm32v04.xml
        Test URI:valid/P32/ibm32v04.xml
        Spec Sections:2.9
        Description:Tests VC: Standalone Document Declaration with whitespace in mixed content and standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/ibm32v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P32/out/ibm32v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP33ibm33v01xml() {
    /*
        Test ID:ibm-valid-P33-ibm33v01.xml
        Test URI:valid/P33/ibm33v01.xml
        Spec Sections:2.12
        Description:Tests LanguageID with Langcode - Subcode
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P33/ibm33v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P33/out/ibm33v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP34ibm34v01xml() {
    /*
        Test ID:ibm-valid-P34-ibm34v01.xml
        Test URI:valid/P34/ibm34v01.xml
        Spec Sections:2.12
        Description:Duplicate Test as ibm33v01.xml
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P34/ibm34v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P34/out/ibm34v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP35ibm35v01xml() {
    /*
        Test ID:ibm-valid-P35-ibm35v01.xml
        Test URI:valid/P35/ibm35v01.xml
        Spec Sections:2.12
        Description:Tests ISO639Code
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P35/ibm35v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P35/out/ibm35v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP36ibm36v01xml() {
    /*
        Test ID:ibm-valid-P36-ibm36v01.xml
        Test URI:valid/P36/ibm36v01.xml
        Spec Sections:2.12
        Description:Tests IanaCode
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P36/ibm36v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P36/out/ibm36v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP37ibm37v01xml() {
    /*
        Test ID:ibm-valid-P37-ibm37v01.xml
        Test URI:valid/P37/ibm37v01.xml
        Spec Sections:2.12
        Description:Tests UserCode
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P37/ibm37v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P37/out/ibm37v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP38ibm38v01xml() {
    /*
        Test ID:ibm-valid-P38-ibm38v01.xml
        Test URI:valid/P38/ibm38v01.xml
        Spec Sections:2.12
        Description:Tests SubCode
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P38/ibm38v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P38/out/ibm38v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP39ibm39v01xml() {
    /*
        Test ID:ibm-valid-P39-ibm39v01.xml
        Test URI:valid/P39/ibm39v01.xml
        Spec Sections:3
        Description:Tests element with EmptyElemTag and STag content Etag, also tests the VC: Element Valid with elements that have children, Mixed and ANY contents
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P39/ibm39v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P39/out/ibm39v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP40ibm40v01xml() {
    /*
        Test ID:ibm-valid-P40-ibm40v01.xml
        Test URI:valid/P40/ibm40v01.xml
        Spec Sections:3.1
        Description:Tests STag with possible combinations of its fields, also tests WFC: Unique Att Spec.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P40/ibm40v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P40/out/ibm40v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP41ibm41v01xml() {
    /*
        Test ID:ibm-valid-P41-ibm41v01.xml
        Test URI:valid/P41/ibm41v01.xml
        Spec Sections:3.1
        Description:Tests Attribute with Name Eq AttValue and VC: Attribute Value Type
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P41/ibm41v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P41/out/ibm41v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP42ibm42v01xml() {
    /*
        Test ID:ibm-valid-P42-ibm42v01.xml
        Test URI:valid/P42/ibm42v01.xml
        Spec Sections:3.1
        Description:Tests ETag with possible combinations of its fields
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P42/ibm42v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P42/out/ibm42v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP43ibm43v01xml() {
    /*
        Test ID:ibm-valid-P43-ibm43v01.xml
        Test URI:valid/P43/ibm43v01.xml
        Spec Sections:3.1
        Description:Tests content with all possible constructs: element, CharData, Reference, CDSect, Comment
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P43/ibm43v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P43/out/ibm43v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP44ibm44v01xml() {
    /*
        Test ID:ibm-valid-P44-ibm44v01.xml
        Test URI:valid/P44/ibm44v01.xml
        Spec Sections:3.1
        Description:Tests EmptyElemTag with possible combinations of its fields
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P44/ibm44v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P44/out/ibm44v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP45ibm45v01xml() {
    /*
        Test ID:ibm-valid-P45-ibm45v01.xml
        Test URI:valid/P45/ibm45v01.xml
        Spec Sections:3.2
        Description:Tests both P45 elementDecl and P46 contentspec with possible combinations of their constructs
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P45/ibm45v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P45/out/ibm45v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP47ibm47v01xml() {
    /*
        Test ID:ibm-valid-P47-ibm47v01.xml
        Test URI:valid/P47/ibm47v01.xml
        Spec Sections:3.2.1
        Description:Tests all possible children,cp,choice,seq patterns in P47,P48,P49,P50
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P47/ibm47v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P47/out/ibm47v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP49ibm49v01xml() {
    /*
        Test ID:ibm-valid-P49-ibm49v01.xml
        Test URI:valid/P49/ibm49v01.xml
        Spec Sections:3.2.1
        Description:Tests VC:Proper Group/PE Nesting with PEs of choices that are properly nested with parenthesized groups in external subsets
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P49/ibm49v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P49/out/ibm49v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP50ibm50v01xml() {
    /*
        Test ID:ibm-valid-P50-ibm50v01.xml
        Test URI:valid/P50/ibm50v01.xml
        Spec Sections:3.2.1
        Description:Tests VC:Proper Group/PE Nesting with PEs of seq that are properly nested with parenthesized groups in external subsets
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P50/ibm50v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P50/out/ibm50v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP51ibm51v01xml() {
    /*
        Test ID:ibm-valid-P51-ibm51v01.xml
        Test URI:valid/P51/ibm51v01.xml
        Spec Sections:3.2.2
        Description:Tests Mixed with possible combinations of its fields amd VC: No Duplicate Types
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P51/ibm51v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P51/out/ibm51v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP51ibm51v02xml() {
    /*
        Test ID:ibm-valid-P51-ibm51v02.xml
        Test URI:valid/P51/ibm51v02.xml
        Spec Sections:3.2.2
        Description:Tests VC:Proper Group/PE Nesting with PEs of Mixed that are properly nested with parenthesized groups in external subsets
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P51/ibm51v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P51/out/ibm51v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP52ibm52v01xml() {
    /*
        Test ID:ibm-valid-P52-ibm52v01.xml
        Test URI:valid/P52/ibm52v01.xml
        Spec Sections:3.3
        Description:Tests all AttlistDecl and AttDef Patterns in P52 and P53
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P52/ibm52v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P52/out/ibm52v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP54ibm54v01xml() {
    /*
        Test ID:ibm-valid-P54-ibm54v01.xml
        Test URI:valid/P54/ibm54v01.xml
        Spec Sections:3.3.1
        Description:Tests all AttTypes : StringType, TokenizedTypes, EnumeratedTypes in P55,P56,P57,P58,P59. Also tests all DefaultDecls in P60.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P54/ibm54v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP54ibm54v02xml() {
    /*
        Test ID:ibm-valid-P54-ibm54v02.xml
        Test URI:valid/P54/ibm54v02.xml
        Spec Sections:3.3.1
        Description:Tests all AttTypes : StringType, TokenizedType, EnumeratedTypes in P55,P56,P57.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P54/ibm54v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P54/out/ibm54v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP54ibm54v03xml() {
    /*
        Test ID:ibm-valid-P54-ibm54v03.xml
        Test URI:valid/P54/ibm54v03.xml
        Spec Sections:3.3.1
        Description:Tests AttTypes with StringType in P55.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P54/ibm54v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P54/out/ibm54v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP55ibm55v01xml() {
    /*
        Test ID:ibm-valid-P55-ibm55v01.xml
        Test URI:valid/P55/ibm55v01.xml
        Spec Sections:3.3.1
        Description:Tests StringType for P55. The "CDATA" occurs in the StringType for the attribute "att" for the element "a".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P55/ibm55v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P55/out/ibm55v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP56ibm56v01xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v01.xml
        Test URI:valid/P56/ibm56v01.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56. The "ID", "IDREF", "IDREFS", "ENTITY", "ENTITIES", "NMTOKEN", and "NMTOKENS" occur in the TokenizedType for the attribute "attr".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v02xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v02.xml
        Test URI:valid/P56/ibm56v02.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: ID Attribute Default. The value "AC1999" is assigned to the ID attribute "attr" with "#REQUIRED" in the DeaultDecl.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v03xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v03.xml
        Test URI:valid/P56/ibm56v03.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: ID Attribute Default. The value "AC1999" is assigned to the ID attribute "attr" with "#IMPLIED" in the DeaultDecl.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v04xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v04.xml
        Test URI:valid/P56/ibm56v04.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: ID. The ID attribute "UniqueName" appears only once in the document.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v05xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v05.xml
        Test URI:valid/P56/ibm56v05.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: One ID per element type. The element "a" or "b" has only one ID attribute.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v06xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v06.xml
        Test URI:valid/P56/ibm56v06.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: IDREF. The IDREF value "AC456" matches the value assigned to an ID attribute "UniqueName".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v06.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v06.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v07xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v07.xml
        Test URI:valid/P56/ibm56v07.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: IDREF. The IDREFS value "AC456 Q123" matches the values assigned to the ID attribute "UniqueName" and "Uname".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v07.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v07.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v08xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v08.xml
        Test URI:valid/P56/ibm56v08.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: Entity Name. The value "image" of the ENTITY attribute "sun" matches the name of an unparsed entity declared.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v08.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v08.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v09xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v09.xml
        Test URI:valid/P56/ibm56v09.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: Name Token. The value of the NMTOKEN attribute "thistoken" matches the Nmtoken production.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v09.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v09.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP56ibm56v10xml() {
    /*
        Test ID:ibm-valid-P56-ibm56v10.xml
        Test URI:valid/P56/ibm56v10.xml
        Spec Sections:3.3.1
        Description:Tests TokenizedType for P56 VC: Name Token. The value of the NMTOKENS attribute "thistoken" matches the Nmtoken production.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/ibm56v10.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P56/out/ibm56v10.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP57ibm57v01xml() {
    /*
        Test ID:ibm-valid-P57-ibm57v01.xml
        Test URI:valid/P57/ibm57v01.xml
        Spec Sections:3.3.1
        Description:Tests EnumeratedType in the AttType. The attribute "att" has a type (a|b) with the element "a". the
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P57/ibm57v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P57/out/ibm57v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP58ibm58v01xml() {
    /*
        Test ID:ibm-valid-P58-ibm58v01.xml
        Test URI:valid/P58/ibm58v01.xml
        Spec Sections:3.3.1
        Description:Tests NotationType for P58. It shows different patterns fro the NOTATION attribute "attr".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P58/ibm58v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P58/out/ibm58v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP58ibm58v02xml() {
    /*
        Test ID:ibm-valid-P58-ibm58v02.xml
        Test URI:valid/P58/ibm58v02.xml
        Spec Sections:3.3.1
        Description:Tests NotationType for P58: Notation Attributes. The value "base64" of the NOTATION attribute "attr" matches one of the notation names declared.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P58/ibm58v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P58/out/ibm58v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP59ibm59v01xml() {
    /*
        Test ID:ibm-valid-P59-ibm59v01.xml
        Test URI:valid/P59/ibm59v01.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration in the EnumeratedType for P59. It shows different patterns for the Enumeration attribute "attr".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P59/ibm59v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P59/out/ibm59v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP59ibm59v02xml() {
    /*
        Test ID:ibm-valid-P59-ibm59v02.xml
        Test URI:valid/P59/ibm59v02.xml
        Spec Sections:3.3.1
        Description:Tests Enumeration for P59 VC: Enumeration. The value "one" of the Enumeration attribute "attr" matches one of the element names declared.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P59/ibm59v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P59/out/ibm59v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP60ibm60v01xml() {
    /*
        Test ID:ibm-valid-P60-ibm60v01.xml
        Test URI:valid/P60/ibm60v01.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl for P60. It shows different options "#REQUIRED", "#FIXED", "#IMPLIED", and default for the attribute "chapter".
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/ibm60v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/out/ibm60v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP60ibm60v02xml() {
    /*
        Test ID:ibm-valid-P60-ibm60v02.xml
        Test URI:valid/P60/ibm60v02.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl for P60 VC: Required Attribute. In the element "one" and "two" the value of the #REQUIRED attribute "chapter" is given.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/ibm60v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/out/ibm60v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP60ibm60v03xml() {
    /*
        Test ID:ibm-valid-P60-ibm60v03.xml
        Test URI:valid/P60/ibm60v03.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl for P60 VC: Fixed Attribute Default. The value of the #FIXED attribute "chapter" is exactly the same as the default value.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/ibm60v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/out/ibm60v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP60ibm60v04xml() {
    /*
        Test ID:ibm-valid-P60-ibm60v04.xml
        Test URI:valid/P60/ibm60v04.xml
        Spec Sections:3.3.2
        Description:Tests DefaultDecl for P60 VC: Attribute Default Legal. The default value specified for the attribute "attr" meets the lexical constraints of the declared attribute type.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/ibm60v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P60/out/ibm60v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP61ibm61v01xml() {
    /*
        Test ID:ibm-valid-P61-ibm61v01.xml
        Test URI:valid/P61/ibm61v01.xml
        Spec Sections:3.4
        Description:Tests conditionalSect for P61. It takes the option "invludeSect" in the file ibm61v01.dtd.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P61/ibm61v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P61/out/ibm61v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP61ibm61v02xml() {
    /*
        Test ID:ibm-valid-P61-ibm61v02.xml
        Test URI:valid/P61/ibm61v02.xml
        Spec Sections:3.4
        Description:Tests conditionalSect for P61. It takes the option "ignoreSect" in the file ibm61v02.dtd.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P61/ibm61v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P61/out/ibm61v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP62ibm62v01xml() {
    /*
        Test ID:ibm-valid-P62-ibm62v01.xml
        Test URI:valid/P62/ibm62v01.xml
        Spec Sections:3.4
        Description:Tests includeSect for P62. The white space is not included before the key word "INCLUDE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/ibm62v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/out/ibm62v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP62ibm62v02xml() {
    /*
        Test ID:ibm-valid-P62-ibm62v02.xml
        Test URI:valid/P62/ibm62v02.xml
        Spec Sections:3.4
        Description:Tests includeSect for P62. The white space is not included after the key word "INCLUDE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/ibm62v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/out/ibm62v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP62ibm62v03xml() {
    /*
        Test ID:ibm-valid-P62-ibm62v03.xml
        Test URI:valid/P62/ibm62v03.xml
        Spec Sections:3.4
        Description:Tests includeSect for P62. The white space is included after the key word "INCLUDE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/ibm62v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/out/ibm62v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP62ibm62v04xml() {
    /*
        Test ID:ibm-valid-P62-ibm62v04.xml
        Test URI:valid/P62/ibm62v04.xml
        Spec Sections:3.4
        Description:Tests includeSect for P62. The white space is included before the key word "INCLUDE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/ibm62v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/out/ibm62v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP62ibm62v05xml() {
    /*
        Test ID:ibm-valid-P62-ibm62v05.xml
        Test URI:valid/P62/ibm62v05.xml
        Spec Sections:3.4
        Description:Tests includeSect for P62. The extSubsetDecl is not included.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/ibm62v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P62/out/ibm62v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP63ibm63v01xml() {
    /*
        Test ID:ibm-valid-P63-ibm63v01.xml
        Test URI:valid/P63/ibm63v01.xml
        Spec Sections:3.4
        Description:Tests ignoreSect for P63. The white space is not included before the key word "IGNORE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/ibm63v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/out/ibm63v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP63ibm63v02xml() {
    /*
        Test ID:ibm-valid-P63-ibm63v02.xml
        Test URI:valid/P63/ibm63v02.xml
        Spec Sections:3.4
        Description:Tests ignoreSect for P63. The white space is not included after the key word "IGNORE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/ibm63v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/out/ibm63v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP63ibm63v03xml() {
    /*
        Test ID:ibm-valid-P63-ibm63v03.xml
        Test URI:valid/P63/ibm63v03.xml
        Spec Sections:3.4
        Description:Tests ignoreSect for P63. The white space is included after the key word "IGNORE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/ibm63v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/out/ibm63v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP63ibm63v04xml() {
    /*
        Test ID:ibm-valid-P63-ibm63v04.xml
        Test URI:valid/P63/ibm63v04.xml
        Spec Sections:3.4
        Description:Tests ignoreSect for P63. The ignireSectContents is included.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/ibm63v04.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/out/ibm63v04.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP63ibm63v05xml() {
    /*
        Test ID:ibm-valid-P63-ibm63v05.xml
        Test URI:valid/P63/ibm63v05.xml
        Spec Sections:3.4
        Description:Tests ignoreSect for P63. The white space is included before and after the key word "IGNORE" in the beginning sequence.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/ibm63v05.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P63/out/ibm63v05.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP64ibm64v01xml() {
    /*
        Test ID:ibm-valid-P64-ibm64v01.xml
        Test URI:valid/P64/ibm64v01.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents for P64. One "ignore" field is included.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P64/ibm64v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P64/out/ibm64v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP64ibm64v02xml() {
    /*
        Test ID:ibm-valid-P64-ibm64v02.xml
        Test URI:valid/P64/ibm64v02.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents for P64. Two "ignore" and one "ignoreSectContents" fields are included.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P64/ibm64v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P64/out/ibm64v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP64ibm64v03xml() {
    /*
        Test ID:ibm-valid-P64-ibm64v03.xml
        Test URI:valid/P64/ibm64v03.xml
        Spec Sections:3.4
        Description:Tests ignoreSectContents for P64. Four "ignore" and three "ignoreSectContents" fields are included.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P64/ibm64v03.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P64/out/ibm64v03.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP65ibm65v01xml() {
    /*
        Test ID:ibm-valid-P65-ibm65v01.xml
        Test URI:valid/P65/ibm65v01.xml
        Spec Sections:3.4
        Description:Tests Ignore for P65. An empty string occurs in the Ignore filed.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P65/ibm65v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P65/out/ibm65v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP65ibm65v02xml() {
    /*
        Test ID:ibm-valid-P65-ibm65v02.xml
        Test URI:valid/P65/ibm65v02.xml
        Spec Sections:3.4
        Description:Tests Ignore for P65. An string not including the brackets occurs in each of the Ignore filed.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P65/ibm65v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P65/out/ibm65v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}



#[test]
fn ibmvalidP66ibm66v01xml() {
    /*
        Test ID:ibm-valid-P66-ibm66v01.xml
        Test URI:valid/P66/ibm66v01.xml
        Spec Sections:4.1
        Description:Tests all legal CharRef's.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P66/ibm66v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P66/out/ibm66v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP67ibm67v01xml() {
    /*
        Test ID:ibm-valid-P67-ibm67v01.xml
        Test URI:valid/P67/ibm67v01.xml
        Spec Sections:4.1
        Description:Tests Reference could be EntityRef or CharRef.
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P67/ibm67v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P67/out/ibm67v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP68ibm68v01xml() {
    /*
        Test ID:ibm-valid-P68-ibm68v01.xml
        Test URI:valid/P68/ibm68v01.xml
        Spec Sections:4.1
        Description:Tests P68 VC:Entity Declared with Entities in External Subset , standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P68/ibm68v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P68/out/ibm68v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP68ibm68v02xml() {
    /*
        Test ID:ibm-valid-P68-ibm68v02.xml
        Test URI:valid/P68/ibm68v02.xml
        Spec Sections:4.1
        Description:Tests P68 VC:Entity Declared with Entities in External Parameter Entities , standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P68/ibm68v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P68/out/ibm68v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP69ibm69v01xml() {
    /*
        Test ID:ibm-valid-P69-ibm69v01.xml
        Test URI:valid/P69/ibm69v01.xml
        Spec Sections:4.1
        Description:Tests P68 VC:Entity Declared with Parameter Entities in External Subset , standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P69/ibm69v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P69/out/ibm69v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}

#[test]
fn ibmvalidP69ibm69v02xml() {
    /*
        Test ID:ibm-valid-P69-ibm69v02.xml
        Test URI:valid/P69/ibm69v02.xml
        Spec Sections:4.1
        Description:Tests P68 VC:Entity Declared with Parameter Entities in External Parameter Entities, standalone is no
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P69/ibm69v02.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P69/out/ibm69v02.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP70ibm70v01xml() {
    /*
        Test ID:ibm-valid-P70-ibm70v01.xml
        Test URI:valid/P70/ibm70v01.xml
        Spec Sections:4.2
        Description:Tests all legal GEDecls and PEDecls constructs derived from P70-76
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P70/ibm70v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P70/out/ibm70v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP78ibm78v01xml() {
    /*
        Test ID:ibm-valid-P78-ibm78v01.xml
        Test URI:valid/P78/ibm78v01.xml
        Spec Sections:4.3.2
        Description:Tests ExtParsedEnt, also TextDecl in P77 and EncodingDecl in P80
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P78/ibm78v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P78/out/ibm78v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP79ibm79v01xml() {
    /*
        Test ID:ibm-valid-P79-ibm79v01.xml
        Test URI:valid/P79/ibm79v01.xml
        Spec Sections:4.3.2
        Description:Tests extPE
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P79/ibm79v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP82ibm82v01xml() {
    /*
        Test ID:ibm-valid-P82-ibm82v01.xml
        Test URI:valid/P82/ibm82v01.xml
        Spec Sections:4.7
        Description:Tests NotationDecl in P82 and PublicID in P83
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P82/ibm82v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P82/out/ibm82v01.xml").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP85ibm85v01xml() {
    /*
        Test ID:ibm-valid-P85-ibm85v01.xml
        Test URI:valid/P85/ibm85v01.xml
        Spec Sections:B.
        Description:This test case covers 149 legal character ranges plus 51 single legal characters for BaseChar in P85 using a PI target Name
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P85/ibm85v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP86ibm86v01xml() {
    /*
        Test ID:ibm-valid-P86-ibm86v01.xml
        Test URI:valid/P86/ibm86v01.xml
        Spec Sections:B.
        Description:This test case covers 2 legal character ranges plus 1 single legal characters for IdeoGraphic in P86 using a PI target Name
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P86/ibm86v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP87ibm87v01xml() {
    /*
        Test ID:ibm-valid-P87-ibm87v01.xml
        Test URI:valid/P87/ibm87v01.xml
        Spec Sections:B.
        Description:This test case covers 65 legal character ranges plus 30 single legal characters for CombiningChar in P87 using a PI target Name
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P87/ibm87v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP88ibm88v01xml() {
    /*
        Test ID:ibm-valid-P88-ibm88v01.xml
        Test URI:valid/P88/ibm88v01.xml
        Spec Sections:B.
        Description:This test case covers 15 legal character ranges for Digit in P88 using a PI target Name
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P88/ibm88v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}




#[test]
fn ibmvalidP89ibm89v01xml() {
    /*
        Test ID:ibm-valid-P89-ibm89v01.xml
        Test URI:valid/P89/ibm89v01.xml
        Spec Sections:B.
        Description:This test case covers 3 legal character ranges plus 8 single legal characters for Extender in P89 using a PI target Name
    */

    let testxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/valid/P89/ibm89v01.xml").unwrap());
    let canonicalxml = parser::xml::parse(fs::read_to_string("tests/conformance/xml/xmlconf/ibm/").unwrap());

    assert!(testxml.is_ok());
    assert!(canonicalxml.is_ok());
    assert!(testxml.unwrap() == canonicalxml.unwrap());

}



