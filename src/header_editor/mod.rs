extern crate hyper;
extern crate colored;

use std;
use std::io;
use self::colored::*;

#[derive(Debug)]
pub struct ReqHeader {
    field: String,
    value: String,
}

pub struct HeaderEditor {
    headers: Vec<ReqHeader>,    
}

mod parser;

impl HeaderEditor {
    pub fn new() -> Self {
        let vec: Vec<ReqHeader> = Vec::new();
        HeaderEditor {
            headers: vec,
        }
    }

    /// Sets the default headers for a generic HTTP request. This will set
    /// things like Content-Type to text/plain. The headers set by this method
    /// will be overwritten by user input if the user inputs a header that was
    /// already defined here.
    pub fn set_default_headers(&mut self){
        // TODO: Add default headers here
    }

    /// Starts the headereditor, prints the necessary information to screen,
    /// and then lets the user enter headers until they type "end" by itself
    /// on a line. The lines are automatically parsed at the end and the program
    /// exits if any of the headers are not valid (it attempts to automatically
    /// ASCII escape them, if this fails the header is invalid). Additionally,
    /// headers are invalid if the field of the header contains whitespace after
    /// trimming.
    pub fn start(&mut self){
        // Clear the terminal with magic control character, we will use the whole screen
        print!("{}[2J", 27 as char);

        let mut lines: Vec<String> = Vec::new();
        println!("{}", "Header Editor".red());
        println!("{} {}", "Type headers of the form: \n".blue(), "FieldName: Value".green());
        println!("{}", "Separate headers by going onto a new line. When you have finished entering the desired headers, type the word 'end' on it's own line to continue.".blue());

        // Get all of the headers
        let reader = io::stdin();
        loop {
            let mut line: String = String::new();
            let result = reader.read_line(&mut line);
            if result.is_err(){
                println!("There was a problem reading from stdin!");
                std::process::exit(2);
            }

            // We don't need to add this one
            if line.trim() == "end"{
                break;
            }

            let line = String::from(line.trim());

            lines.push(line);
        }

        // Now we parse them into header types
        let headers: Vec<ReqHeader> = lines.iter().map(|line| {
            let header = ReqHeader::from_string(line);
            if header.is_err(){
                println!("Couldn't parse header: {}", header.err().unwrap());
                std::process::exit(2);
            }

            // we've validated there isn't an error so we can do this safely
            header.unwrap()

        }).collect();

        for header in headers {
            self.add_header(header);
        }
    }

    /// Adds a header to the editor. 
    /// This function performs no validations, and assumes that you have validated
    /// the headers yourself (this can be done with `ReqHeader`).
    pub fn add_header(&mut self, header: ReqHeader) {
        self.headers.push(header);
    }

    /// Write all the current headers in the HeaderEditor to request_headers to be
    /// attached to a request. 
    /// NOTE: This may overwrite any headers that already exist (with some exceptions).
    /// For example, if you try to write the Content-Type header and it was already written by 
    /// `set_default_headers`, then it will be overwritten with the `HeaderEditor`'s version.
    pub fn write_all_headers(&self, request_headers: &mut hyper::header::Headers) {
        for header in &self.headers {
            request_headers.remove_raw(header.field.as_str());
            request_headers.set_raw(header.field.clone(), header.value.clone());
        }
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
