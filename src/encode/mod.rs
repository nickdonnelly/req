use std;
use super::Payload;

pub mod base64;

#[derive(Clone, Debug, PartialEq)]
pub enum Encoding {
    NoEncoding,
    Base64,
    UnknownEncoding(String)
}

impl Encoding {
    pub fn from_str(s: &str) -> Option<Self>
    {
        match s {
            "base64" => Some(Encoding::Base64),
            _        => None
        }
    }
}

pub trait Encoder {
    fn encode(&self, &mut Payload) -> Result<(), EncodeError>;
}

#[derive(Debug)]
pub struct EncodeError {
    desc: String
}

impl std::error::Error for EncodeError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.desc)
    }
}
