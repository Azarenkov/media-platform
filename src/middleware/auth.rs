use std::future::{Ready, ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{Error as ActixWebError, web};
use actix_web::{
    FromRequest,
    http::{self},
};
use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}

impl Claims {
    fn new(id: usize, exp: usize) -> Self {
        Self { id, exp }
    }
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

pub async fn encode_token(id: usize, secret: &str) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims = Claims::new(id, exp);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );
    token.context("Errod encode token()")
}
