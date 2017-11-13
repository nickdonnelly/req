use hyper::Method;
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
#[derive(Debug, PartialEq)]
pub struct Payload {
    pub data: Vec<u8>,
    pub content_type: String // maybe use enum type
}

/// Generic error type. Exit code may be ignored if it is zero.
#[derive(Debug)]
pub struct ReqError {
    pub exit_code: u16, // TODO: Make this enum type
    pub description: &'static str
}

/// Generic response type
// TODO: add headers and other bits here.
pub struct ReqResponse {
    pub body: Vec<u8>,
}

#[derive(PartialEq, Debug)]
pub enum ReqOption {
    FOLLOW_REDIRECTS(FollowRedirectInfo), // max redirect count, usize
    CUSTOM_ENV_FILE(CustomEnvFileInfo) // filepath
}

/// Master struct for the actual requesting
pub struct Req {
    cfg: ReqConfig,
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

impl RequestMethod {
    pub fn as_hyper_method(&self) -> Method {
        match self {
            Get     => Method::Get,
            Head    => Method::Head,
            Put     => Method::Put,
            Post    => Method::Post,
            Delete  => Method::Delete,
            Options => Method::Options,
            Patch   => Method::Patch,
            Connect => Method::Connect,
            Trace   => Method::Trace,
        }
    }
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

impl ReqCommand {
    pub fn as_method(&self) -> Result<Method> {
        match self {
            &ReqCommand::Request(ref meth) => Ok(meth.as_hyper_method()),
            _ => Err(ReqError { 
                exit_code: 1,
                description: "Bad command type for request."
            })
        }
    }
}

impl FromStr for ReqCommand {
    type Err = &'static str;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        // Match the request types first, this is easy;
        let request_type = RequestMethod::from_str(s);

        if request_type.is_ok() {
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
#[derive(Debug, PartialEq)]
pub struct ReqConfig {
    pub command: ReqCommand,
    pub host: Option<String>,
    pub port: Option<usize>,
    pub timeout: Option<usize>,
    pub payload: Option<Payload>,
    pub options: Option<Vec<ReqOption>>,
}

pub struct ReqCommandResult {
    pub to_show: Option<String>,
    pub response: Option<ReqResponse>,
}

impl ReqCommandResult {
    /// Get an empty ReqCommandResult
    pub fn new_stub() -> ReqCommandResult {
        ReqCommandResult {
            to_show: None,
            response: None
        }
    }

    /// Get a ReqCommandResult for a request command.
    pub fn new_response(res: ReqResponse) -> ReqCommandResult {
        ReqCommandResult {
            to_show: None,
            response: Some(res)
        }
    }
}

/// Fetch a new Req client for a given config.
impl Req {
    /// Get a new Req instance from a config.
    pub fn new_from_cfg(cfg: ReqConfig) 
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

    pub fn get_config(&self) -> &ReqConfig {
        return &self.cfg;
    }


    /// Runs the current config. Can only be safely run once with each
    /// config.
    pub fn run(&self) -> Result<ReqCommandResult> {
        use ReqCommand::*;
        let res = match self.cfg.command {
            Request(_) => self.run_request(),
            Show(_) => self.run_show(),
            CleanEnvironment => self.clean_env(),
        };
        if res.is_err() {
            return Err(res.err().unwrap());
        }

        let res = self.handle_result(res.unwrap());

        if res.is_ok() {
            Ok(res.unwrap())
        } else {
            Err(res.err().unwrap())
        }

    }

    /// Handles based on command type and options.
    // TODO: Consider moving this to impl ReqCommandResult
    fn handle_result(&self, 
        /* mut */res: ReqCommandResult) 
        -> Result<ReqCommandResult>
    {
        Ok(res)
    }

    fn clean_env(&self) -> Result<ReqCommandResult> {
        Ok(ReqCommandResult::new_stub())
    }

    fn run_show(&self) -> Result<ReqCommandResult> {
        Ok(ReqCommandResult::new_stub())
    }

    fn run_request(&self) -> Result<ReqCommandResult> {
        use hyper::{ Request, Uri };

        let err: Option<ReqError> = self.validate_config_request();
        if err.is_some() {
            return Err(err.unwrap());
        }

        let mut request: Request;
        let host_str = self.cfg.host.clone().unwrap();
        let meth = self.cfg.command.as_method().unwrap();
        let uri = Uri::from_str(host_str.as_str());

        Ok(ReqCommandResult::new_stub())
    }

    /// Checks URI, port number, etc. for validity.
    // TODO
    fn validate_config_request(&self) -> Option<ReqError>{
        if self.cfg.host.is_none() {
            return Some(
                ReqError{ exit_code: 1, description: "No remote host given." }
            );
        }
        None
    }
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
