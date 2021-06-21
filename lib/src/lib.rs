#![forbid(unsafe_code)]
// This crate has a library architecture for easier testing and code reuse, with binaries that make use of the library in the server crates

pub mod errors;
mod load_env;
mod db;
mod schemas;
mod graphql;
mod oid;
mod pubsub;
mod graphql_utils;
pub mod auth;
pub mod routes;

pub use crate::graphql::{
    AppSchemaWithoutSubscriptions,
    AppSchemaForSubscriptions,
    get_schema_without_subscriptions,
    get_schema_for_subscriptions,
};
pub use crate::load_env::load_env;
pub use crate::pubsub::PubSub;
