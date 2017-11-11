use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::{ Body, Client };
use tokio_core::reactor::Core;

use std::result;

type Result<T> = result::Result<T, ReqError>;

pub struct Payload {
    data: Vec<u8>,
    content_type: String // maybe use enum type
}

pub struct ReqError {
    pub exit_code: u16,
    pub description: &'static str
}

#[derive(PartialEq)]
pub enum ReqOption {
    FOLLOW_REDIRECTS(usize), // max redirect count
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
pub enum ReqResource {
    Help(&'static str),
    //HelpEmoji(&'static str),
    Env,
    EnvVar(String)
}

/// The possible types of HTTP requests.
pub enum RequestMethod {
    Get, Head, Put, Post, Delete, Options,
    Patch, Connect, Trace
}

/// All possible commands.
pub enum ReqCommand {
    Request(RequestMethod),
    Show(ReqResource)
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