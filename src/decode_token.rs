use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedTokenResponse {
    pub application: String,
    pub address: String,
    pub iat: usize,
    pub exp: usize,
    pub iss: String,
    // Add other fields from your JWT payload
}

pub fn decode_token(token: &str, secret_key: &str) -> Result<DecodedTokenResponse, String> {
    let key = DecodingKey::from_secret(secret_key.as_bytes());
    match decode::<DecodedTokenResponse>(token, &key, &Validation::new(Algorithm::HS256)) {
        Ok(TokenData { claims, .. }) => {
            println!("claims.exp: {:?}", claims.exp);
            let application = claims.application;
            let address = claims.address;
            let iat = claims.iat;
            let exp = claims.exp;
            let iss = claims.iss;
            let response = DecodedTokenResponse { application, address, iat, exp, iss };
            println!("Decoded JWT: {:?}", response);
            Ok(response)
        }
        Err(err) => {
            eprintln!("Failed to decode JWT: {:?}", err);
            Err(format!("JWT decode error: {}", err))
        }
    }
}