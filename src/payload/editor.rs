extern crate colored;

use super::Payload;
use std;
use std::io;
use self::colored::*;

pub struct PayloadEditor {
    payload_str: String,
}

pub fn new() -> PayloadEditor {
    PayloadEditor {
        payload_str: String::from(""),
    }
}


impl PayloadEditor {
    pub fn start(&mut self) {
        print!("{}[2J", 27 as char);

        let mut lines: Vec<String> = Vec::new();
        println!("{}", "Payload Editor".red());
        println!("{}", "Enter your payload, followed by the word \"end\" on its own line.".green());

        let reader = io::stdin();

        loop {
            let mut line: String = String::new();
            let result = reader.read_line(&mut line);

            if result.is_err() {
                println!("{}", "There was a problem reading from stdin!".red());
                std::process::exit(2);
            }

            if line.trim() == "end" {
                break;
            }

            let line = String::from(line.trim());
            lines.push(line);
        }

        self.payload_str = lines.iter().fold(String::new(), |item, col| {
                item + col.as_str() + "\n"
            });
    }

    pub fn get_payload(&self) -> Payload {
        return Payload::from(self.payload_str.clone());
    }
}
