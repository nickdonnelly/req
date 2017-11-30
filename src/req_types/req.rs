use super::*;
use std::time::Duration;
use std::ops::Deref;

/// Master struct for the actual requesting
pub struct Req {
    cfg: ReqConfig,
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
        res
    }


    fn clean_env(&self) -> Result<ReqCommandResult> {
        Ok(ReqCommandResult::new_stub())
    }

    fn run_show(&self) -> Result<ReqCommandResult> {
        Ok(ReqCommandResult::new_stub())
    }

    fn run_request(&self) -> Result<ReqCommandResult> {
        use hyper::{ Request, Uri };
        use futures::Future;

        let err: Option<ReqError> = self.validate_config_request();
        if err.is_some() {
            return Err(err.unwrap());
        }

        let host_str = self.cfg.host.clone().unwrap();
        let meth = self.cfg.command.as_method().unwrap();
        let timeout = self.cfg.timeout.clone();
        let uri = Uri::from_str(host_str.as_str()).unwrap();

        let mut core = Core::new();
        if core.is_err() {
            return Err(ReqError { 
                exit_code: FailureCode::IOError, 
                description: "Unable to fetch event loop."});
        }
 
        let request = Request::new(meth, uri); 
        let mut core = core.unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .keep_alive_timeout(self.resolve_timeout(timeout))
            .build(&handle);
 
        let work = client.request(request).map(|res| {
            let mut req_body = Vec::new();
            let mut req_headers: Vec<ReqHeader> = Vec::new();
            res.headers().iter().for_each(|hv| {
                 req_headers.push(ReqHeader::from_header_view(&hv));
            });
            let body = res.body().collect().wait();
            if body.is_err() {
                return Err(ReqError {
                    exit_code: FailureCode::IOError,
                    description: "Error reading body."
                });
            }

            let body = body.unwrap();
            body.iter().for_each(|chunk| {
                chunk.iter().for_each(|val| {
                    write!(&mut req_body, "{}", val);
                });
            });


            let req_res = ReqResponse::new(req_headers, req_body);
            Ok(ReqCommandResult::new_stub())
        });
        let core_result = core.run(work);

        Ok(ReqCommandResult::new_stub())
    }

    /// Checks URI, port number, etc. for validity.
    // TODO
    fn validate_config_request(&self) -> Option<ReqError>{
        if self.cfg.host.is_none() {
            return Some(
                ReqError{ exit_code: FailureCode::ClientError, description: "No remote host given." }
            );
        }
        None
    }


    fn resolve_timeout(&self, timeout: Option<usize>) -> Option<Duration>
    {
        if timeout.is_none() {
            None
        } else {
            Some(Duration::from_millis(timeout.unwrap() as u64))
        }
    }
}
