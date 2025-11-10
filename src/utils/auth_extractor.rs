use std::{fs::read, future::{Ready, ready}};
use crate::utils::auth::decode_jwt;
use actix_web::{
    dev::Payload,
    Error as ActixError, FromRequest , HttpRequest
};
pub struct AuthenticatedUser {
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = Ready<Result<Self,Self::Error>>;
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let header = match req.headers().get("Authorization") {
            Some(header) => header,
            None =>{ 
                return ready(Err(actix_web::error::ErrorUnauthorized("No token provided")));
            }
        };
        let auth_str = match header.to_str() {
            Ok(auth_str) => auth_str,
            Err(_) => {
                return  ready(Err(actix_web::error::ErrorUnauthorized("Invalid token format")));
            }
        };
        let vec : Vec<&str> = auth_str.split_whitespace().collect();
        if vec.len() != 2 || vec[0] != "Bearer" {
            // Header is not "Bearer <token>"
            return ready(Err(actix_web::error::ErrorUnauthorized("Invalid token format")));
        }
        let token = vec[1];
        match decode_jwt(token) {
            Ok(claims) => {
                ready(Ok(AuthenticatedUser { username: claims.sub }))
            }
            Err(_) => {
                // Token is invalid (expired, wrong signature, etc.)
                ready(Err(actix_web::error::ErrorUnauthorized("Invalid or expired token")))
            }
        }
    }
}