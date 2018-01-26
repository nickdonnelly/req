use hyper::client::Request;
use hyper::error;
use hyper::Uri;
use tokio_core::reactor::Timeout;
use futures::future::Either;
use super::*;
use std::time::Duration;

/// Master struct for the actual requesting
#[derive(Debug)]
pub struct Req {
    pub cfg: ReqConfig,
}

/// Fetch a new Req client for a given config.
impl Req {

    /// Get a new Req instance from a config.
    pub fn new_from_cfg(cfg: ReqConfig) 
        -> Result<Req> {
      
        Ok(Req{
            cfg: cfg,
        })
    }

    pub fn get_config(&self) -> &ReqConfig {
        return &self.cfg;
    }


    /// Runs the current config. Can only be safely run once with each
    /// config. Also, this does not validate your configuration, it will
    /// assume it is correct.
    pub fn run(self) -> Result<ReqCommandResult> {
        use ReqCommand::*;
        let fres = match self.cfg.command {
            Request(_) => { 
                if self.cfg.should_redirect() {
                    let res = self.run_request();
                    if res.is_err() {
                        res
                    } else {
                        let res = res.unwrap();
                        let config = res.from_config;
                        let response = res.response.unwrap();
                        let redirect_place = response.get_location_header_val();

                        if redirect_place.is_none() || 
                            !(response.status.status_type() == ReqStatusType::Redirect) 
                        { 
                            let r = Req::new_from_cfg(config).unwrap();
                            let cr = ReqCommandResult::new_response(response, r);
                            return Ok(cr);
                        }

                        let redirect_place = redirect_place.unwrap();
                        let redirect_place = if redirect_place.starts_with("/") {
                            let hyper_uri = config.host.clone().unwrap().parse::<Uri>().unwrap();
                            let scheme = hyper_uri.scheme().unwrap_or("http://");
                            let authority = hyper_uri.authority().unwrap();

                            format!("{}{}{}", scheme, authority, redirect_place)
                        } else {
                            redirect_place
                        };

                        let mut config = config.host(redirect_place);
                        config.reduce_redirect_count();

                        let new_req = Req::new_from_cfg(config); // run with exact same config on new uri

                        if new_req.is_err() {
                            Err(new_req.err().unwrap())
                        } else {
                            new_req.unwrap().run()
                        }
                    }

                } else {
                    self.run_request()
                }
            },
            Show(_) => self.run_show(),
            Socket(port) => self.run_socket(port),
            CleanEnvironment => self.clean_env(),
        };
        if fres.is_err() {
            return Err(fres.err().unwrap());
        }
        fres
    }
    
    fn clone_options(opts: &[ReqOption]) -> Vec<ReqOption>
    {
        let v = opts.to_vec();
        v
    }

    fn env_string() -> String
    {
        use std::env;
        use std::fmt::Write;

        let mut s = String::new();
        for (k, v) in env::vars() {
            if k.starts_with("REQ_") {
                write!(&mut s, "{}={}\n", k, v).unwrap();
            }
        }
        s
    }

    #[inline(always)]
    fn run_socket(self, port: usize) -> Result<ReqCommandResult>
    {
        use hyper::StatusCode;
        use quicksock::{ QuickSocket, SocketType };

        let options: Vec<ReqOption> = Req::clone_options(self.cfg.options.as_slice());
        let mut sc = StatusCode::Ok;
        let mut socket_type = SocketType::Talkback;
        
        for option in options {
            match option {
                ReqOption::CustomSocketResponseCode(c) => { sc = c },
                ReqOption::LiteralSocket(literal) => { socket_type = SocketType::Literal(literal) },
                _ => {}
            }
        }

        let qs = QuickSocket::new(socket_type);
        println!("Starting socket on  127.0.0.1:{}", &port);
        qs.start(port, sc);
        Ok(ReqCommandResult::new_stub()) // we never get here.
    }

    #[inline(always)]
    fn clean_env(self) -> Result<ReqCommandResult> {
        Ok(ReqCommandResult::new_stub())
    }

    #[inline(always)]
    fn run_show(self) -> Result<ReqCommandResult> {
        use std::env;

        let to_show = match &self.cfg.command {
            &ReqCommand::Show(ReqResource::Body(ref b)) => format!("{}", b),
            &ReqCommand::Show(ReqResource::EnvVar(ref var)) => {
                match env::var(var) {
                    Ok(valstr) => format!("{}={}", var, valstr),
                    Err(_) => format!("{}=<no value>", var)
                }
            },
            &ReqCommand::Show(ReqResource::Env) => Req::env_string(),
            _ => String::new()
        };

        Ok(ReqCommandResult::new_to_show(to_show, self))
    }

    #[inline(always)]
    fn run_request(self) -> Result<ReqCommandResult> {
        use hyper::{ Request, Uri };
        use futures::{ Future, Stream };
 
        let err = self.validate_request_config();
        if err.is_some() {
            return Err(err.unwrap());
        }
       
        let host_str = self.cfg.host.clone().unwrap();
        let meth = self.cfg.command.as_method().unwrap();
        let timeout = self.cfg.timeout.clone().unwrap();
        let payload = self.cfg.payload.clone();
        let uri = Uri::from_str(host_str.as_str()).unwrap();
        let options: Vec<ReqOption> = Req::clone_options(self.cfg.options.as_slice());
        let mut custom_headers: Vec<(String, String)> = Vec::new();

        let payload = if payload.is_some() {
            payload.unwrap()
        } else {
            Payload::empty()
        };

        options.iter().for_each(|option| {
            match option {
                &ReqOption::CustomHeader(ref v) => custom_headers.push(v.clone()),
                _ => {}
            };
        });

        
        let core = Core::new();
        if core.is_err() {
            return Err(ReqError { 
                exit_code: FailureCode::IOError, 
                description: "Unable to fetch event loop."});
        }
 
        let mut request = Request::new(meth, uri); 
        let mut request_headers: Vec<ReqHeader> = Vec::new();

        Req::add_payload(&mut request, payload);
        // We must add the headers afterwards in case they want to override the headers
        // set by add_payload.
        Req::add_request_headers(&mut request, &mut custom_headers);
        Req::copy_request_headers(&mut request, &mut request_headers);
        
        let mut core = core.unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .keep_alive(false)
            .build(&handle);

        let timeout = Timeout::new(Duration::from_millis(timeout as u64), &handle).unwrap();
        let work = client.request(request).select2(timeout)
            .then(|res| match res{
                Ok(Either::A((res, _))) => Ok(res), // request sucecss
                Ok(Either::B((_timeout, _))) => Err(ReqError {
                        exit_code: FailureCode::Timeout,
                        description: "The request timed out."
                    }),
                Err(Either::A((res_err, _))) => 
                    Err(Req::match_hyper_error(Some(res_err))), // request error
                Err(Either::B((_timeout_err, _))) => Err(ReqError{
                        exit_code: FailureCode::IOError,
                        description: "Something went wrong measuring the timeout...Doh!"
                    })
            });

        let response = core.run(work);

        if response.is_ok() {
            let response = response.unwrap();

            let mut response_body = Vec::new();
            let mut response_headers: Vec<ReqHeader> = Vec::new();
            
            // == Extract the headers ==
            response.headers().iter().for_each(|header_view| {
                response_headers.push(ReqHeader::from_header_view(&header_view));
            });

            // == Extract the response status ==
            let response_status = ReqResponseStatus::from(response.status());

            // == Extract the body ==
            // NOTE: This must be done with the reactor core!
            // Without it this will block indefinitely if the stream contains more information
            // than can be retrieved with the initial poll.
            let raw_response_body = core.run(response.body().concat2());

            if raw_response_body.is_err() {
                return Err(ReqError {
                    exit_code: FailureCode::IOError,
                    description: "Could not read from body stream."
                });
            }

            let raw_response_body = raw_response_body.unwrap();
            response_body.extend_from_slice(&*raw_response_body);

            let req_response = ReqResponse::new(
                response_headers, 
                response_status,
                response_body, 
                request_headers);

            Ok(ReqCommandResult::new_response(req_response, self))
        } else {
            let response_error = response.err();
            if response_error.is_none() {
                Err(ReqError {
                    exit_code: FailureCode::ClientError,
                    description: "Silent failure! No error returned!"
                })
            } else {
                Err(response_error.unwrap())
            }
        }
    }

    fn validate_request_config(&self) -> Option<ReqError>
    {
        if self.cfg.host.is_none() {
            return Some(ReqError{
                exit_code: FailureCode::ClientError, description: "No remote host given."
            });
        }
        None
    }

    fn match_hyper_error(err: Option<error::Error>) -> ReqError
    {
        use self::error::Error;
        use std::io::ErrorKind;

        if err.is_none() {
            ReqError {
                exit_code: FailureCode::ClientError,
                description: "Silent failure! No error returned!"
            }
        } else {
            match err.unwrap() {
              Error::Header => ReqError { 
                  exit_code: FailureCode::IOError, 
                  description: "Invalid header." 
              },
              Error::Io(e) => {
                  //let erstr = format!("{}", e.description());
                  let estr =  match e.kind() {
                      ErrorKind::ConnectionRefused => "Connection refused.",
                      ErrorKind::ConnectionReset => "Connection reset.",
                      ErrorKind::ConnectionAborted => "Connection aborted.",
                      ErrorKind::NotConnected => "Connection failed.",
                      ErrorKind::TimedOut => "IO timed out.",
                      ErrorKind::PermissionDenied => "Permission denied.",
                      ErrorKind::Interrupted => "Interrupted.",
                      _ => "Unknown error."
                  };
                  ReqError { 
                      exit_code: FailureCode::IOError, 
                      description: estr
                  }
              },
              Error::Timeout => {
                  ReqError {
                      exit_code: FailureCode::Timeout,
                      description: "Connection timed out."
                  }
              },
              Error::Status => {
                  ReqError {
                      exit_code: FailureCode::IOError,
                      description: "Bad HTTP status."
                  }
              },
              _ => ReqError { exit_code: FailureCode::ClientError, description: "Unknown error." }
            }
        }
    }

    fn add_payload(
        req: &mut Request,
        payload: Payload)
    {
        let ctt = payload.content_type().clone();
        if let ReqContentType::Empty = ctt {
            req.headers_mut().set_raw("Content-Length", "0");
        }

        req.headers_mut().set_raw("Content-Type", payload.content_type_str());
        req.headers_mut().set_raw("Content-Length", format!("{}", payload.data.len()));
        let data = payload.data();
        req.set_body(data);
    }

    /// Converts and copies all of the headers from the request to the given vector.
    /// Used to return the headers from the request with the `ReqCommandResult`.
    fn copy_request_headers(
        req: &mut Request,
        copy_to: &mut Vec<ReqHeader>)
    {
        req.headers().iter().for_each(|header|{
            let name = String::from(header.name());
            let value = header.value_string();
            let r_header = ReqHeader{
                name: name,
                value: value
            };

            copy_to.push(r_header);
        });

    }


    /// Adds all headers in `headers` to the request.
    fn add_request_headers(
        req: &mut Request, 
        headers: &mut Vec<(String, String)>)
    {
        headers.iter().for_each(|header|{
            req.headers_mut().set_raw(header.0.clone(), header.1.clone());
        });
    }
}
