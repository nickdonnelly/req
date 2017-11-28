extern crate reqlib;
use reqlib::*;

fn main() {
        let config = ReqConfig {
            command: ReqCommand::Request(RequestMethod::Get),
            host: Some(String::from("www.google.com")),
            port: Some(443),
            timeout: Some(10000),
            payload: Some(Payload { data: vec![1,2,3], content_type: String::from("application/octet-stream") }),
            options: None
        };
        let req = Req::new_from_cfg(config).unwrap();
}
