use hyper::Request;
use hyper::Uri;
use hyper::http::request::Builder;
use hyper::client::Client;
use tokio_core::reactor::Timeout;
use futures::future::{ Future, Either };
use futures::stream::Stream;
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
                            let scheme = hyper_uri.scheme_str().unwrap_or("http://");
                            let authority = hyper_uri.authority().unwrap().as_str();

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
            ExtractAssets(_) => self.run_extract_assets()
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
    fn run_socket(self, port: u16) -> Result<ReqCommandResult>
    {
        use quicksock::{ QuickSocket, SocketType };

        let options: Vec<ReqOption> = Req::clone_options(self.cfg.options.as_slice());
        let mut socket_type = SocketType::Talkback;
        
        for option in options {
            match option {
                ReqOption::LiteralSocket(literal) => { socket_type = SocketType::Literal(literal) },
                _ => {}
            }
        }

        let qs = QuickSocket::new(socket_type);
        println!("Starting socket on  127.0.0.1:{}", &port);
        qs.start(port);
        println!("Started socket on  127.0.0.1:{}", &port);
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
    fn run_extract_assets(self) -> Result<ReqCommandResult>
    {
        use super::super::asset_extract;
        use rand::Rng;
        use std::fs::{ self };
        use std::io::Write;

        let host = self.cfg.host.clone().unwrap();
        let directory =  if let ReqCommand::ExtractAssets(d) = self.cfg.command {
            d.clone()
        } else { 
            String::from(".")
        };

        let core = Core::new();
        if core.is_err() {
            return Err(ReqError { 
                exit_code: FailureCode::IOError, 
                description: "Unable to fetch event loop."});
        }
        let config = ReqConfig::new().global_defaults()
                .command(ReqCommand::Request(RequestMethod::Get))
                .host(host.clone());
                
        let initial_request = Req::new_from_cfg(config).unwrap();
        let response = initial_request.run();

        if response.is_err() {
            return Err(response.err().unwrap());
        }

        let response_body = response.unwrap().response.unwrap().body;
        let extractions = asset_extract::extract_resource_list(&response_body);
        fs::create_dir_all(directory.clone());

        for extraction in extractions.unwrap() {
            let uri = extraction.uri().unwrap();
            let uri = if uri.starts_with("http") {
                uri
            } else {
                let mut x = host.clone();
                x += "/";
                x += &uri;
                x
            };

            let config = ReqConfig::new().global_defaults()
                .command(ReqCommand::Request(RequestMethod::Get))
                .host(uri);
            let res = Req::new_from_cfg(config).unwrap().run().unwrap().response.unwrap();
            let mut saveLoc = directory.clone() + "/";
            for header in res.headers {
                if header.name == "Content-Disposition" {
                    saveLoc += header.name.as_str();
                    break;
                }
            }

            if saveLoc.ends_with("/") {
                let random_chars: String = rand::thread_rng()
                    .gen_ascii_chars()
                    .take(10)
                    .collect();
                saveLoc += random_chars.as_str();
            }

            let mut file = fs::File::create(saveLoc).unwrap();
            file.write_all(&res.body).unwrap();
        }
        
        Ok(ReqCommandResult::new_stub())
    }

    #[inline(always)]
    fn run_request(self) -> Result<ReqCommandResult> {
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
 
        let mut request = Request::builder();
        let mut request = request.uri(uri).method(meth);
        let mut request_headers: Vec<ReqHeader> = Vec::new();

        // We must add the headers afterwards in case they want to override the headers
        // set by add_payload.
        let mut request = Req::add_request_headers(&mut request, &mut custom_headers);
        Req::copy_request_headers(&mut request, &mut request_headers);
        let request = Req::add_payload(&mut request, payload);
        
        let mut core = core.unwrap();
        let handle = core.handle();
        let https = HttpsConnector::new();
        let client = Client::builder()
            .keep_alive(false)
            .set_host(true)
            .build::<_, hyper::Body>(https);

        let timeout = Timeout::new(Duration::from_millis(timeout as u64), &handle).unwrap();
        let work = client.request(request.unwrap()).select2(timeout)
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
            response.headers().iter().for_each(|(key, value)| {
                let key = key.as_str();
                let value = value.to_str().unwrap();
                response_headers.push(ReqHeader::new(key, value));
            });

            // == Extract the response status ==
            let response_status = ReqResponseStatus::from(response.status());

            // == Extract the body ==
            // NOTE: This must be done with the reactor core!
            // Without it this will block indefinitely if the stream contains more information
            // than can be retrieved with the initial poll.

            let raw_response_body = core.run(response.into_body().concat2());
            if raw_response_body.is_err() {
                return Err(ReqError {
                    exit_code: FailureCode::IOError,
                    description: "Could not read from body stream."
                });
            }

            //let raw_response_body = response.into_body();
            let data  = raw_response_body.unwrap();
            response_body.extend_from_slice(&(*data));

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

    fn match_hyper_error(err: Option<hyper::Error>) -> ReqError
    {
        if err.is_none() {
            ReqError {
                exit_code: FailureCode::ClientError,
                description: "Silent failure! No error returned!"
            }
        } else {
            let err = err.unwrap();
            if err.is_parse() {
                return ReqError {
                    exit_code: FailureCode::IOError,
                    description: "Hyper parse error."
                };
            }

            if err.is_closed() || err.is_incomplete_message() {
                return ReqError {
                    exit_code: FailureCode::IOError,
                    description: "Connection reset."
                };
            }

            if err.is_canceled() {
                return ReqError {
                    exit_code: FailureCode::IOError,
                    description: "Connection closed."
                };
            }

            if err.is_user() {
                return ReqError {
                    exit_code: FailureCode::ClientError,
                    description: "Unknown error."
                };
            }

            /*
            let desc = if let Some(e) = err.into_cause() {
                let s = format!("{}", e);
                &s
            } else {
                "Unknown error."
            };
            */

            ReqError {
                exit_code: FailureCode::ClientError,
                description: "Unknown error."
            }
        }
    }

    fn add_payload<'a>(req: &'a mut &'a mut Builder, payload: Payload)
        -> std::result::Result<hyper::Request<hyper::Body>, hyper::http::Error> 
    {
        let ctt = payload.content_type().clone();
        if let ReqContentType::Empty = ctt {
            //req.headers_mut().insert("Content-Length", HeaderValue::from_str("0").unwrap());
            req.header("Content-Length", 0);
        }

        req.header("Content-Type", payload.content_type_str());
        req.header("Content-Length", payload.data.len());
        /*
        req.headers_mut().insert("Content-Type", 
                                 HeaderValue::from_str(payload.content_type_str()).unwrap());
        req.headers_mut().insert("Content-Length", 
                                 HeaderValue::from_str(
                                     format!("{}", payload.data.len()).as_str()).unwrap());
         */
        let data = payload.data();

        return req.body(hyper::Body::from(data));
    }

    /// Converts and copies all of the headers from the request to the given vector.
    /// Used to return the headers from the request with the `ReqCommandResult`.
    fn copy_request_headers(
        req: &mut &mut Builder,
        copy_to: &mut Vec<ReqHeader>)
    {
        req.headers_ref().unwrap().iter().for_each(|header|{
            let name = String::from(header.0.as_str());
            let value = String::from(header.1.to_str().unwrap());
            let r_header = ReqHeader{
                name: name,
                value: value
            };

            copy_to.push(r_header);
        });
    }

    /// Adds all headers in `headers` to the request.
    fn add_request_headers<'a>(req: &'a mut &'a mut Builder,
                           headers: &mut Vec<(String, String)>)
        -> &'a mut &'a mut Builder
    {
        use hyper::http::header::HeaderValue;
        use hyper::http::header::HeaderName;
        headers.iter().for_each(|header|{
            let headername = HeaderName::from_lowercase(header.0.as_bytes()); 
            req.headers_mut().unwrap().insert(headername.unwrap(),
                                              HeaderValue::from_str(header.1.clone().as_str()).unwrap());
            //req.headers_mut().insert(header.0.clone().as_str(),
                                     //HeaderValue::from_str(header.1.clone()).unwrap());
        });

        return req;
    }
}
