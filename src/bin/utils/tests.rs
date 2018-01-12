/*****
*
* NOTICE: The tests in this file MUST be run on a single thread. They involve
* setting and unsetting environment variables for the process and therefore
* will interfere with one another if you attempt to run them in parallel. Pass
* the --test-threads=1 flag to the test executable if you are running these tests.
*
*****/
use reqlib::{ ReqConfig, ReqCommand, ReqOption, RequestMethod };
use clap::{App};
use std::env;

fn input_vec_from_str(inp: &str) -> Vec<&str>
{
    inp.split(' ').collect()
}

fn new_config() -> ReqConfig {
    ReqConfig::new()
        .global_defaults()
        .environment_defaults()
}

fn config_from_str(inp: &str) -> ReqConfig 
{
    
    let v = input_vec_from_str(inp);
    let matches = super::build_app().get_matches_from_safe(v).unwrap();
    super::process_arg_matches(matches, new_config())
}

fn purge_env()
{
    // purge the environment variables so we are at true defaults
    env::remove_var("REQ_HTTP_METHOD"); 
    env::remove_var("REQ_PAYLOAD_FILE"); 
    env::remove_var("REQ_TIMEOUT"); 
    env::remove_var("REQ_MAX_REDIRECTS"); 
    env::remove_var("REQ_URI"); 
}

#[test]
fn test_vanilla_request() 
{
    purge_env();
    let config = config_from_str("req get google.com");

    assert_eq!(config.host.unwrap(), "http://google.com");
    assert_eq!(config.command, ReqCommand::Request(RequestMethod::Get));
}

#[test]
fn test_normal_with_scheme() 
{
    purge_env();
    let config = config_from_str("req post https://memes.google.com");

    println!("CONFIG HOST; {:?}", config.host);
    assert_eq!(config.host.unwrap(), "https://memes.google.com");
    assert_eq!(config.command, ReqCommand::Request(RequestMethod::Post));
}

#[test]
fn test_default_method() 
{
    purge_env();
    let config = config_from_str("req google.com");

    assert_eq!(config.host.unwrap(), "http://google.com");
    assert_eq!(config.command, ReqCommand::Request(RequestMethod::Get)); 
}

#[test]
fn test_env_method() {
    purge_env();
    env::set_var("REQ_HTTP_METHOD", "post");

    let config = config_from_str("req https://abc.google.com");

    assert_eq!(config.host.unwrap(), "https://abc.google.com");
    assert_eq!(config.command, ReqCommand::Request(RequestMethod::Post));
}

#[test]
fn test_one_print_flag() 
{
    purge_env();
    let config = config_from_str("req put google.com -p config");

    assert!(config.options.contains(&ReqOption::PRINT(String::from("config"))));
}

#[test]
fn test_many_print_flags() 
{
    purge_env();
    let config = config_from_str("req put --print request-headers google.com -p config -p headers");

    assert!(config.options.contains(&ReqOption::PRINT(String::from("config"))));
    assert!(config.options.contains(&ReqOption::PRINT(String::from("request-headers"))));
    assert!(config.options.contains(&ReqOption::PRINT(String::from("headers"))));
}

#[test]
#[should_panic]
fn test_bad_print_flag()
{
    purge_env();
    let config = config_from_str("req options google.com -p config -p notvalid");
}

#[test]
fn test_timeout() 
{
    purge_env();
    let config = config_from_str("req connect google.com --timeout 3000");
    
    assert_eq!(config.timeout.unwrap(), 3000);
}

#[test]
fn test_timeout_env()
{
    purge_env();
    env::set_var("REQ_TIMEOUT", "2000");
    let config = config_from_str("req trace google.com");

    assert_eq!(config.timeout.unwrap(), 2000);
}

#[test]
fn test_env_override()
{
    purge_env();
    env::set_var("REQ_TIMEOUT", "2000");
    let config = config_from_str("req trace google.com --timeout 5000");

    assert_eq!(config.timeout.unwrap(), 5000);
}
