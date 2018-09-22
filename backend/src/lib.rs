//! The backend library

#![feature(extern_prelude)]
#![deny(missing_docs)]

extern crate actix;
extern crate actix_web;
extern crate bytes;
// extern crate diesel;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate openssl;
// extern crate r2d2;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate url;
extern crate uuid;
extern crate webapp;

mod cbor;
// mod database;
mod http;
mod server;
mod token;

pub use server::Server;

use actix::prelude::*;
use failure::Fallible;
use webapp::protocol::model::Session;

/// Documentation
pub struct Data(pub String);

impl Message for Data {
    type Result = Fallible<Session>;
}

impl From<String> for Data {
    fn from(s: String) -> Data {
        Data(s)
    }
}
