#![forbid(unsafe_code)]
// This crate has a library architecture for easier testing and code reuse, with binaries that make use of the library in the server crates

mod auth;
mod aws_serverless;
pub mod errors;
mod graphql;
mod graphql_server;
pub mod graphql_utils; // Users need to be able to access these of course in their schemas
mod options;
mod pubsub;
mod routes;
mod serverless;
mod subscriptions_server;

// Public exports accessible from the root (everything the user will need)
pub use crate::aws_serverless::{run_aws_req, AwsError};
pub use crate::graphql_server::create_graphql_server;
pub use crate::options::{AuthCheckBlockState, Options, OptionsBuilder};
pub use crate::pubsub::Publisher;
pub use crate::serverless::{run_serverless_req, ServerlessResponse};
pub use crate::subscriptions_server::create_subscriptions_server;

// Users shouldn't have to install Actix Web themselves for basic usage
pub use actix_web::{App, HttpServer};
// Users also shouldn't have to install the Netlify stuff themselves for basic usage
pub use netlify_lambda_http::{
    handler as create_handler,
    lambda::{run as run_lambda, Context as LambdaCtx},
    IntoResponse as IntoLambdaResponse, Request as LambdaRequest,
};
