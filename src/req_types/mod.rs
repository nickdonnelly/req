use hyper::{self, Method};
use hyper::header::HeaderView;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::{ Body, Client };
use futures::Stream;
use tokio_core::reactor::Core;
use super::options::*;

use std::io::{self, Write};
use std::result;
use std::str::FromStr;

mod display;
pub mod req;
pub use self::req::*;

type Result<T> = result::Result<T, ReqError>;

#[derive(PartialEq, Debug, Clone)]
pub enum ReqContentType {
    Custom(String),
    Json, Xml,
    Png, Jpeg, Webm, Webp, Gif, Mp4, Ogg,
    Html, Css, Javascript,
    OctetStream, Zip,
    Empty
}

/// The type consumed as a payload (direct bytes that will be written).
#[derive(Debug, PartialEq)]
pub struct Payload {
    pub data: Vec<u8>,
    pub content_type: ReqContentType
}

#[derive(Debug)]
pub enum FailureCode {
    NoError,
    ClientError,
    IOError,
    Timeout
}

/// Generic error type. Exit code may be ignored if it is zero.
#[derive(Debug)]
pub struct ReqError {
    pub exit_code: FailureCode,
    pub description: &'static str
}

/// Describes the type of a response status.
#[derive(Debug, Copy, Clone)]
pub enum ReqStatusType {
    Success,
    Redirect,
    Information,
    ClientFailure,
    ServerFailure,
    UnknownType
}

/// Wrapper around Hyper's StatusCode
#[derive(Debug)]
pub struct ReqResponseStatus {
    status_int: u16,
    status_string: String,
    status_type: ReqStatusType
}

impl ReqResponseStatus {

  pub fn status_type(&self) -> ReqStatusType {
      self.status_type.clone()
  }

  pub fn as_string(&self) -> String 
  {
      String::from(self.status_string.as_str())
  }

  pub fn as_int(&self) -> u16 
  {
      self.status_int
  }
}

impl From<hyper::StatusCode> for ReqResponseStatus {
    fn from(status: hyper::StatusCode) -> Self
    {
        let mut s_type = ReqStatusType::UnknownType;
        if status.is_informational() {
            s_type = ReqStatusType::Information;
        } else if status.is_success() {
            s_type = ReqStatusType::Success;
        } else if status.is_redirection() {
            s_type = ReqStatusType::Redirect;
        } else if status.is_client_error() {
            s_type = ReqStatusType::ClientFailure;
        } else if status.is_server_error() {
            s_type = ReqStatusType::ServerFailure;
        }

        let s_int = status.as_u16();
        let s_string = format!("{}", status);

        ReqResponseStatus {
            status_int: s_int,
            status_string: s_string,
            status_type: s_type
        }
    }
}

/// Generic response type
#[derive(Debug)]
pub struct ReqResponse {
    pub body: Vec<u8>,
    pub status: ReqResponseStatus,
    pub request_headers: Vec<ReqHeader>,
    pub headers: Vec<ReqHeader>
}

#[derive(PartialEq, Debug)]
pub struct ReqHeader {
    pub name: String,
    pub value: String
}

#[derive(PartialEq, Debug)]
pub enum ReqOption {
    CUSTOM_HEADER((String, String)), 
    PRINT(String), // string key contains the resource to print
    FOLLOW_REDIRECTS(FollowRedirectInfo), // max redirect count, usize
    CUSTOM_ENV_FILE(CustomEnvFileInfo) // filepath
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
            &RequestMethod::Get     => Method::Get,
            &RequestMethod::Head    => Method::Head,
            &RequestMethod::Put     => Method::Put,
            &RequestMethod::Post    => Method::Post,
            &RequestMethod::Delete  => Method::Delete,
            &RequestMethod::Options => Method::Options,
            &RequestMethod::Patch   => Method::Patch,
            &RequestMethod::Connect => Method::Connect,
            &RequestMethod::Trace   => Method::Trace,
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
                exit_code: FailureCode::ClientError,
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
    pub options: Vec<ReqOption>,
}

#[derive(Debug)]
pub struct ReqCommandResult {
    pub to_show: Option<String>,
    pub response: Option<ReqResponse>,
}

impl ReqHeader {
    pub fn from_header_view(hv: &HeaderView) -> ReqHeader
    {
        ReqHeader {
            name: String::from(hv.name()),
            value: hv.value_string()
        }
    }
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

impl ReqResponse {
    pub fn new(
        headers: Vec<ReqHeader>, 
        status: ReqResponseStatus,
        body: Vec<u8>, 
        request_headers: Vec<ReqHeader>) 
        -> ReqResponse
    {
        ReqResponse {
            headers: headers,
            status: status,
            request_headers: request_headers,
            body: body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_request_working() {
        let pl = Payload::new(vec![1,2,3], "application/octet-stream");
        let config = ReqConfig::new()
            .command(ReqCommand::Request(RequestMethod::Get))
            .host_str("https://google.com")
            .port(443)
            .timeout(10000)
            .payload(pl);
        let req = Req::new_from_cfg(config).unwrap();
        req.run().unwrap();
    }

    #[test]
    fn test_head_request_terminates() {
        let pl = Payload::new(vec![1,2,3], "application/octet-stream");
        let config = ReqConfig::new()
            .command(ReqCommand::Request(RequestMethod::Get))
            .host_str("https://google.com")
            .port(443)
            .timeout(10000)
            .payload(pl);
        let req = Req::new_from_cfg(config).unwrap();
        req.run().unwrap();
    }

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
