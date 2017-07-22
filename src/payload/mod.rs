extern crate hyper;
extern crate colored;

use std;
use self::hyper::Body;

enum EncodingType {
    UTF8,
    UTF16,
}

pub struct Payload {
    body: Body
}

impl Payload {
    pub fn set_body(&mut self, bytes: Vec<u8>) {
        self.body = Body::from(bytes);
    }

    pub fn set_body_string(&mut self, s: String){
        self.body = Body::from(s);
    }

    pub fn body(&self) -> &Body {
        return &self.body;
    }

    pub fn body_mut(&mut self) -> &mut Body {
        return &mut self.body;
    }
}

impl Payload {
    pub fn new() -> Self {
        Payload {
            body: Body::empty(),
        }
    }
}

impl From<String> for Payload {
    fn from(s: String) -> Payload {
        Payload {
            body: Body::from(s),
        }
    }
}

impl From<Vec<u8>> for Payload {
    fn from(b: Vec<u8>) -> Payload {
        Payload {
            body: Body::from(b),
        }
    }
}
