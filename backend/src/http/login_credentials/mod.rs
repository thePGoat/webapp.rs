//! The credential based login request

use actix::{dev::ToEnvelope, prelude::*};
use actix_web::{error::ErrorUnauthorized, AsyncResponder, HttpRequest, HttpResponse};
use cbor::CborResponseBuilder;
// use database::CreateSession;
use cbor::CborRequest;
use futures::Future;
use http::FutureResponse;
use server::State;
use token::Token;
use webapp::protocol::{
    model::Session,
    request::{self, LoginCredentials},
    response::{self, Login},
};
use Data;

mod test;

pub fn login_credentials(http_request: &HttpRequest<State>) -> FutureResponse {
    CborRequest::new(http_request)
        .from_err()
        .and_then(move |request::Logout(Session { token })| {
            debug!("Session token {} wants to be logged out", token);
            Ok(HttpResponse::Ok().cbor(response::Logout)?)
        }).responder()
}
