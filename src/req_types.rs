use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::{ Body, Client };
use tokio_core::reactor::Core;
use super::options::*;

use std::result;
use std::str::FromStr;

type Result<T> = result::Result<T, ReqError>;

// TODO
pub const HELP_STR: &str = "Help String goes here";

/// The type consumed as a payload (direct bytes that will be written).
pub struct Payload {
    data: Vec<u8>,
    content_type: String // maybe use enum type
}

/// Generic error type. Exit code may be ignored if it is zero.
pub struct ReqError {
    pub exit_code: u16, // TODO: Make this enum type
    pub description: &'static str
}

#[derive(PartialEq, Debug)]
pub enum ReqOption {
    FOLLOW_REDIRECTS(FollowRedirectInfo), // max redirect count, usize
    CUSTOM_ENV_FILE(CustomEnvFileInfo) // filepath
}

/// Master struct for the actual requesting
pub struct Req {
    pub cfg: ReqConfig,
    client: Option<Client<
        HttpsConnector<HttpConnector>, Body>>
}

/// The trait that represents a resource of req.
/// This could be something like the environment, a
/// specific environment variable, or the help page.
#[derive(PartialEq, Debug)]
pub enum ReqResource {
    Help(&'static str),
    //HelpEmoji(&'static str),
    Env,
    EnvVar(String)
}

/// The possible types of HTTP requests.
#[derive(PartialEq, Debug)]
pub enum RequestMethod {
    Get, Head, Put, Post, Delete,
    Options, Patch, Connect, Trace
}

impl FromStr for RequestMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "get"       => Ok(RequestMethod::Get),
            "head"      => Ok(RequestMethod::Head),
            "put"       => Ok(RequestMethod::Put),
            "post"      => Ok(RequestMethod::Post),
            "delete"    => Ok(RequestMethod::Delete),
            "options"   => Ok(RequestMethod::Options),
            "patch"     => Ok(RequestMethod::Patch),
            "connect"   => Ok(RequestMethod::Connect),
            "trace"     => Ok(RequestMethod::Trace),
            _           => Err("not a valid http method")
        }
    }
}

/// All possible commands.
#[derive(PartialEq, Debug)]
pub enum ReqCommand {
    Request(RequestMethod),
    CleanEnvironment,
    Show(ReqResource)
}

impl FromStr for ReqCommand {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        // Match the request types first, this is easy;
        let request_type = RequestMethod::from_str(s);

        if(request_type.is_ok()) {
            Ok(ReqCommand::Request(request_type.unwrap()))
        } else {
            match s.trim().to_lowercase().as_str() {
                "help"      => Ok(ReqCommand::Show(
                                     ReqResource::Help(HELP_STR))),
                "cleanenv"  => Ok(ReqCommand::CleanEnvironment),
                _           => Err("unknown command")
            }
        }
    }
}

/// The master config type.
pub struct ReqConfig {
    pub command: ReqCommand,
    pub host: Option<String>,
    pub port: Option<usize>,
    pub timeout: Option<usize>,
    pub payload: Option<Payload>,
    pub options: Option<Vec<ReqOption>>,
}

/// Fetch a new Req client for a given config.
pub fn new_client(cfg: ReqConfig) 
    -> Result<Req> {
    let mut core = Core::new();
    if core.is_err() {
        return Err(ReqError { 
            exit_code: 2, 
            description: "Unable to fetch event loop."});
    }
    let handle = core.unwrap().handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    
    Ok(Req{
        cfg: cfg,
        client: Some(client)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn req_method_from_str_1() {
        let method = "GET";
        let result = RequestMethod::from_str(method);

        assert_eq!(result.unwrap(), RequestMethod::Get);
    }

    #[test]
    fn req_method_from_str_2() {
        let method = "OPTIONS";
        let result = RequestMethod::from_str(method);

        assert_eq!(result.unwrap(), RequestMethod::Options);
    }

    #[test]
    #[should_panic]
    fn req_method_from_str_panic() {
        let method = "NOTVALIDTYPE";
        let result = RequestMethod::from_str(method);

        result.unwrap();
    }

    #[test]
    fn req_command_from_string_1() {
        let commandstr = "get";
        let result = ReqCommand::from_str(commandstr);

        assert_eq!(result.unwrap(), ReqCommand::Request(RequestMethod::Get));
    }

    #[test]
    fn req_command_from_string_2() {
        let commandstr = "options";
        let result = ReqCommand::from_str(commandstr);

        assert_eq!(result.unwrap(), 
            ReqCommand::Request(RequestMethod::Options));
    }

    #[test]
    fn req_command_from_string_3() {
        let commandstr = "help";
        let result = ReqCommand::from_str(commandstr);

        assert_eq!(result.unwrap(), 
            ReqCommand::Show(ReqResource::Help(HELP_STR)));
    }

    #[test]
    #[should_panic]
    fn req_command_from_string_panic() {
        let commandstr = "notacommand";
        let result = ReqCommand::from_str(commandstr);
        result.unwrap();
    }
}
