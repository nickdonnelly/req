use futures::{ Future, Stream };
use hyper;
use hyper::{ Body, Method, Headers, StatusCode };
use hyper::server::{ Service, Http, Request, Response };
use colored::*;

pub struct QuickSocket {
    socket_type: SocketType
}

#[derive(PartialEq, Debug)]
pub enum SocketType {
    Talkback,
    Literal(String)
}

impl QuickSocket {

    /// Returns an instance of QuickSocket ready to open on 
    /// @param t The type of socket you'd like (talkback or literal)
    pub fn new(t: SocketType) -> QuickSocket
    {
        QuickSocket { 
            socket_type: t
        }
    }

    pub fn start(self, port: usize, sc: StatusCode)
    {
        let mut addr = String::from("127.0.0.1:");
        addr.push_str(&format!("{}", port));
        let addr = addr.parse().unwrap();
        if let SocketType::Literal(lit) = self.socket_type {
            let server = Http::new().keep_alive(false).bind(&addr, move || Ok(OkService{
                status_code: sc,
                phrase: lit.clone() // this has to be here otherwise it only fulfills FnOnce, not Fn
            })).unwrap();
            server.run().unwrap()
        } else { // default to talkback, other clauses to go here if more are added
            let server = Http::new().keep_alive(false).bind(&addr, move || Ok(EchoService{
                status_code: sc
            })).unwrap();
            server.run().unwrap()
        }
    }

}


/// Return a body that is the pretty version of the given request.
pub fn pretty_print(headers: Headers, req_meth: Method, req_path: String) -> String 
{
    let mut headerstring = String::new();
    let methstr = format!("{} {}\n", req_meth, req_path);

    headerstring.push_str(&format!("{}", methstr.magenta()));

    headerstring.push_str(&format!("{}", "---Headers---\n".cyan()));
    for headerview in headers.iter() {
        headerstring.push_str(
            &format!("{}{} {}\n", headerview.name().bold(), ":".bold(), headerview.value_string())
        );
    }
    headerstring.push_str(format!("{}", "---Body---\n".red()).as_str());
    headerstring
}

/// Struct used to respond to requests with their body
struct EchoService {
    status_code: StatusCode
}

/// Struct use for constant responses
struct OkService {
    status_code: StatusCode,
    phrase: String
}

impl Service for OkService {
    type Request = Request;
    type Error = hyper::Error;
    type Response = Response;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future 
    {
        let headers = req.headers().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let path = String::from(format!("{}", uri).as_str());
        let status = self.status_code.clone();
        let phrase = self.phrase.clone();
        let mut response_body_start = pretty_print(headers, method, path);

        Box::new(req.body().concat2().map(move |body_chunk|{
            let fbody = body_chunk.iter().cloned().collect::<Vec<u8>>();
            let blen = fbody.len();
            let body_string = String::from_utf8(fbody);
            let body_string = if body_string.is_err() {
                format!("<{} {}>", blen.to_string().green().bold(), " bytes".green().bold())
            } else {
                format!("{}", body_string.unwrap().italic())
            };

            response_body_start.push_str(&body_string);
            response_body_start.push_str(format!("{}", "\n---End Body---\n\n".red()).as_str());
            println!("{}", response_body_start);
            let body = Body::from(phrase);
            Response::new().with_status(status).with_body(body)
        }))
    }
}

impl Service for EchoService {
    type Request = Request;
    type Error = hyper::Error;
    type Response = Response;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future
    {
        let headers = req.headers().clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let path = String::from(format!("{}", uri).as_str());
        let status = self.status_code.clone();
        let mut response_body_start = pretty_print(headers, method, path);
        Box::new(req.body().concat2().map(move |body_chunk|{
            let fbody = body_chunk.iter().cloned().collect::<Vec<u8>>();
            let blen = fbody.len();
            let body_string = String::from_utf8(fbody);
            let body_string = if body_string.is_err() {
                format!("<{} {}>", blen.to_string().green().bold(), " bytes".green().bold())
            } else {
                format!("{}", body_string.unwrap().italic())
            };

            response_body_start.push_str(&body_string);
            response_body_start.push_str(format!("{}", "\n---End Body---\n\n".red()).as_str());
            println!("{}", response_body_start);
            Body::from(response_body_start)
        }).map(move |final_body| {
            Response::new().with_status(status).with_body(final_body)
        }))
    }

}
