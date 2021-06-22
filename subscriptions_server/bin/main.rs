#![forbid(unsafe_code)]
// This binary runs a serverful system with Actix Web for serving subscriptions, designed for production usage
// If your system doesn't use subscriptions, this binary is irrelevant for you
// Serverless functions cannot handle subscriptions because they would need to hold WebSocket connections, which are stateful,
// so we need to use a server. We could use an intermediary, but that's generally more complex and expensive.
// AWS AppSync is a popular solution, though this system deploys on Netlify, and so that's useless here (though it could be modified for AWS)

use std::sync::Mutex;
use async_graphql_actix_web::{Request, Response, WSSubscription};
use async_graphql::Schema;
use actix_web::{
    guard, web, App, HttpServer, HttpRequest, HttpResponse, Result as ActixResult,
};
use lib::{
    load_env,
    AppSchemaForSubscriptions as AppSchema,
    get_schema_for_subscriptions as get_schema,
    PubSub,
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
    // The subscriptions schema can't fail (it doesn't need a potentially problematic Publisher instance)
    let schema = get_schema();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(Mutex::new(PubSub::default()))
            .service(web::resource(GRAPHQL_ENDPOINT)
                .guard(guard::Post())
                .wrap(AuthCheck::block_unauthenticated())
                .to(graphql)
            ) // POST endpoint for queries/mutations
            .service(web::resource(GRAPHQL_ENDPOINT)
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(graphql_ws)
            ) // WebSocket endpoint for subscriptions
            .service(web::resource(GRAPHIQL_ENDPOINT).guard(guard::Get()).to(graphiql)) // GET endpoint for GraphiQL playground (unauthenticated because it's only for development)
    })
    .bind("0.0.0.0:6000")? // This stays the same, that port in the container will get forwarded to whatever's configured in `.ports.env`
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

async fn graphql_ws(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    payload: web::Payload,
) -> ActixResult<HttpResponse> {
    WSSubscription::start(Schema::clone(&schema), &http_req, payload)
}
