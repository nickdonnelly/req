use reqlib::{ReqConfig, ReqOption};
use clap::{Values};


pub fn print_flags<'a>(print_flags: Option<Values<'a>>, cfg: ReqConfig)
    -> ReqConfig
{
    if print_flags.is_some() {
        let mut print_options: Vec<ReqOption> = Vec::new();
        let values: Vec<&str> = print_flags.unwrap().collect();

        for (i, v) in values.iter().enumerate() {
            print_options.push(ReqOption::PRINT(String::from(*v)));
        }

        cfg.options(print_options)
    } else {
        cfg
    }
}

pub fn header_flags<'a>(headers: Option<Values<'a>>, cfg: ReqConfig) -> ReqConfig {
    if headers.is_some() {
        let mut header_options: Vec<ReqOption> = Vec::new();

        let values: Vec<&str> = headers.unwrap().collect();
        for (i, _) in values.iter().enumerate() {
            if i % 2 == 0 {
                header_options.push(
                  ReqOption::CUSTOM_HEADER((String::from(values[i]), String::from(values[i+1]))));
            }
        }

        cfg.options(header_options)
    } else {
        cfg
    }
}