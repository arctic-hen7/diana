use std::pin::Pin;
use std::task::{Context, Poll};
use std::result::Result as StdResult;
use actix_web::{
    dev::{
        Service, Transform,
        ServiceRequest, ServiceResponse
    },
    Error,
    HttpResponse,
    HttpMessage
};
use futures::{
    future::{ok, Ready},
    Future
};

use crate::errors::*;
use crate::auth::jwt::{get_jwt_secret, validate_and_decode_jwt};
use crate::auth::auth_state::{AuthState, AuthToken};

// Extracts an authentication state from the given Optio<String> token
// This is exposed as a primitive for serverful and serverless authentication logic
pub fn get_token_state_from_header(auth_header: Option<&str>, secret_str: String) -> Result<AuthState> {
    // Get the bearer token from the header if it exists
    let bearer_token = match auth_header {
        Some(header) => header.split("Bearer")
                            .collect::<Vec<&str>>()
                            .get(1) // Get everything apart from that first element
                            .map(|token| token.trim()),
        None => None
    };

    // Decode the bearer token into an authentication state
    match bearer_token {
        Some(token) => {
            let jwt_secret = get_jwt_secret(secret_str)?;
            let decoded_jwt = validate_and_decode_jwt(&token, &jwt_secret);

            match decoded_jwt {
                Some(claims) => Ok(AuthState::Authorised(
                    AuthToken(claims)
                )),
                None => Ok(AuthState::InvalidToken) // The token is invalid
            }
        }
        None => Ok(AuthState::NoToken) // No token exists
    }
}

// Extracts an authentication state from the given request
// Needs a JWT secret to validate the client's token
fn get_token_state_from_req(req: &ServiceRequest, secret_str: String) -> Result<AuthState> {
    // Get the authorisation header from the request
    let raw_auth_header = req
                            .headers()
                            .get("AUTHORIZATION");
    let header_str = match raw_auth_header {
        Some(header) => {
            let header_str = header.to_str();
            match header_str {
                Ok(header_str) => Some(header_str),
                Err(_) => None
            }
        },
        None => None
    };

    // This returns a Result already because it needs to attempt to parse the JWT secret
    get_token_state_from_header(header_str, secret_str)
}

// The final decision as to whether or not a user should be allowed through
// We need this because some things can fail
pub enum AuthVerdict {
    Allow(AuthState),
    Block,
    Error
}

// Compares the given token's authentication state (as a raw result) to a given block-level to arrive at a verdict
pub fn get_auth_verdict(token_state: Result<AuthState>, block_state: AuthCheckBlockState) -> AuthVerdict {
    match token_state {
        // We hold `token_state` as the AuthState variant so we don't pointlessly insert a Result into the request extensions
        Ok(token_state @ AuthState::Authorised(_)) => AuthVerdict::Allow(token_state),
        Ok(token_state @ AuthState::InvalidToken) => {
            if let AuthCheckBlockState::AllowAll = block_state {
                AuthVerdict::Allow(token_state)
            } else {
                AuthVerdict::Block
            }
        },
        Ok(token_state @ AuthState::NoToken) => {
            if let AuthCheckBlockState::AllowAll | AuthCheckBlockState::AllowMissing = block_state {
                AuthVerdict::Allow(token_state)
            } else {
                AuthVerdict::Block
            }
        },
        Err(_) => AuthVerdict::Error
    }
}

// The block state chosen may have unforseen security implications, please choose wisely!
#[derive(Debug, Clone, Copy)]
pub enum AuthCheckBlockState {
    AllowAll, // Allows anything through, adding the auth parameters to the request for later processing
    BlockUnauthenticated, // Blocks missing/invalid tokens (all requests must be authenticated)
    AllowMissing // Only block if an invalid token is given (if no token, allowed)
}

// Create a factory for authentication middleware
#[derive(Clone)]
pub struct AuthCheck {
    token_secret: String,
    block_state: AuthCheckBlockState // This defines whether or not we should block requests without a token or with an invalid one
}
impl AuthCheck {
    // Initialises a new instance of the authentication middleware factory
    // Needs a JWT to validate client tokens
    pub fn new(token_secret: &str) -> Self {
        Self {
            token_secret: token_secret.to_string(),
            block_state: AuthCheckBlockState::BlockUnauthenticated // We block by default
        }
    }
    // These functions allow us to initialise the middleware factory (and thus the middleware itself) with custom options
    pub fn block_unauthenticated(mut self) -> Self {
        self.block_state = AuthCheckBlockState::BlockUnauthenticated;
        self
    }
    pub fn allow_missing(mut self) -> Self {
        self.block_state = AuthCheckBlockState::AllowMissing;
        self
    }
    pub fn allow_all(mut self) -> Self {
        self.block_state = AuthCheckBlockState::AllowAll;
        self
    }
}

// This is what we'll actually call, all it does is create the middleware and define all its properties
impl<S> Transform<S> for AuthCheck
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    // All the properties of the middleware need to be defined here
    // We could do this with `wrap_fn` instead, but this approach gives far greater control
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthCheckMiddleware<S>;
    type Future = Ready<StdResult<Self::Transform, Self::InitError>>;

    // This will be called internally by Actix Web to create our middleware
    // All this really does is pass the service itself (handler basically) over to our middleware (along with additional metadata)
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthCheckMiddleware {
            token_secret: self.token_secret.clone(),
            service,
            block_state: self.block_state
        })
    }
}

// The actual middleware
#[derive(Clone)]
pub struct AuthCheckMiddleware<S> {
    token_secret: String, // The JWT secret as a string to validate client tokens
    service: S,
    block_state: AuthCheckBlockState // This will be passed in from whatever is set for the factory
}

impl<S> Service for AuthCheckMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    // More properties for Actix Web
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = StdResult<Self::Response, Self::Error>>>>;

    // Stock function for asynchronous operations
    // The context here has nothing to do with our app's internal context whatsoever!
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<StdResult<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // Check the token
        let token_state = get_token_state_from_req(&req, self.token_secret.clone());
        let verdict = get_auth_verdict(token_state, self.block_state);
        match verdict {
            AuthVerdict::Allow(token_state) => {
                // Insert the authentication data into the request extensions for later retrieval
                req.extensions_mut().insert(token_state);
                // Move on from this middleware to the handler
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            },
            AuthVerdict::Block => {
                // Return a 403
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0, // Eliminates the payload of the request
                        HttpResponse::Unauthorized().finish() // In the playground this will come up as bad JSON, it's a direct HTTP response
                    ))
                })
            },
            AuthVerdict::Error => {
                // Middleware failed, we shouldn't let this proceed to the request just in case
                // This error could be triggered by a failure in transforming the token from base64, meaning the error can be caused forcefully by an attacker
                // In that scenario, we can't allow the bypassing of this layer
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0, // Eliminates the payload of the request
                        HttpResponse::InternalServerError().finish()
                    ))
                })
            }
        }
    }
}
