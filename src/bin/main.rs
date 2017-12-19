extern crate reqlib;
extern crate dotenv;
extern crate clap;
extern crate stopwatch;
extern crate colored;

use reqlib::*;
use dotenv::dotenv;
use stopwatch::Stopwatch;
use colored::*;

use std::str;
use std::process;
use std::io::Write;

mod utils;

fn main() {
    dotenv().ok(); // Add the dotenv environment variables to env::vars
    let mut config = ReqConfig::new()
        .global_defaults()
        .environment_defaults(); // Environment must be after so it overrides global.
    
    config = utils::process_arguments(config)
        .unwrap_or_else(|e| e.exit());

    let print_types: Vec<ReqOption> = get_print_options(&config.options);

    let req = Req::new_from_cfg(config).unwrap();

    let mut watch = Stopwatch::start_new();
    let result = req.run();
    watch.stop();

    if result.is_err() {
        print_error_message(result.err());
    } else {
        handle_result(result.unwrap(), print_types, watch.elapsed_ms());
    }
}

fn get_print_options(opts: &Vec<ReqOption>) -> Vec<ReqOption> {
    let mut result: Vec<ReqOption> = Vec::new();
    for opt in opts {
        match opt {
            &ReqOption::PRINT(ref v) => 
                result.push(ReqOption::PRINT(v.clone())),
            _ => {}
        }
    }
    result
}

fn handle_result(res: ReqCommandResult, print_flags: Vec<ReqOption>, elapsed_millis: i64) {
    if res.to_show.is_some() {
        println!("{}", res.to_show.unwrap());
    } else if res.response.is_some() {
        // We're gonna move the 'res' object so we have to print the config in case
        // we need it after the move.
        let config_string = format!("{}", &res.from_config);
        let response = res.response.unwrap();

        if print_flags.len() == 0 {
            print_response_time(elapsed_millis);
            print_response_status(&response.status);
            print_headers(&response.headers, "Response Headers:");
            print_body(&response.body);
        } else {
            print_flags.iter().for_each(|flag| {
                if let &ReqOption::PRINT(ref v) = flag {
                    match v.to_lowercase().as_str() {
                        "status" => print_response_status(&response.status),
                        "response-time" => print_response_time(elapsed_millis),
                        "headers" => print_headers(&response.headers, "Response Headers:"),
                        "request-headers" => print_headers(&response.request_headers, "Request Headers:"),
                        "body" => print_body(&response.body),
                        "config" => println!("{}", config_string),
                        _ => {}
                    }
                }
            });
        }
    } else {
        println!("Result was unexpected: {:?}", res);
    }
}

fn print_response_status(status: &ReqResponseStatus)
{
    match status.status_type() {
        ReqStatusType::Success => println!("Server responded with {}.", status.as_string().green()),
        ReqStatusType::Redirect => println!("Server responded with {}.", status.as_string().magenta()),
        ReqStatusType::Information => println!("Server responded with {}.", status.as_string().cyan()),
        ReqStatusType::ClientFailure => println!("Server responded with {}.", status.as_string().red()),
        ReqStatusType::ServerFailure => println!("Server responded with {}.", status.as_string().yellow()),
        ReqStatusType::UnknownType => println!("Server responded with {}.", status.as_string().blue())
    };
}

fn print_response_time(elapsed_millis: i64)
{
    let printable = if elapsed_millis < 100 {
        format!("Response Time: {} ms", elapsed_millis.to_string().green())
    } else if elapsed_millis < 500 { 
        format!("Response Time: {} ms", elapsed_millis.to_string().yellow())
    } else {
        format!("Response Time: {} ms", elapsed_millis.to_string().red())
    };

    println!("{}", printable);

}

fn print_body(body: &Vec<u8>)
{
    let body_s = match str::from_utf8(body) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 in response! Aborting print:\n{}", e)
    };

    if body.len() == 0 {
        println!("{}", "<body was empty>".cyan());
    } else {
        println!("{}", body_s);
    }
}

fn print_headers(headers: &Vec<ReqHeader>, title: &str)
{
    println!("{}", title);
    for header in headers {
        let mut values: Vec<&str> = header.value.split(";").collect();
        let title_len = header.name.len() + 1;

        let initial_value = values.swap_remove(0);
        println!("{}: {};", header.name.bold(), initial_value);

        for remaining in values {
            println!("{:width$}{};", " ", remaining, width=title_len);
        }
    }
    println!("---");
}

fn print_error_message(e: Option<ReqError>)
{
    if e.is_none() {
        println!("An unknown error occurred.");
    } else {
        let e = e.unwrap();
        println!("An error occurred:\n{}", &e.description);
        process::exit(e.exit_code.value() as i32);
    }
}
