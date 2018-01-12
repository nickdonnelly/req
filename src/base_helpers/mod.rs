pub use super::req_types::*;

use super::encode::Encoding;

use std::str::FromStr;
use std::fmt;
use std::fmt::{ Formatter, Display };
use std::result;
use std::io::Error;

impl ReqConfig {
    /// Instantiates a new, blank config.
    pub fn new() -> ReqConfig {
        ReqConfig {
            command: ReqCommand::Request(RequestMethod::Get),
            host: None,
            port: None,
            timeout: None,
            payload: None,
            options: Vec::new()
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
        if !self.options.contains(&opt) {
            self.options.push(opt);
        }
        self
    }

    /// Consumes the given config and produces one that contains
    /// the provided options. Does not preserve old options. 
    pub fn options(mut self, mut opts: Vec<ReqOption>) -> ReqConfig {
        self.options.append(&mut opts);
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

        for (key, value) in env::vars() {
            let mref = &mut self;
            match key.as_str() {
                "REQ_URI" => env::set_var("REQ_URI", ReqConfig::fix_schemeless_uri(value.as_str())),
                "REQ_PORT" => mref.port = Some(value.trim().parse()
                    .expect("REQ_PORT invalid")),
                "REQ_HTTP_METHOD" => mref.command = 
                    ReqCommand::Request(RequestMethod::from_str(value.as_str())
                        .expect("REQ_HTTP_METHOD invalid")),
                _ => {}
            }
        }

        self
    }

    /// Consumes the given config and produces one that contains
    /// the system-wide defaults. 
    /// Makes the default command `get` and sets timeout to 30s
    pub fn global_defaults(mut self) -> ReqConfig {
        self.command = ReqCommand::Request(RequestMethod::Get);
        self.timeout = Some(30000); 
        self
    }

    pub fn should_redirect(&self) -> bool
    {
        for opt in &self.options {
            match opt {
                &ReqOption::FOLLOW_REDIRECTS(ref val) => {
                    if val == &(-1) || val > &0 {
                        return true;
                    }
                },
                _ => {}
            }
        }

        false
    }

    pub fn reduce_redirect_count(&mut self)
    {
        let new_opts: Vec<ReqOption> = Vec::new();

        for val in self.options.iter_mut() {
            match val {
                &mut ReqOption::FOLLOW_REDIRECTS(v) => {
                    if v > 0 { *val = ReqOption::FOLLOW_REDIRECTS(v - 1); }
                },
                _ => {}
            }
        }
    }

    pub fn fix_schemeless_uri(uri: &str) -> String
    {
        if uri.starts_with("http") {
            String::from(uri)
        } else {
            let mut s = String::from("http://");
            s.push_str(uri);
            s
        }
    }
}


impl Clone for Payload {
    fn clone(&self) -> Payload
    {
        Payload {
            data: self.data.to_vec(),
            content_type: self.content_type.clone(),
            encoding: Encoding::NoEncoding
        }
    }
}

impl Payload {
    /// Generate an empty `Payload` with `ReqContentType::Empty`
    pub fn empty() -> Payload
    {
        Payload {
            data: Vec::new(),
            content_type: ReqContentType::Empty,
            encoding: Encoding::NoEncoding
        }
    }

    /// Generate a `Payload` with the given data and content type
    pub fn new_typed(data: Vec<u8>, content_type: ReqContentType) -> Payload 
    {
        Payload {
            data: data,
            content_type: content_type,
            encoding: Encoding::NoEncoding
        }
    }

    /// Generate a `Payload` with given data and content type string (of the form
    /// "application/octet-stream", "image/png", etc.
    pub fn new(data: Vec<u8>, content_type: &str) -> Payload 
    {
        Payload {
            data: data,
            content_type: ReqContentType::Custom(String::from(content_type)),
            encoding: Encoding::NoEncoding
        }
    }

    /// Generate a new `Payload` from a file. This operation may fail. Content type
    /// is automatically determined by file extension, or is automatically set to 
    /// application/octet-stream if the MIME type is unknown.
    pub fn from_file(filename: &str) -> Result<Payload, Error>
    {
        use std::fs::File;
        use std::path::Path;
        use std::io::Read;

        let file_ext = Path::new(filename.clone()).extension();
        let ctt = if file_ext.is_some() {
            let os_ext = file_ext.unwrap().to_str(); 
            if os_ext.is_none() {
                ReqContentType::OctetStream
            } else {
                ReqContentType::from_extension(os_ext.unwrap())
            }
        } else { 
            ReqContentType::OctetStream
        };

        let try_file = File::open(String::from(filename));

        if try_file.is_ok() {
            let mut file = try_file.unwrap();
            let mut buf: Vec<u8> = Vec::new();

            file.read_to_end(&mut buf);
            
            Ok(Payload {
                data: buf,
                content_type: ctt,
                encoding: Encoding::NoEncoding // TODO: Detect if the file is already encoded
            })
        } else {
            Err(try_file.err().unwrap())
        }

    }

    /// Get the content type as a reference
    pub fn content_type(&self) -> &ReqContentType
    {
        &self.content_type
    }

    /// Get the content type as a str reference (as it will be printed in headers).
    pub fn content_type_str(&self) -> &str {
        self.content_type.as_str()
    }

    /// Consume the payload and retrieve its data.
    pub fn data(self) -> Vec<u8> 
    {
        self.data
    }
    
    /// Get a reference to the data inside the payload.
    pub fn data_ref(&self) -> &Vec<u8> 
    {
        &self.data
    }

    pub fn encoding(&self) -> Encoding
    {
        self.encoding.clone()
    }
}

impl ReqContentType {
   /// Convert file extension (with or without ".") to a ReqContentType.
   /// #Examples 
   /// ```
   /// use reqlib::ReqContentType;
   /// assert_eq!(ReqContentType::Html, ReqContentType::from_extension(".html"));
   /// ```
   pub fn from_extension(ext: &str) -> ReqContentType
   {
       ReqContentType::from_str(ext.trim_left_matches('.')).unwrap()
   }

   pub fn as_str(&self) -> &str {
      match *self {
          ReqContentType::Json => "application/json",
          ReqContentType::Xml  => "application/xml",
          ReqContentType::Png => "image/png",
          ReqContentType::Jpeg => "image/jpeg",
          ReqContentType::Webm => "video/webm",
          ReqContentType::Webp => "image/webp",
          ReqContentType::Gif => "image/gif",
          ReqContentType::Mp4 => "video/mpeg",
          ReqContentType::Ogg => "audio/ogg",
          ReqContentType::Html => "application/html",
          ReqContentType::Css => "application/css",
          ReqContentType::Javascript => "application/javascript",
          ReqContentType::Zip => "application/zip",
          ReqContentType::Custom(ref v) => v.as_str(),
          ReqContentType::OctetStream | _ => "application/octet-stream",
      }
   }
}

impl FromStr for ReqContentType {
    type Err = (); // We will never error out (default to application/octet-stream)

    /// INTERNAL USE ONLY! This does NOT convert a type e.g. application/javascript to this type!
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        Ok(match s.to_lowercase().as_str() {
            "json" => ReqContentType::Json,
            "xml" => ReqContentType::Xml,
            "png" => ReqContentType::Png,
            "jpg" | "jpeg" => ReqContentType::Jpeg,
            "webm" => ReqContentType::Webm,
            "webp" => ReqContentType::Webp,
            "gif" => ReqContentType::Gif,
            "mp4" => ReqContentType::Mp4,
            "ogg" => ReqContentType::Ogg,
            "html" => ReqContentType::Html,
            "css" => ReqContentType::Css,
            "js" => ReqContentType::Javascript,
            "zip" => ReqContentType::Zip,
            _ => ReqContentType::OctetStream,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Payload, Req, ReqConfig, ReqCommand, ReqContentType, RequestMethod, };
    use super::super::encode::Encoding;
    fn configuration_optionless() -> ReqConfig {
        ReqConfig {
            command: ReqCommand::Request(RequestMethod::Get),
            host: Some(String::from("https://www.google.com")),
            port: Some(443),
            timeout: Some(10000),
            payload: Some(Payload { data: vec![1,2,3], 
                content_type: ReqContentType::OctetStream, 
                encoding: Encoding::NoEncoding }),
            options: Vec::new()
        }
    }

    #[test]
    fn new_client_configures_correctly() {
        let config = configuration_optionless();

        let pl = Payload::new_typed(vec![1,2,3], ReqContentType::OctetStream);

        let mut built_config = ReqConfig::new()
            .command(ReqCommand::Request(RequestMethod::Get))
            .host(String::from("https://www.google.com"))
            .port(443)
            .payload(pl)
            .timeout(10000);

        assert_eq!(config, built_config);
    }
}
