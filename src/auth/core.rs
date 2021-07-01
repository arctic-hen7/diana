// This file contains the core authentication logic that will be used regardless of integration

use crate::auth::auth_state::{AuthState, AuthToken};
use crate::auth::jwt::{get_jwt_secret, validate_and_decode_jwt};
use crate::errors::*;

/// An enum for the level of blocking imposed on a particular endpoint.
/// Your choice on this should be carefully evaluated based on your threat model. Please choose wisely!
#[derive(Debug, Clone, Copy)]
pub enum AuthBlockLevel {
    /// Allows anything through.
    /// - Valid token   -> allow
    /// - Invalid token -> allow
    /// - Missing token -> allow
    AllowAll,
    /// Blocks eveything except requests with valid tokens.
    /// Note that, with this setting, introspection will be impossible in the GraphiQL playground. You may want to use `AllowMissing` in development
    /// and then this in production (see the book).
    /// - Valid token   -> allow
    /// - Invalid token -> block
    /// - Missing token -> block
    BlockUnauthenticated,
    /// Allows requests with valid tokens or no token at all. Only blocks requests that specify an invalid token.
    /// This is mostly useful for development to enable introspection in the GraphiQL playground (see the book).
    /// - Valid token   -> allow
    /// - Invalid token -> block
    /// - Missing token -> allow
    AllowMissing,
}

// Extracts an authentication state from the given Option<String> token
// This is exposed as a primitive for serverful and serverless authentication logic
pub fn get_token_state_from_header(
    auth_header: Option<&str>,
    secret_str: String,
) -> Result<AuthState> {
    // Get the bearer token from the header if it exists
    let bearer_token = match auth_header {
        Some(header) => header
            .split("Bearer")
            .collect::<Vec<&str>>()
            .get(1) // Get everything apart from that first element
            .map(|token| token.trim()),
        None => None,
    };

    // Decode the bearer token into an authentication state
    match bearer_token {
        Some(token) => {
            let jwt_secret = get_jwt_secret(secret_str)?;
            let decoded_jwt = validate_and_decode_jwt(&token, &jwt_secret);

            match decoded_jwt {
                Some(claims) => Ok(AuthState::Authorised(AuthToken(claims))),
                None => Ok(AuthState::InvalidToken), // The token is invalid
            }
        }
        None => Ok(AuthState::NoToken), // No token exists
    }
}

/// This represents the decision as to whether or not a use is allowed through to an endpoint. You should only have to deal with this if you're
/// developing middleware for a custom integration.
#[derive(Clone, Debug)]
pub enum AuthVerdict {
    /// The user should be allowed through, and their decoded authentication data (JWT payload without metadata) is attached.
    Allow(AuthState),
    /// The user should be blocked.
    Block,
    /// Some internal error occurred, the body of which is attached.
    Error(String),
}

// Compares the given token's authentication state (as a raw result) to a given block-level to arrive at a verdict
pub fn get_auth_verdict(
    token_state: Result<AuthState>,
    block_state: AuthBlockLevel,
) -> AuthVerdict {
    match token_state {
        // We hold `token_state` as the AuthState variant so we don't pointlessly insert a Result into the request extensions
        Ok(token_state @ AuthState::Authorised(_)) => AuthVerdict::Allow(token_state),
        Ok(token_state @ AuthState::InvalidToken) => {
            if let AuthBlockLevel::AllowAll = block_state {
                AuthVerdict::Allow(token_state)
            } else {
                AuthVerdict::Block
            }
        }
        Ok(token_state @ AuthState::NoToken) => {
            if let AuthBlockLevel::AllowAll | AuthBlockLevel::AllowMissing = block_state {
                AuthVerdict::Allow(token_state)
            } else {
                AuthVerdict::Block
            }
        }
        Err(err) => AuthVerdict::Error(err.to_string()),
    }
}
