use jsonwebtoken::{
    encode, decode, Header, Validation, 
    EncodingKey, DecodingKey, Algorithm, 
    errors::Error as JwtError
};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::auth::claims::{Claims, LoginRequest};
use axum::Json;
use crate::context::environment::{Environment, Singleton};

fn get_jwt_token_exp(time_hours: u64) -> u64 {
    (SystemTime::now() + Duration::from_secs(time_hours * 3600)).duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn create_jwt(Json(payload): Json<LoginRequest>, secret: &[u8]) -> Result<String, JwtError> {
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret);
    let claims: Claims = Claims {
        username: payload.username,
        exp: get_jwt_token_exp(24),
    };
    encode(&header, &claims, &encoding_key)

}

pub fn validate_jwt(token: &str, secret: &[u8]) -> Result<Claims, JwtError> {
    let decoding_key = DecodingKey::from_secret(secret);
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}



pub fn validate_credentials(username: &str, password: &str) -> bool {
    let expected_username = Environment::get_var("USERNAME").expect("USERNAME environment variable not set");
    let expected_password = Environment::get_var("PASSWORD").expect("PASSWORD environment variable not set");
    username == expected_username && password == expected_password
}