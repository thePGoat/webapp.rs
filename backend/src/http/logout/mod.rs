//! The logout request

use actix::{dev::ToEnvelope, prelude::*};
use actix_web::{AsyncResponder, HttpRequest, HttpResponse};
use cbor::{CborRequest, CborResponseBuilder};
// use database::DeleteSession;
use futures::Future;
use http::FutureResponse;
use server::State;
use webapp::protocol::{model::Session, request, response};

use Data;

mod test;

pub fn logout(http_request: &HttpRequest<State>) -> FutureResponse {
    // Remove the session from the database
    CborRequest::new(http_request)
        .from_err()
        .and_then(move |request::Logout(Session { token })| {
            debug!("Session token {} wants to be logged out", token);
            Ok(HttpResponse::Ok().cbor(response::Logout)?)
        }).responder()
}
