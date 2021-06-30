// This example illustrates how to set up a queries/mutations server for development
// Note that this is literally identical to the `subscriptions_server.rs` example, just using `create_graphql_server` instead and a different port

#![forbid(unsafe_code)]

use diana_actix_web::{
    actix_web::{App, HttpServer},
    create_graphql_server,
};

// This 'dirty-imports' the code in `schema.in`
// It will literally be interpolated here
// Never use this in production unless you have a fantastic reason! Just import your code through Cargo!
// We do this here though because you can't import from another example (which is annoying)
include!("../../../../examples/schema/schema.rs");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configurer = create_graphql_server(get_opts()).expect("Failed to set up configurer!");

    HttpServer::new(move || App::new().configure(configurer.clone()))
        .bind("0.0.0.0:9001")?
        .run()
        .await
}
