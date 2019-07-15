use hyper::{ self, Body, Response, Server, StatusCode, HeaderMap };
use hyper::service::service_fn_ok;
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

    pub fn start(self, port: u16, sc: StatusCode)
    {
        let mut addr = ([127, 0, 0, 1], port).into();

        if let SocketType::Literal(lit) = self.socket_type {
            let mk_srv = || {
                service_fn_ok(|_req| {
                    Response::new(Body::from(lit))
                })
            };

            let server = Server::bind(&addr)
                .serve(mk_srv);

            hyper::rt::run(server.map_err(|err| {
                eprintln!("Problem with hyper server: {}", err);
            }));
        } 
        /*
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
        */
    }

}


/// Return a body that is the pretty version of the given request.
/*
pub fn pretty_print(headers: HeaderMap, req_meth: Method, req_path: String) -> String 
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
*/

/// Struct used to respond to requests with their body
struct EchoService {
    status_code: StatusCode
}

/// Struct use for constant responses
struct OkService {
    status_code: StatusCode,
    phrase: String
}
