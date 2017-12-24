use std;

pub mod base64;

pub trait Encoder {
    fn encode(&self) -> Result<String, EncodeError>;
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
