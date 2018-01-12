use reqlib::{ ReqConfig, ReqOption, RequestMethod, ReqCommand, Payload, FailureCode };
use reqlib::encode::Encoding;
use clap::{Values, ArgMatches};
use std::process;
use std::env;
use std::str::FromStr;

pub fn setup_show_resource<'a>(show_matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    match show_matches.subcommand() {
        ("payload", Some(payload_matches)) => super::show::show_payload(cfg, payload_matches),
        ("env", Some(env_matches)) => super::show::show_env(cfg, env_matches),
        (other, _) => { 
            eprintln!("Unknown subcommand '{}' for show.", other); 
            process::exit(FailureCode::ClientError.value() as i32);
            cfg
        }
    }
}

pub fn setup_request<'a>(meth: &str, request_matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    // Add the command
    let req_method = RequestMethod::from_str(meth);
    let cfg = if req_method.is_ok() {
        cfg.command(ReqCommand::Request(req_method.unwrap()))
    } else {
        cfg
    };

    // Add any flags
    let cfg = header_flags(request_matches.values_of("header"), cfg);
    let cfg = print_flags(request_matches.values_of("print"), cfg);
    let cfg = timeout_flag(request_matches.value_of("timeout"), cfg);
    let cfg = redirect_flag(request_matches.value_of("max-redirects"), cfg);
    // *PAYLOAD MUST COME BEFORE ENCODING!
    let cfg = payload_arg(request_matches.value_of("payload"), request_matches.value_of("encoding"), cfg);

    // Add the URI
    if let Ok(v) = env::var("REQ_URI") {
        let mut env_host = v.to_string();
        let clap_host = request_matches.value_of("uri").unwrap(); // safe to unwrap since env:var is defined.

        if ReqConfig::fix_schemeless_uri(clap_host) == env_host {
            cfg.host(v.to_string())
        } else if clap_host.starts_with("/") { // we were given a relative path.
            env_host.push_str(clap_host);
            cfg.host(env_host)
        } else {
            cfg.host_str(clap_host)
        }

    } else {
        let clap_host = request_matches.value_of("uri");
        if clap_host.is_some() {
            cfg.host_str(clap_host.unwrap())
        } else {
            cfg
        }
    }
}

pub fn setup_no_subcommand<'a>(matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    // Add any flags
    let cfg = header_flags(matches.values_of("header"), cfg);
    let cfg = print_flags(matches.values_of("print"), cfg);
    let cfg = timeout_flag(matches.value_of("timeout"), cfg);
    let cfg = redirect_flag(matches.value_of("max-redirects"), cfg);
    // ** PAYLOAD MUST COME BEFORE ENCODING!
    let cfg = payload_arg(matches.value_of("payload"), matches.value_of("encoding"), cfg);

    if let Ok(v) = env::var("REQ_URI") {
        let mut env_host = v.to_string();
        let clap_host = matches.value_of("uri").unwrap(); // safe to unwrap since env:var is defined.

        if ReqConfig::fix_schemeless_uri(clap_host) == env_host {
            cfg.host(v.to_string())
        } else if clap_host.starts_with("/") { // we were given a relative path.
            env_host.push_str(clap_host);
            cfg.host(env_host)
        } else {
            cfg.host_str(clap_host)
        }

    } else {
        let clap_host = matches.value_of("uri");
        if clap_host.is_some() {
            cfg.host_str(clap_host.unwrap())
        } else {
            cfg
        }
    }
}

pub fn print_flags<'a>(print_flags: Option<Values<'a>>, cfg: ReqConfig)
    -> ReqConfig
{
    if print_flags.is_some() {
        let mut print_options: Vec<ReqOption> = Vec::new();
        let values: Vec<&str> = print_flags.unwrap().collect();

        for (_, v) in values.iter().enumerate() {
            print_options.push(ReqOption::PRINT(String::from(*v)));
        }

        cfg.options(print_options)
    } else {
        cfg
    }
}

/// NOTE: This function *can* fail if the redirect count is not valid.
/// In such a case, the program will exit gracefully with an error message.
pub fn redirect_flag<'a>(redirect_arg: Option<&'a str>, cfg: ReqConfig)
    -> ReqConfig
{
    use reqlib::FailureCode;
    use std::process;

    if redirect_arg.is_some() {
        let prov_str = redirect_arg.unwrap();
        let prov = String::from(prov_str); // for printing if error

        let redirect_count = prov_str.trim().parse();

        if redirect_count.is_err() {
            eprintln!("Could not parse redirect count! \
            Expected integer value of at least -1, got {}", prov);
            process::exit(FailureCode::ClientError.value() as i32);
        }

        let redirect_count = redirect_count.unwrap();

        cfg.option(ReqOption::FOLLOW_REDIRECTS(redirect_count))
    } else {
        cfg
    }
}

/// NOTE: This function *can* fail if the timeout couldn't be parsed.
/// In such a case, it will exit gracefully with an error message.
pub fn timeout_flag<'a>(timeout_arg: Option<&'a str>, cfg: ReqConfig)
    -> ReqConfig
{
    use std::process;
    use reqlib::FailureCode;

    if timeout_arg.is_some() {
        let timeout = timeout_arg.unwrap().trim().parse::<usize>();

        if timeout.is_err() {
            eprintln!("Could not parse the given timeout! Make sure you give a valid number value.");
            process::exit(FailureCode::ClientError.value() as i32);
        }

        cfg.timeout(timeout.unwrap())
    } else {
        cfg
    }
}

/// NOTE: This function *can* fail if the filename provided was not found.
/// In such a case, it will exit gracefully with an error message.
pub fn payload_arg<'a>(payload_arg: Option<&'a str>, encoding_arg: Option<&'a str>, cfg: ReqConfig) 
    -> ReqConfig 
{
    use std::process;
    use reqlib::FailureCode;

    if payload_arg.is_some() {
        let filename = payload_arg.unwrap();
        let payload = Payload::from_file(filename.clone());
        if payload.is_err() {
            eprintln!("Could not open {}", filename);
            process::exit(FailureCode::IOError.value() as i32);
        } else {
            let mut payload = payload.unwrap();
            
            let (payload, encoding_type) = match encoding_arg {
                Some(v) => {
                    (encode_payload(v, payload), Encoding::from_str(v))
                },
                None => (payload, None)
            };

            if encoding_type.is_some() {
                cfg.option(ReqOption::ENCODING(encoding_type.unwrap()))
                   .payload(payload)
            } else {
                cfg.payload(payload)
            }

        }
    } else {
        cfg
    }
}

pub fn header_flags<'a>(headers: Option<Values<'a>>, cfg: ReqConfig) -> ReqConfig {
    if headers.is_some() {
        let mut header_options: Vec<ReqOption> = Vec::new();

        let values: Vec<&str> = headers.unwrap().collect();
        for (i, _) in values.iter().enumerate() {
            if i % 2 == 0 {
                header_options.push(
                  ReqOption::CUSTOM_HEADER((String::from(values[i]), String::from(values[i+1]))));
            }
        }

        cfg.options(header_options)
    } else {
        cfg
    }
}

fn encode_payload(encoding_arg: &str, mut payload: Payload) -> Payload
{
    use reqlib::encode::{ self, Encoder, base64 };

    let encoder = match encoding_arg {
        "base64"        => encode::base64::Base64Encoder::new(),
        _               => {
            println!("Something went wrong selecting an encoder!");
            process::exit(FailureCode::Unknown.value() as i32);
        }
    };

    let result = encoder.encode(&mut payload);
    if result.is_err() {
        println!("Could not encode input file:\n{}", result.err().unwrap());
        process::exit(FailureCode::IOError.value() as i32);
    }

    payload
}

