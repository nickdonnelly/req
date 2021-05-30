use hyper::{ self, Body, Server, Request, Response, StatusCode, HeaderMap, Method };
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use colored::*;

pub struct QuickSocket {
    socket_type: SocketType
}

#[derive(PartialEq, Debug)]
pub enum SocketType {
    Talkback,
    Literal(String)
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
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

    pub async fn start(self, port: u16)
    {
        let addr = ([127, 0, 0, 1], port).into();

        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(hello_world))
        });

        let server = Server::bind(&addr).serve(make_svc);
        if let Err(e) = server.await {
            println!("server error: {}", e);
        }

        //if let SocketType::Literal(lit) = self.socket_type {
        //} else { // default to talkback, other clauses to go here if more are added
         //   let server = Http::new().keep_alive(false).bind(&addr, move || Ok(EchoService{
          //      status_code: StatusCode::OK
           // })).unwrap();
            //server.run().unwrap()
        //}
    }

}


/// Return a body that is the pretty version of the given request.
pub fn pretty_print(headers: HeaderMap, req_meth: Method, req_path: String) -> String 
{
    let mut headerstring = String::new();
    let methstr = format!("{} {}\n", req_meth, req_path);

    headerstring.push_str(&format!("{}", methstr.magenta()));

    headerstring.push_str(&format!("{}", "---Headers---\n".cyan()));
    for headerview in headers.iter() {
        headerstring.push_str(
            &format!("{}{} {}\n", headerview.0.as_str().bold(), ":".bold(), headerview.1.to_str().unwrap())
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
