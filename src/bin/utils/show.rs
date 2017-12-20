use reqlib::{ ReqConfig, Payload, FailureCode, ReqResource, ReqCommand };
use clap::{ Values, ArgMatches };

pub fn show_payload<'a>(cfg: ReqConfig, payload_args: &ArgMatches<'a>) -> ReqConfig
{
    use std::process;

    match payload_args.value_of("payload") {
        Some(filename) => {
            let payload = Payload::from_file(filename);

            if payload.is_err() {
                eprintln!("An IO error ocurred:\n{}", payload.err().unwrap());
                process::exit(FailureCode::IOError.value() as i32);
            }

            let payload = payload.unwrap();

            cfg.command(ReqCommand::Show(ReqResource::Body(payload)))
        },
        None => { 
            eprintln!("No payload file provided!");
            process::exit(FailureCode::ClientError.value() as i32);

            cfg // for compilation purposes.
        }
    }
}
