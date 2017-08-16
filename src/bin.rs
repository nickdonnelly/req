extern crate reqlib;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate colored;

use std::env;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use hyper::Client;
use colored::*;

use reqlib::header_editor::HeaderEditor;
use reqlib::payload::editor;

// DONT FORGET TO REVERT
fn main(){
    let config = Config::new();
    if config.is_err(){
        println!("Error: {}", config.err().unwrap());
        std::process::exit(1);
    }
    let config = config.unwrap();

    let result = run(&config);
    if result.is_err() {
        println!("Error: {}", result.err().unwrap()); // The errors are guaranteed to be non-empty
        std::process::exit(1); // close w/ non-zero exit code
    }

    println!("");
}

fn run(config: &Config) -> Result<(), String>{
    let mut headereditor: HeaderEditor = HeaderEditor::new();
    let mut payloadeditor: editor::PayloadEditor = editor::new();
    let mut configcopy = config.clone();

    headereditor.set_default_headers();
    if config.custom_headers {
        headereditor.start();
    }

    if config.custom_payload {
        payloadeditor.start();
    }

    let mut core = Core::new().expect("Couldn't create a reactor core!");
    let client = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    //let mut redirect_url: Option<String> = None;

    //loop {
        let mut request: hyper::Request;
        
     //   let uri = match redirect_url {
     //       None => Ok(config.uri.clone()),
     //       Some(x) => to_uri(x),
     //   };
     //   if uri.is_err(){
     //       return Err(format!("{}", "Couldn't parse URL!".red()));
     //   }
     //   let uri = uri.unwrap();
        let uri = config.uri.clone();
        request = hyper::Request::new(config.method.clone(), uri);
        headereditor.write_all_headers(request.headers_mut());
        request.set_body(payloadeditor.get_payload());

        let work = client.request(request).and_then(move |res| {
            print_response_code(res.status());
            if config.print_headers {
                print_header(res.headers());
            }

            if config.print_body{
                println!("\n{}", "Response body:".blue());
            }
            let redirected = res.status() == hyper::StatusCode::TemporaryRedirect || res.status() == hyper::StatusCode::PermanentRedirect || res.status() == hyper::StatusCode::Found || res.status() == hyper::StatusCode::MovedPermanently;
            let resheaders = res.headers().clone();
            res.body().for_each(move |chunk| {
                if config.print_body{
                    io::stdout().write_all(&chunk);
                }

                if redirected {
                    for header in resheaders.iter().collect::<Vec<hyper::header::HeaderView>>() {
                        if header.name().to_lowercase().trim() == "location" {
                            let location = to_uri(header.value_string());
                            if location.is_err() {
                                println!("{}", "Redirect did not contain location header!".red());
                                std::process::exit(0);
                            }
                            let location = location.unwrap();
                            
                            println!("\n{}", "Executing redirect...".magenta());
                            let new_uri = to_uri(format!("{}", location));
                            if new_uri.is_err() {
                                println!("{}", "Error: invalid redirect url.".red());
                                std::process::exit(3);
                            }
                            let new_config = Config {
                                uri: new_uri.unwrap(),
                                method: config.method.clone(),
                                print_headers: config.print_headers,
                                print_body: config.print_body,
                                custom_headers: false,
                                custom_payload: false,
                                follow_redirects: true
                                };
                            run(&new_config);        
                            break;
                        }
                    }
                    
                }
                futures::future::result( Ok(()) )
            })

        });

        let core_result = core.run(work);
        if core_result.is_err() {
            return Err(format!("{}", "Reactor core couldn't complete work! Perhaps the result was malformed?".red()));
        //}
        
        
    }

    
    Ok(())
}

struct Config {
    uri: hyper::Uri,
    method: hyper::Method,
    print_headers: bool,
    print_body: bool,
    custom_headers: bool,
    custom_payload: bool,
    follow_redirects: bool,
}

impl Config{
    // This is a big function but it's relatively simple, just parsing the arguments
    // one by one. This could be done more simply using a HashMap or something, and will
    // probably be transitioned there in the future. For now, it works.
    fn new() -> Result<Config, String>{
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err(String::from("Not enough arguments!\nUsage: req <options> [method] [uri]"));
        }

        let mut print_headers = true;
        let mut print_body = true;
        let mut custom_headers = false;
        let mut custom_payload = false;
        let mut follow_redirects = false;
        //let mut payload_file: Option<String> = None;

        for arg in &args[1..(args.len() - 2)] {
            match arg.as_str() {
                "--noheader" => {
                    print_headers = false;
                },
                "--headeronly" => {
                    print_body = false;
                },
                "--customheaders" => {
                    custom_headers = true
                },
                "--enter-payload" => {
                    custom_payload = true
                },
                "--follow-redirects" => {
                    follow_redirects = true
                },
                //"--enter-payload" => {
                    // FIX THIS!
                    //payload_file = Some(String::new())
                //},
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
            print_headers: print_headers,
            print_body: print_body,
            custom_headers: custom_headers,
            custom_payload: custom_payload,
            follow_redirects: follow_redirects,
        })
    }

    pub fn update_uri(&mut self, new_uri: hyper::Uri){
        self.uri = new_uri;
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
}

fn print_header(headers: &hyper::Headers){
    println!("{}", "Reponse Headers:".magenta());
    for header in headers.iter().collect::<Vec<hyper::header::HeaderView>>(){
        println!("{}: {}", header.name().magenta(), header.value_string());
    }
}

fn to_uri(url: String) -> Result<hyper::Uri, String>{
    let uri = url.parse::<hyper::Uri>();
    if uri.is_err() {
        return Err(String::from("Couldn't parse url"));
    }

    let mut uri = uri.unwrap();
    // Check URI scheme.
    if uri.scheme().is_none(){
        let mut uri_corrected: String = String::from("http://");
        uri_corrected.push_str(uri.to_string().as_str());
        let uri_corrected = uri_corrected.parse::<hyper::Uri>().expect("Not a valid URL!");
        uri = uri_corrected;
    }

    Ok(uri)
}
