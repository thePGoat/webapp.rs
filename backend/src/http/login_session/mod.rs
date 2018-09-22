//! The session based login request

use actix::{dev::ToEnvelope, prelude::*};
use actix_web::{AsyncResponder, HttpRequest, HttpResponse};
use cbor::{CborRequest, CborResponseBuilder};
// use database::UpdateSession;
use futures::Future;
use http::FutureResponse;
use server::State;
use token::Token;
use webapp::protocol::{model::Session, request::LoginSession, response::Login};
use Data;

mod test;

pub fn login_session(http_request: &HttpRequest<State>) -> FutureResponse {
    // Create a new token for the already given one
    CborRequest::new(http_request)
        .from_err().and_then(|LoginSession(Session {token})| {
            debug!("Session token {} wants to be renewed", token);
            Ok((Token::verify(&token)?, token))
        })
        // Update the session in the database
        .and_then(move |(new_token, old_token)| {
            Ok(HttpResponse::Ok().cbor(Login(Session { token: new_token }))?)
        })
        .responder()
}
