use actix_web::web::to;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (the username)
    pub exp: usize,  
}

pub fn create_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {

    let JWT_SECRET = std::env::var("JWT_SECRET").unwrap_or_else(|_|"JWT".to_string() );
    let expiration = Utc::now().checked_add_signed(Duration::days(1)).expect("valid timestamp").timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let JWT_SECRET = std::env::var("JWT_SECRET").unwrap_or_else(|_|"JWT".to_string() );
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::default());
    match token_data {
        Ok(data) => {
            Ok(data.claims)
        }
        Err(err) => {
            Err(err)
        }
    }
}