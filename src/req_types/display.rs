use std::fmt::{self, Display, Error, Formatter};
use super::*;
use std::str;

impl FailureCode {
    pub fn value(&self) -> u16 {
        match *self {
            FailureCode::NoError     => 0,
            FailureCode::ClientError => 1,
            FailureCode::IOError     => 2,
            FailureCode::Timeout     => 3
        }
    }
}

impl Display for ReqError {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> 
    {
        write!(f, "Exited with code {}.\n{}", self.exit_code.value(), self.description)
    }
}

impl Display for ReqConfig {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error>
    {
        
        let command_string = match  &self.command {
            &ReqCommand::Request(ref method) => {
                let mut request_string = format!("Request with HTTP method {}\n", method);

                let host_string = if self.host.is_some() {
                    self.host.clone().unwrap()
                } else { 
                    String::from("None given")
                };

                let port_string = if self.port.is_some() {
                    format!("{}", self.port.clone().unwrap())
                } else { 
                    String::from("80")
                };

                let timeout_string = if self.timeout.is_some() {
                    format!("{}", self.timeout.clone().unwrap())
                } else {
                    String::from("30000")
                };

                let payload_string = if self.payload.is_some() {
                    format!("<{} bytes>", self.payload.clone().unwrap().data.len())
                } else {
                    String::from("<0 bytes>")
                };

                let options_string = if self.options.len() > 0 {
                    self.options.iter().fold(String::new(), |mut acc, val| {
                        if acc == "" {
                            acc.push_str(format!("{}\n", val).as_str());
                        } else {
                            acc.push_str(format!("         {}\n", val).as_str());
                        }
                        acc
                    })
                } else {
                    String::from("No options")
                };

                format!("Request with HTTP method {} \n\
                        Host: {}\n\
                        Port: {}\n\
                        Timeout: {}\n\
                        Payload: {}\n\
                        Options: {}\n", 
                        request_string, host_string, 
                        port_string, timeout_string, 
                        payload_string, options_string)
            },
            &ReqCommand::Show(ref resource) => format!("Show {:?}", resource),
            &ReqCommand::CleanEnvironment => format!("Clean Environment")
        };

        write!(f, "{}", command_string)
    }
}

impl Display for ReqOption {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        match self {
            &ReqOption::PRINT(ref s) => write!(f, "Print {}", s),
            &ReqOption::CUSTOM_HEADER(ref t) => write!(f, "Header {}: {}", t.0, t.1),
            &ReqOption::FOLLOW_REDIRECTS(ref count) => {
                if *count <= -1 {
                    write!(f, "Infinite redirects")
                } else {
                    write!(f, "Maximum remaining redirects {}", count)
                }
            }, 
            _ => write!(f, "Unknown option")
        }
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let utf8_decoded = str::from_utf8(&self.data);
        if utf8_decoded.is_err() {
          return write!(f, "{:?}", &self.data);
        }
        write!(f, "{}", utf8_decoded.unwrap())
    }
}

impl Display for ReqResponse{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let utf8_decoded = str::from_utf8(&self.body);
        if utf8_decoded.is_err() {
            return write!(f, "{:?}", &self.body);
        }
        write!(f, "{}", utf8_decoded.unwrap())
    }
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        match self {
            &RequestMethod::Get     => write!(f, "GET"),
            &RequestMethod::Head    => write!(f, "HEAD"),
            &RequestMethod::Put     => write!(f, "PUT"),
            &RequestMethod::Post    => write!(f, "POST"),
            &RequestMethod::Delete  => write!(f, "DELETE"),
            &RequestMethod::Options => write!(f, "OPTIONS"),
            &RequestMethod::Patch   => write!(f, "PATCH"),
            &RequestMethod::Connect => write!(f, "CONNECT"),
            &RequestMethod::Trace   => write!(f, "TRACE")
        }
    }
}

impl Display for ReqHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        write!(f, "{}: {}", &self.name, &self.value)
    }
}
