use std;
use std::sync::mpsc::{ self, Sender, Receiver };
use std::thread;
use std::error::Error;

use futures;
use futures::{ Future, Stream };
use hyper;
use hyper::{ Body, Chunk };
use hyper::header::ContentLength;
use hyper::server::{ Service, Http, Request, Response };

use super::{ ReqResponse };

pub struct QuickSocket {
    service_name: String
}

#[derive(Debug)]
struct QuickSocketError {
    description: String
}

impl QuickSocket {

    /// Returns an instance of QuickSocket ready to open on 
    pub fn new() -> QuickSocket
    {
        QuickSocket { 
            service_name: String::from("echo")
        }
    }

    pub fn start(self, port: usize)
    {
        // TODO allow this to actually switch between services.
        let mut addr = String::from("127.0.0.1:");
        addr.push_str(&format!("{}", port));
        let addr = addr.parse().unwrap();
        let server = Http::new().keep_alive(false).bind(&addr, || Ok(EchoService{})).unwrap();
        server.run().unwrap()
    }

}


/// Return a body that is the pretty version of the given request.
pub fn pretty_print(req: Request) -> Body
{
    let headers = req.headers().clone();
    let req_meth = req.method().clone();
    let req_path = String::from(req.path());
    let mut headerstring = String::new();

    headerstring.push_str(&format!("{} {}", req_meth, req_path));

    headerstring.push_str("Headers:\n");
    for headerview in headers.iter() {
        headerstring.push_str(&format!("{}: {}\n", headerview.name(), headerview.value_string()));
    }
    headerstring.push_str("---\n");

    let body = req.body().concat2().map(|chnk| {
        chnk.iter().cloned().collect::<Vec<u8>>()
    }).wait().unwrap();
    let bodylen = body.len();
    let bodystring = String::from_utf8(body);
    let bodystring = match bodystring {
        Err(e) => format!("Body:\n<{} bytes>", bodylen),
        Ok(bodystr) => format!("Body:\n{}", bodystr)
    };

    headerstring.push_str(&bodystring);
    println!("{}\n\n", &headerstring);
    Body::from(headerstring)
}

/// Struct used to respond to requests with their body
struct EchoService;
struct OkService;

impl Service for OkService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future 
    {
        let hw = "ok";
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(hw.len() as u64))
                .with_body(hw)
        ))
    }
}

impl Service for EchoService {
    type Request = Request;
    type Error = hyper::Error;
    type Response = Response;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future
    {
        let mut response = Response::new();
        let response_body = pretty_print(req);
        response.set_body(response_body);
        Box::new(futures::future::ok(response))
    }

}

impl std::fmt::Display for QuickSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Socket Error:\n{}", &self.description)
    }
}

impl Error for QuickSocketError {
    fn description(&self) -> &str
    {
        &self.description
    }
}
