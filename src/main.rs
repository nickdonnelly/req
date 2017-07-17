extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate colored;

use std::env;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use hyper::Client;
use colored::*;


fn main(){
    let config = Config::new();
    if config.is_err(){
        println!("Error: {}", config.err().unwrap());
        std::process::exit(1);
    }
    let config = config.unwrap();

    let mut core = Core::new().expect("Couldn't create a reactor core!");
    let client = Client::new(&core.handle());

    let request = hyper::Request::new(config.method.clone(), config.uri.clone());
    let work = client.request(request).and_then(move |res| {
        print_response_code(res.status());
        if config.print_headers {
            print_header(res.headers());
        }

        println!("{}", "Response body:".blue());
        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });

    core.run(work).expect("Couldn't execute request!");
    println!("");
}

struct Config {
    uri: hyper::Uri,
    method: hyper::Method,
    print_headers: bool,
}

impl Config{
    fn new() -> Result<Config, String>{
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err(String::from("Not enough arguments!\nUsage: req <options> [method] [uri]"));
        }

        let mut print_headers = true;

        for arg in &args[1..(args.len() - 2)] {
            match arg.as_str() {
                "--noheader" => {
                    print_headers = false;
                },
                _ => {
                    return Err(format!("Unknown argument: {}", arg).to_owned());
                }
            };
        }

        let uri = args.last().unwrap().parse::<hyper::Uri>();
        if uri.is_err(){
            return Err(String::from("Unable to parse URL!"));
        }

        let mut uri = uri.unwrap();

        let method = args.get(args.len() - 2).unwrap().to_uppercase().parse::<hyper::Method>().expect("Not a valid HTTP method!");
        
        if uri.scheme().is_none(){
            let mut uri_corrected: String = String::from("http://");
            uri_corrected.push_str(uri.to_string().as_str());
            let uri_corrected = uri_corrected.parse::<hyper::Uri>().expect("Not a valid URL!");
            uri = uri_corrected;
        }

        Ok(Config{
            uri: uri,
            method: method,
            print_headers: print_headers
        })
    }
}

/// Prints the response code with a color depending on the type (range) of the response code.
/// Uses ANSI colors (so does not work on windows)
/// Response Code Range | Color
/// 100-199               blue
/// 200-299               green
/// 300-399               cyan
/// 400-499               magenta
/// 500-599               red
/// <100 or >599          default
fn print_response_code(code: hyper::StatusCode){
    if code.is_informational(){
        println!("{}", code.to_string().blue());
    }else if code.is_success(){
        println!("{}", code.to_string().green());
    }else if code.is_redirection(){
        println!("{}", code.to_string().cyan());
    }else if code.is_client_error(){
        println!("{}", code.to_string().magenta());
    }else if code.is_server_error(){
        println!("{}", code.to_string().red());
    }else{
        println!("{}", code);
    }
    println!("");
}

fn print_header(headers: &hyper::Headers){
    println!("{}", "Headers:".magenta());
    for header in headers.iter().collect::<Vec<hyper::header::HeaderView>>(){
        println!("{}: {}", header.name().magenta(), header.value_string());
    }
    println!("");
}
