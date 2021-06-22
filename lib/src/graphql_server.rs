// Contains the logic to actually create the GraphQL server that the user will use
// This file does not include any logic for the subscriptions server

use std::any::Any;
use async_graphql::{
    ObjectType, SubscriptionType, EmptySubscription,
    http::{playground_source, GraphQLPlaygroundConfig}
};
use actix_web::{
    web::{self, ServiceConfig}, guard, HttpResponse,
};

use crate::graphql::{get_schema_without_subscriptions};
use crate::auth::middleware::AuthCheck;
use crate::routes::{graphql};
use crate::options::{Options, AuthCheckBlockState};
use crate::errors::*;

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
