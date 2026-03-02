use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::models::Claims;

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry: i64,  // in seconds
    pub refresh_token_expiry: i64, // in seconds
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        
        JwtConfig {
            secret,
            access_token_expiry: 3600,    // 1 hour
            refresh_token_expiry: 604800, // 7 days
        }
    }
}

pub fn generate_tokens(
    user_id: Uuid,
    email: String,
    role: String,
    config: &JwtConfig,
) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp();

    // Access token
    let access_claims = Claims {
        sub: user_id.to_string(),
        email: email.clone(),
        role: role.clone(),
        exp: now + config.access_token_expiry,
        iat: now,
        token_type: "access".to_string(),
    };

    // Refresh token
    let refresh_claims = Claims {
        sub: user_id.to_string(),
        email,
        role,
        exp: now + config.refresh_token_expiry,
        iat: now,
        token_type: "refresh".to_string(),
    };

    let encoding_key = EncodingKey::from_secret(config.secret.as_ref());

    let access_token = encode(&Header::default(), &access_claims, &encoding_key)?;
    let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)?;

    Ok((access_token, refresh_token))
}

pub fn verify_token(token: &str, config: &JwtConfig) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(config.secret.as_ref());
    let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())?;
    Ok(token_data.claims)
}

pub fn extract_token_from_header(auth_header: &str) -> Option<String> {
    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}
