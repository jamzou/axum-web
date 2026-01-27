use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // Subject (user identifier)
    pub exp: usize,           // Expiration time (as UTC timestamp)
    pub iat: usize,           // Issued at (as UTC timestamp)
    pub role: String,         // User role
}

impl Claims {
    pub fn new(user_id: u32, _user_name: String, role: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;
            
        // Access token expires in 1 hour
        let exp = now + 3600;
        
        Claims {
            sub: user_id.to_string(),
            exp,
            iat: now,
            role,
        }
    }
}

pub fn generate_access_token(user_id: u32, user_name: String, role: String) -> Result<String> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
    let claims = Claims::new(user_id, user_name, role);
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

#[allow(unused)]
pub fn verify_token(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        std::env::set_var("JWT_SECRET", "test_secret");
        
        let token = generate_access_token(1, "test_user".to_string(), "admin".to_string()).unwrap();
        let claims = verify_token(&token).unwrap();
        
        assert_eq!(claims.sub, "1");
        assert_eq!(claims.role, "admin");
    }
}