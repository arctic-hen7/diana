// This file contains all the logic for setting up a serverless system for queries/mutations
// This is designed to be used with the `netlify_lambda_http` crate, but it exposes more generic primitives as well
// If a user wants to use some fancy new serverless system that doesn't even have requests, they can use this as long as they have a request body and auth header!

use std::any::Any;
use async_graphql::{
    ObjectType, SubscriptionType, EmptySubscription,
    http::{playground_source, GraphQLPlaygroundConfig},
    Request
};
use actix_web::{
    web::{self, ServiceConfig}, guard, HttpResponse,
};

use crate::graphql::{get_schema_without_subscriptions};
use crate::auth::middleware::{AuthCheck, AuthVerdict, get_token_state_from_header, get_auth_verdict};
use crate::routes::{graphql};
use crate::options::{Options, AuthCheckBlockState};
use crate::errors::*;

pub enum ServerlessResponse {
    Success(String),
    Blocked,
    Error
}

// Runs the given GraphQL query/mutation
// Needs the user's query/mutation and their authentication token
// This function handles all possible errors internally and will gracefully return an instance of `ServerlessResponse` every time
// If you're using AWS or a derivative (like Netlify), you can use `run_lambda_req` instead for greater convenience
pub async fn run_serverless_req<C, Q, M, S>(
    body: String,
    raw_auth_header: Option<&str>,
    opts: Options<C, Q, M, S>
) -> ServerlessResponse
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static
{
    // Get the schema (this also creates a publisher to the subscriptions server and inserts context)
    // We deal with any errors directly with the serverless response enum
    let schema = get_schema_without_subscriptions(opts.schema, opts.subscriptions_server_data, opts.ctx);
    let schema = match schema {
        Ok(schema) => schema,
        Err(_) => return ServerlessResponse::Error
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
                Err(_) => return ServerlessResponse::Error
            };
            // Insert the authentication data directly into that
            gql_req = gql_req.data(auth_data);
            // Run the request
            let res = schema.execute(gql_req).await;
            // Serialise that response into a string (the response bodies all have to be of the same type)
            let res_str = serde_json::to_string(&res);
            let res_str = match res_str {
                Ok(res_str) => res_str,
                Err(_) => return ServerlessResponse::Error
            };

            ServerlessResponse::Success(res_str)
        },
        AuthVerdict::Block => ServerlessResponse::Blocked,
        AuthVerdict::Error => ServerlessResponse::Error
    }
}

pub fn create_graphql_server<C, Q, M, S>(opts: Options<C, Q, M, S>) -> Result<
    impl FnOnce(&mut ServiceConfig) + Clone
>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static
{
    // Get the schema (this also creates a publisher to the subscriptions server and inserts context)
    let schema = get_schema_without_subscriptions(opts.schema, opts.subscriptions_server_data, opts.ctx)?;
    // Get the appropriate authentication middleware set up with the JWT secret
    // This will wrap the GraphQL endpoint itself
    let auth_middleware = match opts.authentication_block_state {
        AuthCheckBlockState::AllowAll => AuthCheck::new(&opts.jwt_secret).allow_all(),
        AuthCheckBlockState::AllowMissing => AuthCheck::new(&opts.jwt_secret).allow_missing(),
        AuthCheckBlockState::BlockUnauthenticated => AuthCheck::new(&opts.jwt_secret).block_unauthenticated()
    };

    let graphql_endpoint = opts.graphql_endpoint;
    let playground_endpoint = opts.playground_endpoint;

    // Actix Web allows us to configure apps with `.configure()`, which is what the user will do
    // Now we create the closure that will configure the user's app to support a GraphQL server
    let configurer = move |cfg: &mut ServiceConfig| {
        // Add everything except for the playground endpoint (which may not even exist)
        cfg
            .data(schema.clone()) // Clone the full schema we got before and provide it here
            // The primary GraphQL endpoint for queries and mutations
            .service(web::resource(&graphql_endpoint)
                .guard(guard::Post()) // Should accept POST requests
                .wrap(auth_middleware.clone()) // Should be authenticated
                .to(graphql::<Q, M, EmptySubscription>) // The handler function it should use
            );

        // Define the closure for the GraphiQL endpoint
        // We don't do this in routes because of annoying type annotations
        let graphql_endpoint_for_closure = graphql_endpoint; // We need this because moving
        let graphiql_closure = move || {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(playground_source(
                    GraphQLPlaygroundConfig::new(&graphql_endpoint_for_closure).subscription_endpoint(&graphql_endpoint_for_closure),
                ))
        };

        // Set up the endpoint for the GraphQL playground
        match playground_endpoint {
            // If we're in development and it's enabled, set it up without authentication
            Some(playground_endpoint) if cfg!(debug_assertions) => {
                cfg
                    .service(web::resource(playground_endpoint)
                        .guard(guard::Get())
                        .to(graphiql_closure) // The playground needs to know where to send its queries
                    );
            },
            // If we're in production and it's enabled, set it up with authentication
            // The playground doesn't process the auth headers, so the token just needs to be valid (no further access control yet)
            Some(playground_endpoint) => {
                cfg
                    .service(web::resource(playground_endpoint)
                        .guard(guard::Get())
                        // TODO by request, the JWT secret and block level can be different here
                        .wrap(auth_middleware.clone())
                        .to(graphiql_closure) // The playground needs to know where to send its queries
                    );
            },
            None => ()
        };
        // This closure works entirely with side effects, so we don't need to return anything here
    };


    Ok(configurer)
}
