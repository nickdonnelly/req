use reqlib::{ReqConfig, RequestMethod, ReqCommand};
use clap::{Arg, ArgSettings, ArgMatches, App, AppSettings, SubCommand};
use std::str::FromStr;

/// Modifies the given config with all options given by in the given Args.
pub fn process_arguments(cfg: ReqConfig) -> ReqConfig
{
    let matches = build_matches();

    return match matches.subcommand() {
        ("get", Some(request)) => setup_request("get", request, cfg),
        ("post", Some(request)) => setup_request("post", request, cfg),
        ("put", Some(request)) => setup_request("put", request, cfg),
        ("options", Some(request)) => setup_request("options", request, cfg),
        ("head", Some(request)) => setup_request("head", request, cfg),
        ("trace", Some(request)) => setup_request("trace", request, cfg),
        ("connect", Some(request)) => setup_request("connect", request, cfg),
        ("delete", Some(request)) => setup_request("delete", request, cfg),
        ("show", Some(show)) => setup_show_resource(show, cfg),
        _ => cfg 
    };


    //println!("{:?}", matches.subcommand_matches("get").unwrap().value_of("uri").unwrap());
}

fn setup_request<'a>(meth: &str, request_matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    // Add the command
    let req_method = RequestMethod::from_str(meth);
    let cfg = if req_method.is_ok() {
        cfg.command(ReqCommand::Request(req_method.unwrap()))
    } else {
        cfg
    };

    // Add the URI
    if let Some(uri) = request_matches.value_of("uri") {
        cfg.host_str(uri)
    } else {
        cfg
    }
}

fn setup_show_resource<'a>(show_matches: &ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    cfg
}

fn build_matches<'a>() -> ArgMatches<'a>
{
    App::new("Req")
        .version("1.0")
        .author("Nick Donnelly <nick@donnelly.cc>")
        .about("Quick, easy, configurable HTTP client.")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("get").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("post").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("put").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("options").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("head").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("trace").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("connect").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("delete").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("patch").args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("show").about("Show the specified resource.")
                    .arg(Arg::with_name("resource")
                    .help("The resource you wish to show")
                    .takes_value(true)
                    .value_name("RESOURCE")))
        .arg(Arg::with_name("output-file")
             .help("Specify a file to dump the output to.")
             .short("f")
             .long("output-file")
             .takes_value(true)
             .value_name("FILENAME"))
        .get_matches()
}

fn request_subcommand_args<'a, 'b>() -> Vec<Arg<'a, 'b>> 
{
    let mut result: Vec<Arg<'a, 'b>> = Vec::new();
    result.push(print_arg());
    result.push(uri_arg());
    result.push(payload_arg());

    result
}

fn payload_arg<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("payload")
        .help("Specify a file to use as the body of the request.")
        .set(ArgSettings::CaseInsensitive)
        .short("b")
        .long("body")
        .takes_value(true)
        .value_name("PAYLOAD_FILE")
}

fn print_arg<'a, 'b>() -> Arg<'a, 'b> 
{
    Arg::with_name("print")
        .help("Explicitly decide which parts of the response to print.")
        .set(ArgSettings::CaseInsensitive)
        .short("p")
        .long("print")
        .multiple(true)
        .takes_value(true)
        .value_name("PARTIAL")
}

fn uri_arg<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("uri")
      .help("The URI to fire a request to.")
      .required(true)
      .takes_value(true)
      .value_name("URI")
}
