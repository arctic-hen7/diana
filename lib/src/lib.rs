#![forbid(unsafe_code)]
// This crate has a library architecture for easier testing and code reuse, with binaries that make use of the library in the server crates

// These will eventually be removed from the core code (MongoDB-related dev utils)
mod oid;
pub mod db;

pub mod errors;
pub mod load_env;
pub mod schemas;
mod graphql;
mod pubsub;
mod options;
pub mod graphql_utils;
mod graphql_server;
mod subscriptions_server;
mod auth;
mod routes;

// Public exports accessible from the root (everything the user will need)
pub use crate::graphql_server::create_graphql_server;
pub use crate::subscriptions_server::create_subscriptions_server;
pub use crate::options::{Options, OptionsBuilder, AuthCheckBlockState};

// Users shouldn't have to install Actix Web themselves for basic usage
pub use actix_web::{App, HttpServer};
