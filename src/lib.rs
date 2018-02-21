#![allow(non_snake_case)]
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate bytes;
extern crate regex;
extern crate colored; // for socket.
extern crate rand;

mod base_helpers;
mod req_types;
pub mod asset_extract;
pub mod encode;
pub mod options;
pub mod quicksock;

pub use base_helpers::*;

#[cfg(test)]
mod tests {

}
