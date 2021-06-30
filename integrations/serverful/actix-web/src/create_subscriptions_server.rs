use diana::{
    DianaHandler, Options, AuthCheckBlockState,
    errors::*
};
use actix_web::{
    guard,
    web::{self, ServiceConfig},
    HttpResponse,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    ObjectType, SubscriptionType,
};
use std::any::Any;

use crate::auth_middleware::AuthCheck;
use crate::routes::{graphql_for_subscriptions, graphql_ws};

pub fn create_subscriptions_server<C, Q, M, S>(
    opts: Options<C, Q, M, S>,
) -> Result<impl FnOnce(&mut ServiceConfig) + Clone>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    // Create a new Diana handler (core logic primitive)
    let diana_handler = DianaHandler::new(opts.clone())?;

    // Get the appropriate authentication middleware set up with the JWT secret
    // This will wrap the GraphQL endpoint itself
    let auth_middleware = match opts.authentication_block_state {
        AuthCheckBlockState::AllowAll => AuthCheck::new(&diana_handler),
        AuthCheckBlockState::AllowMissing => AuthCheck::new(&diana_handler),
        AuthCheckBlockState::BlockUnauthenticated => {
            AuthCheck::new(&diana_handler)
        }
    };

    let graphql_endpoint = opts.graphql_endpoint;
    let playground_endpoint = opts.playground_endpoint;

    // Actix Web allows us to configure apps with `.configure()`, which is what the user will do
    // Now we create the closure that will configure the user's app to support a GraphQL server
    let configurer = move |cfg: &mut ServiceConfig| {
        // Add everything except for the playground endpoint (which may not even exist)
        cfg.data(diana_handler.clone()) // Clone the full DianaHandler we got before and provide it here
            // The primary GraphQL endpoint for queries and mutations
            .service(
                web::resource(&graphql_endpoint)
                    .guard(guard::Post()) // Should accept POST requests
                    .wrap(auth_middleware.clone())
                    .to(graphql_for_subscriptions::<C, Q, M, S>), // The handler function it should use
            )
            // The GraphQL endpoint for subscriptions over WebSockets
            .service(
                web::resource(&graphql_endpoint)
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_ws::<C, Q, M, S>),
            );

        // Define the closure for the GraphiQL endpoint
        // We don't do this in `routes` because of annoying type annotations
        let graphql_endpoint_for_closure = graphql_endpoint; // We need this because `move`
        let graphiql_closure = move || {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(playground_source(
                    GraphQLPlaygroundConfig::new(&graphql_endpoint_for_closure)
                        .subscription_endpoint(&graphql_endpoint_for_closure),
                ))
        };

        // Set up the endpoint for the GraphQL playground
        match playground_endpoint {
            // If we're in development and it's enabled, set it up without authentication
            Some(playground_endpoint) if cfg!(debug_assertions) => {
                cfg.service(
                    web::resource(playground_endpoint)
                        .guard(guard::Get())
                        .to(graphiql_closure), // The playground needs to know where to send its queries
                );
            }
            // This shouldn't be possible (playground in production), see `.finish()` in `options.rs`
            Some(_) => (),
            None => (),
        };
        // This closure works entirely with side effects, so we don't need to return anything here
    };

    Ok(configurer)
}