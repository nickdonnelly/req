use reqlib::{ReqConfig};
use clap::{Arg, ArgSettings, ArgMatches, App, AppSettings, SubCommand};

mod config_extract;

/// Modifies the given config with all options given by in the given Args.
pub fn process_arguments(cfg: ReqConfig) -> ReqConfig
{
    let matches = build_matches();

    return match matches.subcommand() {
        ("get", Some(request))     => config_extract::setup_request("get", request, cfg),
        ("post", Some(request))    => config_extract::setup_request("post", request, cfg),
        ("put", Some(request))     => config_extract::setup_request("put", request, cfg),
        ("options", Some(request)) => config_extract::setup_request("options", request, cfg),
        ("head", Some(request))    => config_extract::setup_request("head", request, cfg),
        ("trace", Some(request))   => config_extract::setup_request("trace", request, cfg),
        ("connect", Some(request)) => config_extract::setup_request("connect", request, cfg),
        ("delete", Some(request))  => config_extract::setup_request("delete", request, cfg),
        ("show", Some(show))       => config_extract:: setup_show_resource(show, cfg),
        _ => cfg 
    };
}

fn build_matches<'a>() -> ArgMatches<'a>
{
    App::new("Req")
        .version("1.0")
        .author("Nick Donnelly <nick@donnelly.cc>")
        .about("Quick, easy, configurable HTTP client.")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequired)

        .subcommand(SubCommand::with_name("get")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("post")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("put")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("options")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("head")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("trace")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("connect")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("delete")
            .args(request_subcommand_args().as_slice()))
        .subcommand(SubCommand::with_name("patch")
            .args(request_subcommand_args().as_slice()))

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
    result.push(uri_arg());
    result.push(payload_arg());
    result.push(header_flag());
    result.push(print_flag());

    result
}

fn header_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("header")
        .help("Specify a custom header. Use the format \"Header Name\" \"Value\"")
        .short("h")
        .long("header")
        .multiple(true)
        .number_of_values(2)
        .takes_value(true)
        .value_name("HEADER")
}

fn payload_arg<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("payload")
        .help("Specify a file to use as the body of the request.")
        .set(ArgSettings::CaseInsensitive)
        .short("b")
        .long("body")
        .takes_value(true)
        .number_of_values(1)
        .value_name("PAYLOAD_FILE")
}

fn print_flag<'a, 'b>() -> Arg<'a, 'b> 
{
    Arg::with_name("print")
        .help("Explicitly decide which parts of the response to print.")
        .set(ArgSettings::CaseInsensitive)
        .short("p")
        .long("print")
        .number_of_values(1)
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
