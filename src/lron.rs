/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

//! lron stands for Lightroom Object Notation, specific to Lightroom
//! that is found throughout the catalog database to store abtrary bu
//! structured data.
//!
//! lron looks like plist (before XML) or JSON, but doesn't match
//! either.
//!
//! Note: I couldn't figure out what this format was called, so I
//! couldn't reuse an existing parser. If you have a better idea,
//! please, let me know.
//!
//! It has the form
//! ```json
//! name = {
//!   object = {
//!     x = 1.3,
//!     string = "some text",
//!   },
//! }
//! ```
//!
//! The text is parsed using peg.
//!
//! You obtain the expression from the text by the following:
//! ```
//! use lrcat::lron;
//!
//! let lron_text = "name = {}"; // load the text in the string
//!
//! if let Ok(object) = lron::Object::from_string(lron_text) {
//!     // do your stuff with it
//! }
//! ```

/// Lron Value
#[derive(Debug, PartialEq)]
pub enum Value {
    Dict(Vec<Object>),
    Str(String),
    ZStr(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

/// A key/value pair.
#[derive(Debug, PartialEq)]
pub struct Pair {
    pub key: String,
    pub value: Value,
}

/// Lron Object
#[derive(Debug, PartialEq)]
pub enum Object {
    Dict(Vec<Object>),
    Pair(Pair),
    Str(String),
    ZStr(String),
    Int(i64),
}

/// Alias result type for parsing a Lron object.
type Result<T> = std::result::Result<T, peg::error::ParseError<peg::str::LineCol>>;

impl Object {
    /// Create an object from a string
    pub fn from_string(s: &str) -> Result<Object> {
        lron::root(s)
    }
}

// lron stand for Lightroom Object Notation
// Some sort of JSON specific to Lightroom
//
// lron data syntax is defined in this PEG grammar.
peg::parser! {grammar lron() for str {

use std::str::FromStr;

pub rule root() -> Object
        = key:identifier() _() "=" _() value:array() _()
    { Object::Pair(Pair{key, value: Value::Dict(value)}) }

rule array() -> Vec<Object>
        = "{" _() v:(object() ** (_() "," _())) _()(",")? _() "}" { v }

rule object() -> Object
        = a:array() { Object::Dict(a) } /
        p:pair() { Object::Pair(p) } /
        s:string_literal() { Object::Str(s) } /
        z:zstr() { Object::ZStr(z) } /
        n:int() { Object::Int(n) }

rule pair() -> Pair
        = key:identifier() _() "=" _() value:value() { Pair { key, value } } /
        "[" key:string_literal() "]" _() "=" _() value:value()
    { Pair { key, value } }

rule value() -> Value
        = i:int() { Value::Int(i) } /
        b:bool() { Value::Bool(b) } /
        f:float() { Value::Float(f) } /
        s:string_literal() { Value::Str(s) } /
        a:array() { Value::Dict(a) } /
        z:zstr() { Value::ZStr(z) }

rule int() -> i64
        = n:$("-"? ['0'..='9']+) !"." { i64::from_str(n).unwrap() } / expected!("integer")

rule bool() -> bool
        = "true" { true } / "false" { false }

rule float() -> f64
        = f:$("-"? ['0'..='9']+ "." ['0'..='9']+) { f64::from_str(f).unwrap() } / expected!("floating point")

rule identifier() -> String
        = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) { s.to_owned() } / expected!("identifier")

rule string_literal() -> String
        = "\"" s:$((!['"'][_])*) "\"" { s.to_owned() }

rule zstr() -> String
        = "ZSTR" _() s:string_literal() { s }

rule _() = quiet!{[' ' | '\r' | '\n' | '\t']*}

}}

#[test]
fn test_parser() {
    const DATA: &'static str = "s = { \
         { \
         criteria = \"rating\", \
         operation = \">\", \
         value = 0, \
         value2 = 0, \
         }, \
         combine = \"intersect\", \
         }";
    let r = Object::from_string(DATA);

    assert!(r.is_ok());
    let o = r.unwrap();

    println!("{:?}", o);
    assert!(matches!(o, Object::Pair(_)));
    if let Object::Pair(ref p) = o {
        assert_eq!(p.key, "s");
        assert!(matches!(p.value, Value::Dict(_)));

        if let Value::Dict(ref d) = p.value {
            assert_eq!(d.len(), 2);
            assert!(matches!(d[0], Object::Dict(_)));
            if let Object::Dict(ref d) = d[0] {
                assert_eq!(d.len(), 4);
                assert!(matches!(d[0], Object::Pair(_)));
                assert!(matches!(d[1], Object::Pair(_)));
                assert!(matches!(d[2], Object::Pair(_)));
                assert!(matches!(d[3], Object::Pair(_)));
            }
            assert!(matches!(d[1], Object::Pair(_)));
            if let Object::Pair(ref p) = d[1] {
                assert_eq!(p.key, "combine");
                assert_eq!(p.value, Value::Str("intersect".to_owned()));
            }
        }
    } else {
        assert!(false);
    }
}
