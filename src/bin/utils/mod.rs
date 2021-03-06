use reqlib::{ReqConfig};
use clap::{self, Arg, ArgSettings, ArgMatches, App, AppSettings, SubCommand};

mod config_extract;
mod show;

#[cfg(test)]
mod tests;

/// Modifies the given config with all options given by in the given Args.
pub fn process_arguments(cfg: ReqConfig) -> Result<ReqConfig, clap::Error>
{
    let matches = build_app().get_matches_safe();

    match matches {
        Ok(val) => Ok(process_arg_matches(val, cfg)),
        Err(e) => Err(e)
    }
}

fn process_arg_matches<'a>(matches: ArgMatches<'a>, cfg: ReqConfig) -> ReqConfig
{
    match matches.subcommand() {
        ("get", Some(request))     => config_extract::setup_request("get", request, cfg),
        ("post", Some(request))    => config_extract::setup_request("post", request, cfg),
        ("put", Some(request))     => config_extract::setup_request("put", request, cfg),
        ("options", Some(request)) => config_extract::setup_request("options", request, cfg),
        ("head", Some(request))    => config_extract::setup_request("head", request, cfg),
        ("trace", Some(request))   => config_extract::setup_request("trace", request, cfg),
        ("connect", Some(request)) => config_extract::setup_request("connect", request, cfg),
        ("delete", Some(request))  => config_extract::setup_request("delete", request, cfg),
        ("show", Some(show))       => config_extract::setup_show_resource(show, cfg),
        ("socket", Some(sock))     => config_extract::setup_socket(sock, cfg),
        ("extract", Some(ext))     => config_extract::setup_extract(ext, cfg),
        _                          => config_extract::setup_no_subcommand(&matches, cfg)
    }
}

fn build_app<'a, 'b>() -> App<'a, 'b>
{
    App::new("Req")
        .version("1.2")
        .author("Nick Donnelly <nick@donnelly.cc>")
        .about("Quick, easy, and environment-aware HTTP client.")
        .setting(AppSettings::GlobalVersion)
        .arg(Arg::with_name("uri")
            .help("The URI to fire a request to.")
            .required(false)
            .takes_value(true)
            .env("REQ_URI")
            .value_name("URI"))
        .arg(payload_arg())
        .arg(encoding_flag())
        .arg(body_prefix_flag())
        .arg(redirect_flag())
        .arg(header_flag())
        .arg(header_file_flag())
        .arg(timeout_flag())
        .arg(print_flag())

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

        .subcommand(SubCommand::with_name("show")
                .about("Show the specified resource.")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommands(show_subcommands().into_iter()))
        
        .subcommand(SubCommand::with_name("extract")
                    .about("Extract the assets from a url (must be an html document).")
                    .arg(Arg::with_name("directory")
                         .takes_value(true)
                         .multiple(false)
                         .required(true))
                    .arg(Arg::with_name("uri")
                        .help("The URI to fire a request to.")
                        .required(false)
                        .takes_value(true)
                        .env("REQ_URI")
                        .value_name("URI")))

        .subcommand(SubCommand::with_name("socket")
            .about("Launch a socket on the given port to read incoming requests easily.")
            .arg(Arg::with_name("port")
                 .takes_value(true)
                 .default_value("8000")
                 .multiple(false)
                 .value_name("PORT")
                 .env("REQ_SOCKET_PORT"))
            .arg(Arg::with_name("response-code")
                .help("The response code (as a number) you would like to send all requests.")
                .takes_value(true).default_value("200")
                .long("response-code").short("c")
                .multiple(false)
                .env("REQ_SOCKET_RESPONSE_CODE")
                .value_name("RESPONSE_CODE"))
            .arg(Arg::with_name("response-mode")
                .help("The mode for responses the socket gives")
                .long("response-mode").short("m")
                .default_value("talkback")
                .multiple(false)
                .env("REQ_SOCKET_RESPONSE_MODE")
                .possible_values(&["talkback", "literal"])
                .value_name("RESPONSE_MODE"))
            .arg(Arg::with_name("literal-response")
                .help("A literal string you would like the socket to respond with.")
                .long("response").short("r")
                .required_if("response-mode", "literal")
                .env("REQ_SOCKET_LITERAL_RESPONSE")
                .value_name("RESPONSE")))
}

fn show_subcommands<'a, 'b>() -> Vec<App<'a, 'b>>
{
    let mut result: Vec<App> = Vec::new();

    /* Payload Subcommand */
    result.push(SubCommand::with_name("payload")
        .about("Displays how a payload would look when attached to a request. \
                This is useful for things that will look like UTF-8 text.")
        .arg(Arg::with_name("payload")
            .takes_value(true)
            .multiple(false)
            .required(true)
            .env("REQ_PAYLOAD_FILE")
            .value_name("PAYLOAD_FILE"))
        .arg(encoding_flag())
        .arg(body_prefix_flag()));


    result.push(SubCommand::with_name("env")
        .about("Displays the current req values from the environment. \
                Prioritizes .env over other values.")
        .arg(Arg::with_name("env_variable")
            .help("Choose a single variable to see the value of by req name.")
            .possible_values(&["all", "uri", "timeout", "http_method", "max_redirects", "payload_file",
                               "body_prefix", "encoding", "socket_port", "socket_response_code",
                               "socket_response_mode", "socket_response_literal", "headers"])
            .required(false)
            .takes_value(true)
            .value_name("VARIABLE")
            .number_of_values(1)
            .multiple(false)));

    result
}

fn request_subcommand_args<'a, 'b>() -> Vec<Arg<'a, 'b>> 
{
    let mut result: Vec<Arg<'a, 'b>> = Vec::new();
    result.push(uri_arg());
    result.push(payload_arg());
    result.push(encoding_flag());
    result.push(body_prefix_flag());
    result.push(redirect_flag());
    result.push(header_flag());
    result.push(header_file_flag());
    result.push(print_flag());
    result.push(timeout_flag());

    result
}

fn header_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("header")
        .help("Specify a custom header. Overrides headers in --header-file.\
               Use the format \"Header Name\" \"Value\"")
        .short("h")
        .long("header")
        .multiple(true)
        .number_of_values(2)
        .takes_value(true)
        .value_name("HEADER")
}

fn header_file_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("header-file")
        .help("Specify multiple headers in a file (one per line, same format as --header).\
               Use the value 'none' to ignore the environment variable.")
        .short("f")
        .long("header-file")
        .multiple(false)
        .env("REQ_HEADERS")
        .value_name("FILE")
}

fn payload_arg<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("payload")
        .help("Specify a file to use as the body of the request.")
        .set(ArgSettings::CaseInsensitive)
        .short("b")
        .long("body")
        .takes_value(true)
        .multiple(false)
        .env("REQ_PAYLOAD_FILE")
        .value_name("PAYLOAD_FILE")
}

fn timeout_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("timeout")
        .help("Specify the request timeout in millseconds.")
        .short("t")
        .long("timeout")
        .env("REQ_TIMEOUT")
        .multiple(false)
        .takes_value(true)
        .value_name("TIMEOUT")
}

fn print_flag<'a, 'b>() -> Arg<'a, 'b> 
{
    Arg::with_name("print")
        .help("Explicitly decide which parts of the response to print.")
        .set(ArgSettings::CaseInsensitive)
        .short("p")
        .long("print")
        .number_of_values(1)
        .possible_values(&["body", "headers", "request-headers", "status", "response-time", "config"])
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
      .env("REQ_URI")
      .value_name("URI")
}

fn redirect_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("max-redirects")
        .help("The maximum number of redirects to follow. Set this to -1 for infinite follows.")
        .set(ArgSettings::AllowLeadingHyphen)
        .takes_value(true)
        .env("REQ_MAX_REDIRECTS")
        .short("r")
        .long("max-redirects")
        .value_name("MAX_REDIRECTS")
        .multiple(false)
        .default_value("0")
}

fn encoding_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("encoding")
        .help("Automatically encode the payload using this type.")
        .takes_value(true)
        .env("REQ_ENCODING")
        .short("e")
        .long("encoding")
        .required(false)
        .value_name("ENCODING")
        .possible_values(&["none", "base64"])
        .multiple(false)
}

fn body_prefix_flag<'a, 'b>() -> Arg<'a, 'b>
{
    Arg::with_name("body-prefix")
        .help("Append a prefix to the request body (added after encoding if encoding was applied).")
        .takes_value(true)
        .env("REQ_BODY_PREFIX")
        .long("body-prefix")
        .required(false)
        .value_name("PREFIX")
        .multiple(false)
}
