// This file contains routes that are common between the GraphQL and subscriptions servers
// The GraphQL and GraphQL WS routes would require generic functions as arguments to work, and so are left as they are

use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use async_graphql::{ObjectType, Schema, SubscriptionType};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use crate::auth::auth_state::AuthState;

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
    S: SubscriptionType + 'static,
{
    // Get the GraphQL request so we can add data to it
    let mut query = req.into_inner();
    // Get the authorisation data from the request extensions if it exists (it would be set by the middleware)
    let extensions = http_req.extensions();
    let auth_data = extensions.get::<AuthState>();

    // Clone the internal AuthState so we can place the variable into the context (lifetimes...)
    let auth_data_for_ctx = auth_data.cloned();
    // Add that to the GraphQL request data so we can access it in the resolvers
    query = query.data(auth_data_for_ctx);
    schema.execute(query).await.into()
}

pub async fn graphql_ws<Q, M, S>(
    schema: web::Data<Schema<Q, M, S>>,
    http_req: HttpRequest,
    payload: web::Payload,
) -> ActixResult<HttpResponse>
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    WSSubscription::start(Schema::clone(&schema), &http_req, payload)
}
