use reqlib::{ReqConfig, ReqOption, RequestMethod, ReqCommand, Payload};
use clap::{Values, ArgMatches};
use std::env;
use std::str::FromStr;

pub fn setup_show_resource<'a>(show_matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    cfg
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
    let cfg = payload_arg(request_matches.value_of("payload"), cfg);

    // Add the URI
    if let Some(uri) = request_matches.value_of("uri") {
        cfg.host_str(uri)
    } else {
        cfg
    }
}

pub fn setup_no_subcommand<'a>(matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    // Add any flags
    let cfg = header_flags(matches.values_of("header"), cfg);
    let cfg = print_flags(matches.values_of("print"), cfg);
    let cfg = timeout_flag(matches.value_of("timeout"), cfg);
    let cfg = payload_arg(matches.value_of("payload"), cfg);

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

pub fn timeout_flag<'a>(timeout_arg: Option<&'a str>, cfg: ReqConfig)
    -> ReqConfig
{
    if timeout_arg.is_some() {
        let timeout = timeout_arg.unwrap().trim().parse().unwrap_or(30000);
        cfg.timeout(timeout)
    } else {
        cfg
    }
}

/// NOTE: This function *can* fail if the filename provided was not found.
/// In such a case, it will exit gracefully with an error message.
pub fn payload_arg<'a>(payload_arg: Option<&'a str>, cfg: ReqConfig) 
    -> ReqConfig 
{
    use std::process;
    use reqlib::FailureCode;

    if payload_arg.is_some() {
        let filename = payload_arg.unwrap();
        let payload = Payload::from_file(filename.clone());
        if payload.is_err() {
            println!("Could not open {}", filename);
            process::exit(FailureCode::IOError.value() as i32);
        } else {
            cfg.payload(payload.unwrap())
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
