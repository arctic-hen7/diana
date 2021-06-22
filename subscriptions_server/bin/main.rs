#![forbid(unsafe_code)]
// This binary runs a serverful system with Actix Web for serving subscriptions, designed for production usage
// If your system doesn't use subscriptions, this binary is irrelevant for you
// Serverless functions cannot handle subscriptions because they would need to hold WebSocket connections, which are stateful,
// so we need to use a server. We could use an intermediary, but that's generally more complex and expensive.
// AWS AppSync is a popular solution, though this system deploys on Netlify, and so that's useless here (though it could be modified for AWS)

use std::env;
use lib::{
    schemas::users::{Query, Mutation, Subscription},
    graphql_utils::Context,
    db::DbPool,
    load_env::load_env,

    App, HttpServer,
    create_subscriptions_server, OptionsBuilder, AuthCheckBlockState
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env().expect("Failed env.");

    let opts = OptionsBuilder::new()
                    .ctx(Context {
                        pool: DbPool::default()
                    })
                    .subscriptions_server_hostname("http://subscriptions-server")
                    .subscriptions_server_port("6000")
                    .subscriptions_server_endpoint("/graphql")
                    .jwt_to_connect_to_subscriptions_server(&env::var("SUBSCRIPTIONS_SERVER_PUBLISH_JWT").unwrap())
                    .auth_block_state(AuthCheckBlockState::AllowAll)
                    .jwt_secret(&env::var("JWT_SECRET").unwrap())
                    .schema(Query {}, Mutation {}, Subscription {})
                    // Endpoints are set up as `/graphql` and `/graphiql` automatically
                    .finish().expect("Options building failed!");

    let configurer = create_subscriptions_server(opts);

    HttpServer::new(move || {
        App::new()
            .configure(configurer.clone())
    })
    .bind("0.0.0.0:6000")? // This stays the same, that port in the container will get forwarded to whatever's configured in `.ports.env`
    .run()
    .await
}
