use std::collections::HashMap;

use crate::auth::jwt::Claims;
use crate::errors::*;

/// An authentication token with claims.
#[derive(Debug, Clone)]
pub struct AuthToken(pub Claims);

/// The three states authentication can be in at the token level.
#[derive(Debug, Clone)]
pub enum AuthState {
    /// The user is authorized, authentication data is attached.
    Authorised(AuthToken),
    /// An invalid token was provided.
    InvalidToken,
    /// No token was provided.
    NoToken,
}
impl AuthState {
    /// Checks if the each key/value pair in the given `HashMap` is present in the token. This will return false if the token was invalid
    /// or not provided.
    pub fn has_claims(&self, test_claims: HashMap<&str, &str>) -> bool {
        if let Self::Authorised(AuthToken(Claims { claims, .. })) = self {
            for (key, val) in &test_claims {
                if claims.get(&key.to_string()) != Some(&val.to_string()) {
                    return false;
                }
            }

            true // If we're here everything's passed
        } else {
            false
        }
    }
    /// Checks if the token is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Authorised(_))
    }
    /// Checks if the token is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::InvalidToken)
    }
    /// Checks if the token is not present.
    pub fn has_no_token(&self) -> bool {
        matches!(self, Self::NoToken)
    }
    /// Gets a reference to the claims of the token (including metadata like expiry).
    pub fn get_claims(&self) -> Result<&Claims> {
        match self {
            Self::Authorised(AuthToken(claims)) => Ok(claims),
            _ => bail!(ErrorKind::Unauthorised),
        }
    }
}
