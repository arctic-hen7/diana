#![forbid(unsafe_code)]
// This binary runs a serverful setup with Actix Web, as opposed to a serverless approach (TODO)
// Even so, this system does NOT support subscriptions so we maintain the separity in development that will be present in production

use async_graphql_actix_web::{Request, Response};
use actix_web::{guard, web, App, HttpServer, HttpRequest};
use lib::{
    load_env,
    AppSchemaWithoutSubscriptions as AppSchema,
    get_schema_without_subscriptions as get_schema,
    routes::{
        graphiql
    },
    auth::{
        middleware::AuthCheck,
        auth_state::{AuthState}
    }
};

const GRAPHIQL_ENDPOINT: &str = "/graphiql"; // For the graphical development playground
const GRAPHQL_ENDPOINT: &str = "/graphql";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env().expect("Error getting environment variables!");
    // We get the schema once and then use it for all queries
    // If this fails, we can't do anything at all
    let schema = get_schema().expect("Failed to fetch schema.");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource(GRAPHQL_ENDPOINT)
                .guard(guard::Post())
                .wrap(AuthCheck::block_unauthenticated())
                .to(graphql)
            ) // POST endpoint for queries/mutations
            // This system emulates the serverless one, and thus does not support subscriptions
            .service(web::resource(GRAPHIQL_ENDPOINT).guard(guard::Get()).to(graphiql)) // GET endpoint for GraphiQL playground (unauthenticated because it's only for development)
    })
    .bind("0.0.0.0:7000")? // This stays the same, that port in the container will get forwarded to whatever's configured in `.ports.env`
    .run()
    .await
}

async fn graphql(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    req: Request,
) -> Response {
    // Get the GraphQL request so we can add data to it
    let mut query = req.into_inner();
    // Get the authorisation data from the request extensions if it exists (it would be set by the middleware)
    let extensions = http_req.extensions();
    let auth_data = extensions.get::<AuthState>();

    // Clone the internal AuthState so we can place the variable into the context (lifetimes...)
    let auth_data_for_ctx = auth_data.map(|auth_data| auth_data.clone());
    // Add that to the GraphQL request data so we can access it in the resolvers
    query = query.data(auth_data_for_ctx);
    schema.execute(query).await.into()
}
