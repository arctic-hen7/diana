// This file contains routes that are common between the GraphQL and subscriptions servers
// The GraphQL and GraphQL WS routes would require generic functions as arguments to work, and so are left as they are

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix_web::{HttpResponse, Result as ActixResult};

const GRAPHQL_ENDPOINT: &str = "/graphql";

// TODO only compile this in development
// The endpoint for the development graphical playground
pub async fn graphiql() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(GRAPHQL_ENDPOINT).subscription_endpoint(GRAPHQL_ENDPOINT),
        )))
}
