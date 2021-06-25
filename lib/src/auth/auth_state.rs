use std::collections::HashMap;

use crate::auth::jwt::Claims;

// Allows us to formally access all the authentication data
#[derive(Debug, Clone)]
pub struct AuthToken(pub Claims);

// The three states authentication can be in
// This is copyable because we need to be able to take it entirely out of the request extensions after we've put it in
#[derive(Debug, Clone)]
pub enum AuthState {
    Authorised(AuthToken),
    InvalidToken,
    NoToken
}

impl AuthState {
    // Checks the claims set in the token by the user
    // This accepts a HashMap of `&str` for convenience of writing
    pub fn has_claims(&self, test_claims: HashMap<&str, &str>) -> bool {
        if let Self::Authorised(AuthToken(Claims {
            claims,
            ..
        })) = self {
            for (key, val) in &test_claims {
                if claims.get(&key.to_string()) != Some(&val.to_string()) {
                    return false
                }
            }

            true // If we're here everything's passed
        } else {
            false
        }
    }
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Authorised(_))
    }
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::InvalidToken)
    }
    pub fn has_no_token(&self) -> bool {
        matches!(self, Self::NoToken)
    }
}
