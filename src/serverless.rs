// This file contains all the logic for setting up a serverless system for queries/mutations
// This exposes more generic primitives that the serverless systems not derived from AWS Lambda can depend on
// If a user wants to use some fancy new serverless system that doesn't even have requests, they can use this as long as they have a request body and auth header!

use async_graphql::{ObjectType, Request, SubscriptionType};
use std::any::Any;

use crate::auth::middleware::{get_auth_verdict, get_token_state_from_header, AuthVerdict};
use crate::graphql::get_schema_without_subscriptions;
use crate::options::Options;

/// The response from a generic serverless request invocation. This is the primitive you'll likely process into some kind of custom response
/// if you're not using AWS Lambda or one of its derivatives (e.g. Netlify). If you are, use [run_aws_req](crate::run_aws_req) and ignore this!
pub enum ServerlessResponse {
    /// The request was successful and the response is attached.
    /// Return a 200.
    Success(String),
    /// The request was blocked (unauthorized).
    /// Return a 403.
    Blocked,
    /// An error occurred on the server side. Any GraphQL errors will be encapsulated in the `Success` variant's payload.
    /// Return a 500.
    Error,
}

// Runs the given GraphQL query/mutation
// Needs the user's query/mutation and their authentication token
// This function handles all possible errors internally and will gracefully return an instance of `ServerlessResponse` every time
// If you're using AWS or a derivative (like Netlify), you can use `run_lambda_req` instead for greater convenience

/// Runs a GraphQL query/mutation in a serverless function. This is deliberately as general as possible so we support basically every serverless
/// function provider. If you're using AWS Lambda or one of its derivatives (e.g. Netlify), you can use [run_aws_req](crate::run_aws_req) instead for greater
/// convenience.
/// There are no examples for the usage of this function because it's a primitive. Just provide a request body (which should contain the query,
/// variables, etc.) and the HTTP `Authorization` header. You don't need to do any further processing, this function will do the rest.
pub async fn run_serverless_req<C, Q, M, S>(
    body: String,
    raw_auth_header: Option<&str>,
    opts: Options<C, Q, M, S>,
) -> ServerlessResponse
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    // Get the schema (this also creates a publisher to the subscriptions server and inserts context)
    // We deal with any errors directly with the serverless response enum
    let schema =
        get_schema_without_subscriptions(opts.schema, opts.subscriptions_server_data, opts.ctx);
    let schema = match schema {
        Ok(schema) => schema,
        Err(_) => return ServerlessResponse::Error,
    };

    // Get a verdict on whether or not the user should be allowed through
    let token_state = get_token_state_from_header(raw_auth_header, opts.jwt_secret);
    let verdict = get_auth_verdict(token_state, opts.authentication_block_state);

    match verdict {
        AuthVerdict::Allow(auth_data) => {
            // Deserialise that raw JSON request into an actual request with variables etc.
            let gql_req = serde_json::from_str::<Request>(&body);
            let mut gql_req = match gql_req {
                Ok(gql_req) => gql_req,
                Err(_) => return ServerlessResponse::Error,
            };
            // Insert the authentication data directly into that
            gql_req = gql_req.data(auth_data);
            // Run the request
            let res = schema.execute(gql_req).await;
            // Serialise that response into a string (the response bodies all have to be of the same type)
            let res_str = serde_json::to_string(&res);
            let res_str = match res_str {
                Ok(res_str) => res_str,
                Err(_) => return ServerlessResponse::Error,
            };

            ServerlessResponse::Success(res_str)
        }
        AuthVerdict::Block => ServerlessResponse::Blocked,
        AuthVerdict::Error => ServerlessResponse::Error,
    }
}
