#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    #[serde(rename = "pieces")]
    pub pieces: Vec<Vec<models::Field>>,

}

impl Board {
    pub fn new(pieces: Vec<Vec<models::Field>>, ) -> Board {
        Board {
            pieces: pieces,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "type")]
    pub _type: String,

    #[serde(rename = "X")]
    pub x: isize,

    #[serde(rename = "Y")]
    pub y: isize,

}

impl Field {
    pub fn new(_type: String, x: isize, y: isize, ) -> Field {
        Field {
            _type: _type,
            x: x,
            y: y,
        }
    }
}
