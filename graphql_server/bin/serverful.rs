#![forbid(unsafe_code)]
// This binary runs a serverful setup with Actix Web, as opposed to a serverless approach (TODO)
// Even so, this system does NOT support subscriptions so we maintain the separity in development that will be present in production

use actix_web::{App, HttpServer};
use lib::{
    schemas::users::{Query, Mutation, Subscription},
    graphql_utils::Context,
    db::DbPool,

    create_graphql_server, OptionsBuilder, AuthCheckBlockState
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = OptionsBuilder::new()
                    .ctx(Context {
                        pool: DbPool::default()
                    })
                    .subscriptions_server_hostname("http://subscriptions-server")
                    .subscriptions_server_port("6000")
                    .subscriptions_server_endpoint("/graphql")
                    .jwt_to_connect_to_subscriptions_server("invalidtoken")
                    .auth_block_state(AuthCheckBlockState::AllowAll)
                    .jwt_secret("aninsecuresecret")
                    .schema(Query {}, Mutation {}, Subscription {})
                    // Endpoints are set up as `/graphql` and `/graphiql` automatically
                    .finish().expect("Options building failed!");

    let configurer = create_graphql_server(opts).expect("Failed to set up configurer.");

    HttpServer::new(move || {
        App::new()
            .configure(configurer.clone())
    })
    .bind("0.0.0.0:7000")? // This stays the same, that port in the container will get forwarded to whatever's configured in `.ports.env`
    .run()
    .await
}
