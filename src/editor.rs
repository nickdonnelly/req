extern crate colored;
extern crate hyper;

use payload::Payload;
use header_editor::parser;
use self::colored::*;
use std;
use std::{io};

pub struct ReqHeader {
    field: String,
    value: String,
}

pub trait Editor {

    fn prompt(&self) -> &str;
    fn instructions(&self) -> &str;
    fn terminate_phrase(&self) -> &str;

    /// Starts the editor with the parameters setup during construction. This is
    /// a blocking function and will block until `terminate_phrase` is typed by the
    /// user. `lines` will be filled with the user's input (excluding the `terminate_phrase`).
    fn start(&mut self, lines: &mut Vec<String>) {
        // Clear screen
        print!("{}[2J", 27 as char);
        
        println!("{}", self.prompt().red());
        println!("{}", self.instructions().green());
        println!("{} {} {}", "To finish editing, type \"".green(), self.terminate_phrase().red(), "\" on it's own line.".green());

        let reader = io::stdin();

        loop {
            let mut line: String = String::new();
            let result = reader.read_line(&mut line);

            if result.is_err() {
                println!("{}", "There was a problem reading from stdin!".red());
                std::process::exit(2);
            }

            if line.trim() == String::from(self.terminate_phrase()) {
                break;
            }

            let line = String::from(line.trim());
            lines.push(line);
        }
    }

}

pub struct PayloadEditor {
    prompt: String,
    instructions: String,
    terminate_phrase: String,
}

pub struct HeaderEditor {
    prompt: String,
    instructions: String,
    terminate_phrase: String,
}


impl PayloadEditor {
    pub fn new() -> PayloadEditor {
        PayloadEditor {
            prompt: String::from("Payload Editor"),
            instructions: String::from("Enter your payload (newlines will be honored)."),
            terminate_phrase: String::from("end"),
        }
    }

    pub fn as_payload(lines: &Vec<String>) -> Payload {
        let payload_str: String = lines.iter().fold(String::new(), |item, col| {
                item + col.as_str() + "\n" });
        return Payload::from(payload_str);
    }

}

impl HeaderEditor {
    pub fn new() -> HeaderEditor {
        HeaderEditor {
            prompt: String::from("Header Editor"),
            instructions: String::from("Type headers each on their own line in the form: \nFieldName: Value"),
            terminate_phrase: String::from("end"),
        }
    }
    pub fn as_headers(lines: &Vec<String>) -> Vec<ReqHeader> {
        lines.iter().map(|line| {
            let header = ReqHeader::from_string(line);
            if header.is_err() {
                println!("Couldn't parse header: {}", header.err().unwrap());
                std::process::exit(2);
            }

            header.unwrap()
        }).collect()
    }

    pub fn write_all_headers(&self, lines: &Vec<String>, request_headers: &mut hyper::header::Headers) {
        let headers = HeaderEditor::as_headers(lines);
        for header in headers {
            request_headers.remove_raw(header.field.as_str());
            request_headers.set_raw(header.field.clone(), header.value.clone());
        }
    }
    
    // TODO!
    pub fn set_default_headers(&self, request_headers: &mut hyper::header::Headers) {
    }
}

impl Editor for PayloadEditor {
    fn prompt(&self) -> &str {
        return self.prompt.as_str();
    }

    fn instructions(&self) -> &str {
        return self.instructions.as_str();
    }

    fn terminate_phrase(&self) -> &str {
        return self.terminate_phrase.as_str();
    }
}

impl Editor for HeaderEditor {
    fn prompt(&self) -> &str {
        return self.prompt.as_str();
    }

    fn instructions(&self) -> &str {
        return self.instructions.as_str();
    }

    fn terminate_phrase(&self) -> &str {
        return self.terminate_phrase.as_str();
    }
}

impl ReqHeader {
    /// Converts a string of the form "HeaderName: Header Value" to a ReqHeader
    /// validating along the way. The error messages in the `String` are verbose
    /// and are appropriately colored for direct terminal printing.
    pub fn from_string(value: &String) -> Result<ReqHeader, String>{
        if !value.contains(":") {
            return Err(String::from("invalid format (no : found)"));
        }

        let splits: Vec<&str> = value.splitn(2, ':').collect();
        if splits.len() != 2 {
            return Err(format!("couldn't get fields for header {}", value.magenta()));
        }
        let field = splits[0];
        let body = splits[1];

        if !parser::is_valid_field_name(&String::from(field)){
            return Err(format!("field name {} is not valid", field.magenta()));
        }

        if !parser::is_valid_field_body(&String::from(body)){
            return Err(format!("field body {} is not valid", body.magenta()));
        }

        // If we've made it this far, we're all good to go.
        Ok(ReqHeader{
            field: String::from(field), // this is done for lifetime reasons
            value: String::from(body.trim()),
        })
    }
}

impl std::fmt::Display for ReqHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.value)
    }
}
