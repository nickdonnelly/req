use std;
/* 
    let encoder = encode::base64::Base64Encoder::new(&payload);
    let result: Result<String, Error> = encoder.encode();
*/

mod base64;

trait Encoder {
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
