#![forbid(unsafe_code)]
// This crate has a library architecture for easier testing and code reuse, with binaries that make use of the library in the server crates

// These will eventually be removed from the core code (MongoDB-related dev utils)
mod oid;
pub mod db;

pub mod errors;
mod load_env;
pub mod schemas;
mod graphql;
mod pubsub;
mod options;
pub mod graphql_utils;
mod graphql_server;
pub mod auth;
pub mod routes;

// Public exports accessible from the root (everything the user will need)
pub use crate::graphql_server::create_graphql_server;
pub use crate::options::{Options, OptionsBuilder, AuthCheckBlockState};
pub use crate::graphql::{
    AppSchemaWithoutSubscriptions,
    AppSchemaForSubscriptions,
    get_schema_without_subscriptions,
    get_schema_for_subscriptions,
};
pub use crate::load_env::load_env;
pub use crate::pubsub::PubSub;
