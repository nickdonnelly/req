use std::fmt::{self, Display, Error, Formatter};
use super::*;
use std::str;

impl FailureCode {
    pub fn value(&self) -> u16 {
        match *self {
            FailureCode::NoError     => 0,
            FailureCode::ClientError => 1,
            FailureCode::IOError     => 2
        }
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let utf8_decoded = str::from_utf8(&self.data);
        if utf8_decoded.is_err() {
          return write!(f, "{:?}", &self.data);
        }
        write!(f, "{}", utf8_decoded.unwrap())
    }
}

impl Display for ReqResponse{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let utf8_decoded = str::from_utf8(&self.body);
        if utf8_decoded.is_err() {
            return write!(f, "{:?}", &self.body);
        }
        write!(f, "{}", utf8_decoded.unwrap())
    }
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        match self {
            &RequestMethod::Get     => write!(f, "GET"),
            &RequestMethod::Head    => write!(f, "HEAD"),
            &RequestMethod::Put     => write!(f, "PUT"),
            &RequestMethod::Post    => write!(f, "POST"),
            &RequestMethod::Delete  => write!(f, "DELETE"),
            &RequestMethod::Options => write!(f, "OPTIONS"),
            &RequestMethod::Patch   => write!(f, "PATCH"),
            &RequestMethod::Connect => write!(f, "CONNECT"),
            &RequestMethod::Trace   => write!(f, "TRACE")
        }
    }
}

impl Display for ReqHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        write!(f, "{}: {}", &self.name, &self.value)
    }
}
