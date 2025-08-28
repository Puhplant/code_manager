use chrono::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user id)
    pub exp: i64,     // Expiration time
    pub iat: i64,     // Issued at
    pub account_id: i32,
}

pub struct JwtAuth {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtAuth {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn create_token(&self, user_id: &str, account_id: i32) -> Result<(String, DateTime<Utc>), jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expires_at = now + Duration::minutes(60);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            account_id: account_id,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok((token, expires_at))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
} 