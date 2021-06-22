// This file contains routes that are common between the GraphQL and subscriptions servers
// The GraphQL and GraphQL WS routes would require generic functions as arguments to work, and so are left as they are

use async_graphql::{
    Schema, ObjectType, SubscriptionType,
    http::{playground_source, GraphQLPlaygroundConfig}
};
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use async_graphql_actix_web::{Request, Response};

use crate::auth::auth_state::AuthState;

const GRAPHQL_ENDPOINT: &str = "/graphql";

// The endpoint for the development graphical playground
pub async fn graphiql() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(GRAPHQL_ENDPOINT).subscription_endpoint(GRAPHQL_ENDPOINT),
        )))
}

// The main GraphQL endpoint for queries and mutations with authentication support
// This handler does not support subscriptions
pub async fn graphql<Q, M, S>(
    schema: web::Data<Schema<Q, M, S>>,
    http_req: HttpRequest,
    req: Request,
) -> Response
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static
{
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
