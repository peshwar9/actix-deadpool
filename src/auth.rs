use std::env;
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
//use jsonwebtoken::errors::ErrorKind;
use crate::errors::ApiError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

// Create a json web token (JWT)
pub fn create_jwt() -> Result<String, ApiError> {
    let key = env::var("JWT_SECRET").expect("JWT secret must be set");
    let env_sub = env::var("JWT_SUBJECT").expect("JWT secret must be set");
    let env_company = env::var("JWT_COMPANY").expect("JWT secret must be set");
    
    let my_claims = Claims {
        sub: env_sub.to_owned(),
        company: env_company.to_owned(),
        exp: 10000000000,
    };
    let encoding_key = EncodingKey::from_secret(key.as_bytes());

    encode(&Header::default(), &my_claims, &encoding_key)
        .map_err(|e| ApiError::CannotEncodeJwtToken(e.to_string()))
    //  .map_err(|e| ResponseError(e.to_string()))
    
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, ApiError> {
    let key = env::var("JWT_SECRET").expect("JWT secret must be set");
    let decoding_key = DecodingKey::from_secret(key.as_bytes());

    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeJwtToken(e.to_string()))

    //   .map_err(|e| ResponseError(e.to_string()))
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    println!("{:?}", credentials.token());
    println!("Going to decode jwt");
    decode_jwt(credentials.token())?;
    Ok(req)
}
