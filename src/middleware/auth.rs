use std::future::{Ready, ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{Error as ActixWebError, web};
use actix_web::{
    FromRequest, HttpMessage,
    http::{self, header::HeaderValue},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Claims {
    pub id: usize,
    pub email: String,
    pub username: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    pub id: usize,
}

impl FromRequest for AuthToken {
    type Error = ActixWebError;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        let auth_token = auth_header.unwrap().to_str().unwrap_or("");
        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token")));
        }

        let secret = req.app_data::<web::Data<String>>().unwrap();

        let decode = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );

        match decode {
            Ok(token) => ready(Ok(AuthToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized"))),
        }
    }
}
