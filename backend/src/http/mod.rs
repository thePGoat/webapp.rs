//! HTTP message handling

pub mod login_credentials;
pub mod login_session;
pub mod logout;
mod test;

use actix::{dev::ToEnvelope, prelude::*};
use actix_web::{error::Error, HttpRequest, HttpResponse};
use cbor::CborRequest;
use futures::{future::FromErr, Future};
pub use http::{
    login_credentials::login_credentials, login_session::login_session, logout::logout,
};
use serde::de::DeserializeOwned;
use server::State;

/// The generic response
pub type FutureResponse = Box<Future<Item = HttpResponse, Error = Error>>;
