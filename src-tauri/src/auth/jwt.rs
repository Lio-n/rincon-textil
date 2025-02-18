use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub fn generate_jwt(user_id: String, secret: String) -> Result<String, Box<dyn Error>> {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: 10000000000,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    let token = encode(&header, &claims, &encoding_key)?;
    Ok(token)
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();

    let token_data = decode::<Claims>(&token, &decoding_key, &validation)?;
    Ok(token_data)
}
