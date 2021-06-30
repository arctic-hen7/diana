use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use async_graphql::{ObjectType, SubscriptionType};
use async_graphql_actix_web::WSSubscription; // Pre-built WebSocket logic
use std::any::Any;

use diana::{DianaHandler, DianaResponse, AuthVerdict};

// TODO reduce code duplication here

// The main GraphQL endpoint for queries and mutations with authentication support
// This handler does not support subscriptions
pub async fn graphql_without_subscriptions<C, Q, M, S>(
    diana_handler: web::Data<DianaHandler<C, Q, M, S>>,
    http_req: HttpRequest,
    body: String
) -> HttpResponse
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    // Get the authorisation verdict from the request extensions if it exists (it would be set by the middleware)
    let extensions = http_req.extensions();
    let auth_verdict = extensions.get::<AuthVerdict>().cloned();

    // Run the query, stating that authentication checks don't need to be performed again
    let res = diana_handler.run_stateless_without_subscriptions(body, None, auth_verdict).await;

    // Transform the DianaResponse into an HttpResponse
    match res {
        DianaResponse::Success(res) => res.into(),
        DianaResponse::Blocked => HttpResponse::Forbidden().finish(),
        DianaResponse::Error(_) => HttpResponse::InternalServerError().finish()
    }
}
// The main GraphQL endpoint for queries and mutations with authentication support
// This handler does not support subscriptions, but is for use in the subscriptions system (which also needs query/mutation support)
pub async fn graphql_for_subscriptions<C, Q, M, S>(
    diana_handler: web::Data<DianaHandler<C, Q, M, S>>,
    http_req: HttpRequest,
    body: String
) -> HttpResponse
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    // Get the authorisation verdict from the request extensions if it exists (it would be set by the middleware)
    let extensions = http_req.extensions();
    let auth_verdict = extensions.get::<AuthVerdict>().cloned();

    // Run the query, stating that authentication checks don't need to be performed again
    let res = diana_handler.run_stateless_for_subscriptions(body, None, auth_verdict).await;

    // Transform the DianaResponse into an HttpResponse
    match res {
        DianaResponse::Success(res) => res.into(),
        DianaResponse::Blocked => HttpResponse::Forbidden().finish(),
        DianaResponse::Error(_) => HttpResponse::InternalServerError().finish()
    }
}

// The endpoint for GraphQL subscriptions
// This doesn't use DianaHandler at all (except to extract the needed schema) because `async_graphql` provides practically pre-built integration for this
pub async fn graphql_ws<C, Q, M, S>(
    diana_handler: web::Data<DianaHandler<C, Q, M, S>>,
    http_req: HttpRequest,
    payload: web::Payload,
) -> ActixResult<HttpResponse>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    let schema = &diana_handler.schema_for_subscriptions;
    WSSubscription::start(schema.clone(), &http_req, payload)
}
