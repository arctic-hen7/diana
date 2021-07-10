#![forbid(unsafe_code)]
#![deny(missing_docs)]

/*!
Diana is an out-of-the-box fully-fledged GraphQL system with inbuilt support for commonly-used features like subscriptions and authentication.
It was built to allow a simple but fully-featured GraphQL system to be very quickly created for systems that have complex data structures
and no time for boilerplate.

Diana builds on the fantastic work of [async_graphql](https://crates.io/crates/async_graphql) to provide an architecture that allows you
to run queries and mutations **serverlessly**, with subscriptions running on serverful infrastructure. To achieve this, Diana uses an
integrations system, whereby the core [`DianaHandler`] logic is used to create high-level wrappers for common deployment systems like
Actix Web and AWS Lambda (including its derivatives, like Netlify). The communication between the serverless and serverful systems is done
for you, exposing a simple, automatically authenticated publishing API.

In development, Diana supports setting up one server for queries/mutations and another for subscriptions. When it comes time to go serverless,
you just change one file!

This documentation does not contain examples, which can be found in the [GitHub repository](https://github.com/diana-graphql/diana). More
detailed explanations and tutorials can be found in the [book](https://diana-graphql.github.io).
*/

mod auth;
mod diana_handler;
/// The module for errors and results. This uses [error_chain] behind the scenes.
/// You'll also find [`GQLResult`](crate::errors::GQLResult) and [`GQLError`](crate::errors::Error) in here, which may be useful in working
/// with your own resolvers.
pub mod errors;
mod graphql;
/// The module for utility functions for schema development.
pub mod graphql_utils;
mod options;
mod pubsub;

// Public exports accessible from the root (everything the user will need)
pub use crate::auth::core::{AuthBlockLevel, AuthVerdict};
pub use crate::auth::jwt::{JWTSecret, Claims, create_jwt, decode_time_str, get_jwt_secret, validate_and_decode_jwt};
pub use crate::diana_handler::{DianaHandler, DianaResponse, SysSchema};
pub use crate::options::{Options, OptionsBuilder};
pub use crate::pubsub::Publisher;

// Users shouldn't have to install `async_graphql` themselves for basic usage
#[doc(no_inline)]
pub use async_graphql;
// Other stuff users shouldn't have to install for basic use
#[doc(no_inline)]
pub use async_stream::stream; // The `stream!` macro
#[doc(no_inline)]
pub use tokio_stream::{Stream, StreamExt}; // For subscriptions
