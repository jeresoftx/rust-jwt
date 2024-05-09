use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct UserDto {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    email: String,
    company: String,
    exp: i64,
}

pub fn get_jwt(user: UserDto) -> Result<String, String> {
    dotenv().ok();
    let secret_jwt_key = env::var("SECRET_JWT_KEY").expect("Can't get secret jwt key");

    let token = encode(
        &Header::default(),
        &Claims {
            email: user.email,
            company: "softrek".to_string(),
            exp: (Utc::now() + Duration::minutes(1)).timestamp(),
        },
        &EncodingKey::from_secret(secret_jwt_key.as_bytes()),
    )
    .map_err(|e| e.to_string());

    token
}

pub fn decode_jwt(token: &str) -> Result<UserDto, String> {
    dotenv().ok();
    let secret_jwt_key = env::var("SECRET_JWT_KEY").expect("Can't get secret jwt key");
    let token_data = decode::<UserDto>(
        token,
        &DecodingKey::from_secret(secret_jwt_key.as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),

        Err(e) => Err(e.to_string()),
    }
}
