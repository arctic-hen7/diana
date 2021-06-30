#![forbid(unsafe_code)]
#![deny(missing_docs)]

/*!
Diana is an out-of-the-box fully-fledged GraphQL system with inbuilt support for commonly-used features like subscriptions and authentication.
It was built to allow a simple but fully-featured GraphQL system to be very quickly created for systems that have complex data structures
and no time for boilerplate.

Diana's greatest feature is that it provides opinionated deployment methods that just work. You can deploy Diana with Actix Web (support for
alternative server libraries coming soon) or as a serverless function on any system that gives you a request body and access to HTTP headers,
with prebuilt deployment support for AWS Lambda and its derivatives (like Netlify). However, serverless functions cannot run subscriptions,
so Diana provides a subscriptions server system that can be run externally to the serverless function, allowing you to minimise hosting costs.
The communication between the two is supported fully out-of-the-box authenticated by JWTs.

In development, Diana supports setting up one server for queries/mutations and another for subscriptions. When it comes time to go serverless,
you just change one file!

Diana is built as a high-level wrapper around [async_graphql](https://docs.rs/async-graphql), and uses it for all internal GraphQL operations.

All examples on how to use Diana and further documentation are in the book (under construction).

# Installation

This crate is [on crates.io](https://crates.io/crates/diana) and can be used by adding `diana` to your dependencies in your project's
`Cargo.toml` like so:

```toml
[dependencies]
diana = "0.1.0"
```
*/

// This crate has a library architecture for easier testing and code reuse, with binaries that make use of the library in the server crates

mod auth;
mod diana_handler;
/// The module for errors and results. This uses [error_chain] behind the scenes.
/// You'll also find [`GQLResult`](crate::errors::GQLResult) and [`GQLError`](crate::errors::Error) in here, which may be useful in working with your own resolvers.
pub mod errors;
mod graphql;
/// The module for utility functions useful when developing your own schemas.
pub mod graphql_utils; // Users need to be able to access these of course in their schemas
mod options;
mod pubsub;

// Public exports accessible from the root (everything the user will need)
pub use crate::auth::core::AuthVerdict;
pub use crate::auth::jwt::{create_jwt, decode_time_str, get_jwt_secret, validate_and_decode_jwt};
pub use crate::diana_handler::{DianaHandler, DianaResponse};
pub use crate::options::{AuthCheckBlockState, Options, OptionsBuilder};
pub use crate::pubsub::Publisher;

// Users shouldn't have to install `async_graphql` themselves for basic usage
#[doc(no_inline)]
pub use async_graphql;
// Other stuff users shouldn't have to install for basic use
pub use async_stream::stream; // The `stream!` macro
pub use tokio_stream::{Stream, StreamExt}; // For subscriptions
