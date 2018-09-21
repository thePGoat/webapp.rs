//! The credential based login request

use actix::{dev::ToEnvelope, prelude::*};
use actix_web::{error::ErrorUnauthorized, AsyncResponder, HttpRequest, HttpResponse};
use cbor::CborResponseBuilder;
//use database::CreateSession;
use futures::Future;
use http::{unpack_cbor, FutureResponse};
use server::State;
use token::Token;
use webapp::protocol::{request, response, request::LoginCredentials, response::Login, model::Session};
use ::Data;

mod test;

pub fn login_credentials<T: Actor>(http_request: &HttpRequest<State>) -> FutureResponse
where
    T: Actor + Handler<Data>,
    <T as Actor>::Context: ToEnvelope<T, Data>
{
    let (request_clone, cbor) = unpack_cbor(http_request);

    cbor.and_then(move |request::Logout(Session { token })| {
        debug!("Session token {} wants to be logged out", token);
        Ok(HttpResponse::Ok().cbor(response::Logout)?)
        /*
        request_clone
            .state()
            .database
            .send(DeleteSession(token))
            .from_err()
            .and_then(|result| {
                result?;
                Ok(HttpResponse::Ok().cbor(response::Logout)?)
            })*/
    }).responder()


/*
    // Verify username and password
    cbor.and_then(move |LoginCredentials { username, password }| {
            debug!("User {} is trying to login", username);
            if username.is_empty() || password.is_empty() || username != password {
                return Err(ErrorUnauthorized("wrong username or password"));
            }
            Ok(username)
        })
        // Create a new token
        .and_then(|username| Ok(Token::create(&username)?))
        // Update the session in the database
        .and_then(move |token| {
          // request_clone
          //      .state()
          //      .database
          //      .send(CreateSession(token))
          //      .from_err()
          //      .and_then(|result| Ok(HttpResponse::Ok().cbor(Login(result?))?))
          //      Ok(HttpResponse::Ok().cbor(Login(Session { token }))?)
          Ok(HttpResponse::Ok().cbor("")?)
        })
        .responder()
*/
}
