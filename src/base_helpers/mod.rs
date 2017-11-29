pub use super::req_types::*;

use std::str::FromStr;
use std::fmt;
use std::fmt::{ Formatter, Display };
use std::result;

impl Display for ReqError {
    fn fmt(&self, f: &mut Formatter) ->
        result::Result<(), fmt::Error> {
            write!(f, "Exited with code {}.\n{}", self.exit_code.value(), self.description)
        }
}

impl ReqConfig {
    /// Instantiates a new, blank config.
    pub fn new() -> ReqConfig {
        ReqConfig {
            command: ReqCommand::Request(RequestMethod::Get),
            host: None,
            port: None,
            timeout: None,
            payload: None,
            options: None
        }
    }

    /// Consumes the given config and produces one that contains
    /// the provided command.
    pub fn command(mut self, cmd: ReqCommand) -> ReqConfig {
        self.command = cmd; 
        self
    }

    /// Consumes the given config and produces one that contains
    /// the provided host.
    pub fn host(mut self, host: String) -> ReqConfig {
        if host.starts_with("http") {
            self.host = Some(host);
        } else { 
            let mut s = String::from("http://");
            s.push_str(host.as_str());
            self.host = Some(s)
        }
        self
    }


    /// Consumes the given config and produces one that contains
    /// the provided host.
    pub fn host_str(mut self, host: &str) -> ReqConfig {
        if host.starts_with("http") {
            self.host = Some(String::from(host));
        } else {
            let mut s = String::from("http://");
            s.push_str(host);
            self.host = Some(s)
        }
        self
    }

    /// Consumes the given config and produces one that contains
    /// the provided port.
    pub fn port(mut self, port: usize) -> ReqConfig {
        self.port = Some(port);
        self
    }

    /// Consumes the given config and produces one that contains
    /// the provided timeout.
    pub fn timeout(mut self, timeout: usize) -> ReqConfig {
        self.timeout = Some(timeout);
        self
    }

    /// Consumes the given config and produces one that contains
    /// the provided option on top of any previous ones. Duplicates
    /// will not be added.
    pub fn option(mut self, opt: ReqOption) -> ReqConfig {
        if self.options.is_none() {
            let opts = vec![opt];
            self.options = Some(opts);
            self
        } else {
            let mut old_opts = self.options.unwrap();
            let mut new_opts = Vec::<ReqOption>::new();
            new_opts.append(&mut old_opts);
            if !old_opts.contains(&opt) {
                new_opts.push(opt);
            }
            self.options = Some(new_opts);
            self
        }
    }

    /// Consumes the given config and produces one that contains
    /// the provided options. Does not preserve old options. 
    pub fn options(mut self, opts: Vec<ReqOption>) -> ReqConfig {
        self.options = Some(opts);
        self
    }

    /// Consumes the given config and produces one that contains
    /// the provided payload.
    pub fn payload(mut self, payload: Payload) -> ReqConfig {
        self.payload = Some(payload);
        self
    }

    /// Consumes the given config and produces one that contains
    /// the defaults in the .env file of the current working
    /// directory.
    /// This method can only be called when you've called
    /// dotenv().ok() during the initialization of the application.
    pub fn environment_defaults(mut self) -> ReqConfig {
        use std::env;

        //TODO: Conversions for commented lines (string wrong type)
        for (key, value) in env::vars() {
            let mref = &mut self;
            match key.as_str() {
                "REQ_DEFAULT_HOST" => mref.host = Some(value),
                "REQ_DEFAULT_PORT" => mref.port = Some(value.trim().parse()
                    .expect("REQ_DEFAULT_PORT invalid")),
                "REQ_DEFAULT_HTTP_METHOD" => mref.command = 
                    ReqCommand::Request(
                        RequestMethod::from_str(value.as_str())
                        .expect("REQ_DEFAULT_HTTP_METHOD invalid")),
                //"REQ_DEFAULT_COMMAND" => mref.command = value,
                //"REQ_DEFAULT_TIMEOUT" => mref.timeout = Some(value),
                //"REQ_DEFAULT_PAYLOAD_FILE" => mref.payload = Some(Payload::from_file(value))
                _ => {}
            }
        }

        self
    }

    /// Consumes the given config and produces one that contains
    /// the system-wide defaults. These can be configured in the install
    /// directory
    pub fn global_defaults(mut self) -> ReqConfig {
        // TODO
        self
    }
}

impl Payload {
    pub fn new(data: Vec<u8>, content_type: &str) -> Payload {
        Payload {
            data: data,
            content_type: String::from(content_type)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Payload, Req, ReqConfig, ReqCommand, RequestMethod};
    fn configuration_optionless() -> ReqConfig {
        ReqConfig {
            command: ReqCommand::Request(RequestMethod::Get),
            host: Some(String::from("www.google.com")),
            port: Some(443),
            timeout: Some(10000),
            payload: Some(Payload { data: vec![1,2,3], content_type: String::from("application/octet-stream") }),
            options: None
        }
    }

    #[test]
    fn new_client_configures_correctly() {
        let config = configuration_optionless();

        let pl = Payload::new(vec![1,2,3], "application/octet-stream");

        let mut built_config = ReqConfig::new()
            .command(ReqCommand::Request(RequestMethod::Get))
            .host(String::from("www.google.com"))
            .port(443)
            .payload(pl)
            .timeout(10000);

        assert_eq!(config, built_config);
    }
}
