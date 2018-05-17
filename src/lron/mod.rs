/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! lron stand for Lightroom Object Notation
//! Some sort of JSON specific to Lightroom

/// Value for the object
#[derive(Debug)]
pub enum Value {
    Dict(Vec<Object>),
    Str(String),
    ZStr(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

/// The key/value pair.
#[derive(Debug)]
pub struct Pair {
    pub key: String,
    pub value: Value,
}

/// Object
#[derive(Debug)]
pub enum Object {
    Dict(Vec<Object>),
    Pair(Pair),
    Str(String),
    ZStr(String),
    Int(i64),
}

impl Object {
    /// Create an object from a string
    pub fn from_str(s: &str) -> Result<Object, grammar::ParseError> {
        grammar::root(s)
    }
}

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/lron_grammar.rs"));
}

#[test]
fn test_parser() {
    let r = Object::from_str(
        "s = { \
         { \
         criteria = \"rating\", \
         operation = \">\", \
         value = 0, \
         value2 = 0, \
         }, \
         combine = \"intersect\", \
         }",
    );
    assert!(r.is_ok());
    if let Some(o) = r.ok() {
        println!("{:?}", o);
    }
}
