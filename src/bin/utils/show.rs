/*******
 * This class' methods can fail if they don't parse valid arguments.
 * In that event, they exit gracefully with an error message.
 *
 * TODO: Move validations of arguments to clap-rs "Validators".
 ******/

use reqlib::{ ReqConfig, Payload, FailureCode, ReqResource, ReqCommand, ReqOption };
use super::super::encode::Encoding;
use clap::{ ArgMatches };
use std::process;

pub fn show_payload<'a>(cfg: ReqConfig, payload_args: &ArgMatches<'a>) -> ReqConfig
{
    let payload = match payload_args.value_of("payload") {
        Some(filename) => {
            let payload = Payload::from_file(filename);

            if payload.is_err() {
                eprintln!("An IO error ocurred:\n{}", payload.err().unwrap());
                process::exit(FailureCode::IOError.value() as i32);
            }

            payload.unwrap()

        },
        None => { 
            eprintln!("No payload file provided!");
            process::exit(FailureCode::ClientError.value() as i32);
        }
    };

    let (mut payload, encoding_type) = match payload_args.value_of("encoding") {
        Some(v) => {
            (super::config_extract::encode_payload(v, payload), Encoding::from_str(v))
        },
        None => {
            (payload, None)
        }
    };

    if let Some(prefix) = payload_args.value_of("body-prefix") {
        payload.insert_prefix(String::from(prefix).into_bytes());
    }

    if encoding_type.is_some() {
        cfg.option(ReqOption::Encode(encoding_type.unwrap()))
           .command(ReqCommand::Show(ReqResource::Body(payload)))
    } else {
        cfg.command(ReqCommand::Show(ReqResource::Body(payload)))
    }

}

pub fn show_env<'a>(cfg: ReqConfig, env_args: &ArgMatches<'a>) -> ReqConfig
{
    // We don't validate the value of env_variable because we have set what
    // is valid in the clap definition, so this can't be invalid if it gets
    // to this method.
    match env_args.value_of("env_variable") {
        Some(var) => {
            let resource = match var {
                "uri" => ReqResource::EnvVar(String::from("REQ_URI")),
                "timeout" => ReqResource::EnvVar(String::from("REQ_TIMEOUT")),
                "http_method" => ReqResource::EnvVar(String::from("REQ_HTTP_METHOD")),
                "max_redirects" => ReqResource::EnvVar(String::from("REQ_MAX_REDIRECTS")),
                "payload_file" => ReqResource::EnvVar(String::from("REQ_PAYLOAD_FILE")),
                "all" | _ => ReqResource::Env,
            };

            cfg.command(ReqCommand::Show(resource))
        },

        None => {
            cfg.command(ReqCommand::Show(ReqResource::Env))
        }
    }
}
