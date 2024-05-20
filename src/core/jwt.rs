use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use actix_web::{dev::ServiceRequest, error, Error};

const AUTHORIZATION_HEADER: &str = "Authorization";

#[derive(Serialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: String,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub jwt_secret: String,
    pub jwt_expiration: usize,
}

#[derive(Clone)]
pub struct JwtService {
    pub jwt_config: JwtConfig,
}

impl Claims {
    /// # Summary
    ///
    /// Create a new Claims.
    ///
    /// # Arguments
    ///
    /// * `sub` - The subject of the Claims.
    /// * `exp` - The expiration time of the Claims.
    /// * `iat` - The issued at time of the Claims.
    pub fn new(sub: String, exp: usize, iat: usize) -> Claims {
        Claims { sub, exp, iat }
    }
}

impl JwtService {
    pub async fn extract(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
        if let Some(auth_header) = req.headers().get(AUTHORIZATION_HEADER) {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    match token.len() == 16 {
                        true => {
                            println!("Valid token: {}", token);
                            Ok(HashSet::<String>::new())
                        }
                        false => {
                            eprintln!("Failed to verify JWT token: {}", token);
                            Err(error::ErrorBadRequest("Invalid token"))
                        }
                    }
                } else {
                    Err(error::ErrorBadRequest("Invalid token format"))
                }
            } else {
                Err(error::ErrorBadRequest("Invalid header value"))
            }
        } else {
            Err(error::ErrorBadRequest( "Missing authorization token"))
        }
    }
}



