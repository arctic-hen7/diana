#![forbid(unsafe_code)]
#![deny(missing_docs)]

/*!
This is [Diana's](https://arctic-hen7.github.io) integration crate for AWS Lambda and its derivatives (like Netlify), which enables the
easy deployment of a Diana system on those platforms. For more information, see
[the documentation for Diana](https://github.com/arctic-hen7/diana) and [the book](https://arctic-hen7.github.io).

This crate can be used to create handlers for AWS Lambda itself, or any system that wraps it, like Netlify. Handlers created with this crate
will compile, but will not run without being deployed fully. In development, you should use something like Actix Web (and
[its Diana integration](https://crates.io/crates/diana-actix-web)) to deploy a serverful system for queries and mutations, which you can more
easily work with. When you're ready, you can switch to this crate without changing any part of your schema logic. Full examples are in the book.
*/

mod run_aws_req;

pub use crate::run_aws_req::{run_aws_req, AwsError};

// Users also shouldn't have to install the Netlify stuff themselves for basic usage
pub use netlify_lambda_http;
