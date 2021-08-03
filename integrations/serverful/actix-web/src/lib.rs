#![forbid(unsafe_code)]
#![deny(missing_docs)]

/*!
This is [Diana's](https://arctic-hen7.github.io) integration crate for Actix Web, which enables the easy deployment of a Diana system
on that platform. For more information, see [the documentation for Diana](https://github.com/arctic-hen7/diana) and
[the book](https://arctic-hen7.github.io).
*/

mod auth_middleware;
mod create_graphql_server;
mod create_subscriptions_server;
mod routes;

pub use crate::create_graphql_server::create_graphql_server;
pub use crate::create_subscriptions_server::create_subscriptions_server;

// Users shouldn't have to install Actix Web themselves for basic usage
#[doc(no_inline)]
pub use actix_web;
