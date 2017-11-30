extern crate reqlib;
extern crate dotenv;
use reqlib::*;
use dotenv::dotenv;
use std::process;

fn main() {
    dotenv().ok(); // Add the dotenv environment variables to env::vars
    let mut config = ReqConfig::new()
      .global_defaults()
      .environment_defaults(); // Environment must be after so it overrides global.

    let req = Req::new_from_cfg(config).unwrap();
    let result = req.run();
    if result.is_err() {
        print_error_message(result.err());
    } else {
        handle_result(result.unwrap());
    }
}

fn handle_result(res: ReqCommandResult) {

}

fn print_error_message(e: Option<ReqError>)
{
    if e.is_none() {
        println!("An unknown error occurred.");
    } else {
        let e = e.unwrap();
        println!("An error occurred:\n{}", &e.description);
        process::exit(e.exit_code.value() as i32);
    }
}
