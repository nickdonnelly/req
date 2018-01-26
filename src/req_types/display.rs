use std::fmt::{self, Display, Error, Formatter};
use super::*;
use std::str;

impl FailureCode {
    pub fn value(&self) -> u16 {
        match *self {
            FailureCode::NoError     => 0,
            FailureCode::ClientError => 1,
            FailureCode::IOError     => 2,
            FailureCode::Timeout     => 3,
            FailureCode::Unknown     => 99,
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
            &ReqCommand::Socket(ref port) => format!("Socket on {}", port),
            &ReqCommand::CleanEnvironment => format!("Clean Environment")
        };

        write!(f, "{}", command_string)
    }
}

impl Display for ReqOption {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        match self {
            &ReqOption::Print(ref s) => write!(f, "Print {}", s),
            &ReqOption::CustomHeader(ref t) => write!(f, "Header {}: {}", t.0, t.1),
            &ReqOption::Encode(ref e) => write!(f, "{:?} Encoding", e),
            &ReqOption::FollowRedirects(ref count) => {
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
        use std::fmt::Write;

        let utf8_decoded = str::from_utf8(&self.data);
        if utf8_decoded.is_err() {
            let mut strbuf = String::new();
            for chunk in &self.data {
                write!(&mut strbuf, "0x{:X} ", chunk);
            }
            return write!(f, "Detected Content-Type: {}\n\
                              Print mode: Raw Bytes (space-separated hex)\n\
                              ---\n{}", &self.content_type.as_str(), strbuf);
        }
        write!(f, "Detected Content-Type: {}\n\
                   Print mode: UTF-8\n\
                   ---\n\
                   {}", &self.content_type.as_str(), utf8_decoded.unwrap())
    }
}

impl Display for ReqResponse{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        use std::fmt::Write;

        let utf8_decoded = str::from_utf8(&self.body);
        if utf8_decoded.is_err() {
            let mut strbuf = String::new();
            for chunk in &self.body {
                write!(&mut strbuf, "{}", chunk);
            }
            //return write!(f, "{:?}", &self.body);
            return write!(f, "{}", strbuf);
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
