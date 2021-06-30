use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use async_graphql::{ObjectType, SubscriptionType};
use futures::{
    future::{ok, Ready},
    Future,
};
use diana::{DianaHandler, AuthVerdict};
use std::any::Any;
use std::pin::Pin;
use std::result::Result as StdResult;
use std::task::{Context, Poll};

// Create a factory for authentication middleware
#[derive(Clone)]
pub struct AuthCheck<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    diana_handler: DianaHandler<C, Q, M, S>,
}
impl<C, Q, M, S> AuthCheck<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    // Initialises a new instance of the authentication middleware factory by cloning the given DianaHandler
    pub fn new(diana_handler: &DianaHandler<C, Q, M, S>) -> Self {
        Self {
            diana_handler: diana_handler.clone()
        }
    }
}

// This is what we'll actually call, all it does is create the middleware and define all its properties
impl<C, Q, M, Sb, S> Transform<S> for AuthCheck<C, Q, M, Sb>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    Sb: Clone + SubscriptionType + 'static,
    S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    // All the properties of the middleware need to be defined here
    // We could do this with `wrap_fn` instead, but this approach gives far greater control
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthCheckMiddleware<C, Q, M, Sb, S>;
    type Future = Ready<StdResult<Self::Transform, Self::InitError>>;

    // This will be called internally by Actix Web to create our middleware
    // All this really does is pass the service itself (handler basically) over to our middleware (along with additional metadata)
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthCheckMiddleware {
            diana_handler: self.diana_handler.clone(),
            service,
        })
    }
}

// The actual middleware
#[derive(Clone)]
pub struct AuthCheckMiddleware<C, Q, M, Sb, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    Sb: Clone + SubscriptionType + 'static,
{
    diana_handler: DianaHandler<C, Q, M, Sb>,
    service: S,
}

impl<C, Q, M, Sb, S> Service for AuthCheckMiddleware<C, Q, M, Sb, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    Sb: Clone + SubscriptionType + 'static,
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
        // Get the HTTP `Authorization` header
        let auth_header = req.headers().get("AUTHORIZATION").map(|auth_header| {
            // We convert to a string and handle the result, which gives us an Option inside an Option
            let header_str = auth_header.to_str();
            match header_str {
                Ok(header_str) => Some(header_str),
                Err(_) => None
            }
        }).flatten(); // Then we flatten the two Options into one Option
        // Get a verdict and match that to a middleware outcome
        let verdict = self.diana_handler.is_authed(auth_header);
        match verdict {
            auth_verdict @ AuthVerdict::Allow(_)  => {
                // Insert the authentication verdict into the request extensions for later retrieval
                req.extensions_mut().insert(auth_verdict);
                // Move on from this middleware to the handler
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            AuthVerdict::Block => {
                // Return a 403
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,                    // Eliminates the payload of the request
                        HttpResponse::Unauthorized().finish(), // In the playground this will come up as bad JSON, it's a direct HTTP response
                    ))
                })
            }
            AuthVerdict::Error(_) => {
                // Middleware failed, we shouldn't let this proceed to the request just in case
                // This error could be triggered by a failure in transforming the token from base64, meaning the error can be caused forcefully by an attacker
                // In that scenario, we can't allow the bypassing of this layer
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0, // Eliminates the payload of the request
                        HttpResponse::InternalServerError().finish(),
                    ))
                })
            }
        }
    }
}
