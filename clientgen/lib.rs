#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::convert;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct API {
    pub name: String,
    pub description: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub returns: ReturnVal,
    pub method: String,
    pub url: String,
    pub description: String,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnVal {
    pub object: String,
    pub human: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename="type")]
    pub type_: ParamType,
    pub bounds: Option<Vec<String>>,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParamType {
    #[serde(rename="primitive")]
    Primitive(String),
    #[serde(rename="variants")]
    Variants(Vec<Variant>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    #[serde(rename="type")]
    pub type_: String,
    bounds: Option<Vec<String>>,
}

// Errors
#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    SerdeError(serde_json::error::Error),
    PathError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IoError(ref err) => write!(f, "{}", err),
            &Error::SerdeError(ref err) => write!(f, "{}", err),
            &Error::PathError(ref err) => write!(f, "{}", err),
        }
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error::SerdeError(e)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

fn titlecase(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    let chars = word.chars().enumerate();
    chars.map(|(i, c)| {
             if i == 0 {
                 c.to_uppercase().next().unwrap()
             } else {
                 c
             }
         })
         .collect::<String>()
}

pub fn from_api(from_path: &Path, to_dir: &Path) -> Result<String> {
    let basename = try!(from_path.file_stem()
                                 .ok_or(Error::PathError("unknown file_steam".to_string())));
    let fd = try!(File::open(from_path));
    let outfilename = Path::new(to_dir).join(basename).with_extension("rs");
    let api: API = try!(serde_json::from_reader(fd));
    let module = try!(generate_module(&api));
    let mut outfile = try!(OpenOptions::new()
                               .write(true)
                               .create(true)
                               .open(outfilename));
    try!(write!(outfile, "{}", &module));
    Ok(basename.to_string_lossy().into_owned())
}

pub fn generate_module(api: &API) -> Result<String> {
    let mut module = String::new();

    let struct_name = format!("{}Client", &titlecase(&api.name));

    module.push_str(&format!("struct {};", &struct_name));
    Ok(module)
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;
    use std::path::Path;

    #[test]
    fn test_deserialize() {

        let accountjson = include_str!("../api/account.json");

        let _: API = serde_json::from_str(accountjson).expect("Could not deserialize account.json");
    }

    #[test]
    fn test_gen_module() {
        let path = Path::new("../api/account.json");
        let _ = from_api(&path, &Path::new("clients"));
    }
}
