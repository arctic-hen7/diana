// This file contains the core logic primitives that actually run a given request
// This is depended on by serverful and serverless systems

use async_graphql::{EmptySubscription, ObjectType, Request, Schema, SubscriptionType};
use std::any::Any;

use crate::auth::core::{get_auth_verdict, get_token_state_from_header, AuthVerdict};
use crate::errors::*;
use crate::graphql::{
    get_schema_for_subscriptions, get_schema_without_subscriptions, PublishMutation,
    SubscriptionQuery,
};
use crate::options::Options;

/// The basic response from a given request.
#[derive(Clone, Debug)]
pub enum DianaResponse {
    /// The request was successful and the response is attached.
    /// Return a 200.
    Success(String),
    /// The request was blocked (unauthorized).
    /// Return a 403.
    Blocked,
    /// An error occurred on the server side and its body is encapsulated. Any GraphQL errors will be encapsulated in the `Success` variant's
    /// payload.
    /// Return a 500.
    Error(String),
}

// Represents the chice of the schema for/without subscriptions
#[doc(hidden)]
pub enum SysSchema {
    WithoutSubscriptions,
    ForSubscriptions,
}

/// The core logic primitive that underlies Diana's systems. You should only use this if you need to support a platform other than the ones
/// Diana has pre-built systems for (see the book).
/// This is a struct so as to allow the caching of data that stay the same across requests, like the parsed and built schemas.
#[derive(Clone)]
pub struct DianaHandler<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    /// The options parsed in to the handler in `::new()`.You should only need to touch this if you're building a custom integration.
    pub opts: Options<C, Q, M, S>,
    /// The schema created for the queries/mutations system. This has the user's given query and mutation roots and no subscriptions at all.
    /// You should only need to touch this if you're building a custom integration.
    pub schema_without_subscriptions: Schema<Q, M, EmptySubscription>,
    /// The schema created for the subscriptions server. This has the user's given subscription root and internally used query/mutation roots
    /// for communication with the query/mutation system. You should only need to touch this if you're building a custom integration.
    pub schema_for_subscriptions: Schema<SubscriptionQuery, PublishMutation, S>,
}
impl<C, Q, M, S> DianaHandler<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    /// Creates a new instance of the handler with the given options.
    pub fn new(opts: Options<C, Q, M, S>) -> Result<Self> {
        // TODO only create a schema for subscriptions if they're actually being used (will require broader logic changes)
        // Get the schema (this also creates a publisher to the subscriptions server and inserts context)
        // We deal with any errors directly with the serverless response enum
        let schema_without_subscriptions = get_schema_without_subscriptions(
            opts.schema.clone(),
            opts.subscriptions_server_data.clone(),
            opts.ctx.clone(),
        )?;
        let schema_for_subscriptions =
            get_schema_for_subscriptions(opts.schema.clone(), opts.ctx.clone());

        Ok(DianaHandler {
            opts,
            schema_without_subscriptions,
            schema_for_subscriptions,
        })
    }
    /// Determines ahead of time whether or not a request is authenticated. This should be used in middleware if possible so we can avoid
    /// sending full payloads if the auth token isn't even valid.
    /// This just takes the HTTP `Authorization` header and returns an [`AuthVerdict`].
    pub fn is_authed<A: Into<String> + std::fmt::Display>(&self, raw_auth_header: Option<A>) -> AuthVerdict {
        // This function accepts anything that can be turned into a string for convenience
        // Then we convert it into a definite Option<String>
        let auth_header = raw_auth_header.map(|x| x.to_string());
        // And then we get it as an Option<&str> (whic is what we need for slicing)
        let auth_header_str = auth_header.as_deref();
        // Get a verdict on whether or not the user should be allowed through
        let token_state =
            get_token_state_from_header(auth_header_str, self.opts.jwt_secret.clone());
        get_auth_verdict(token_state, self.opts.authentication_block_state)
    }
    /// Runs a query or mutation (stateless) given the request body and the value of the HTTP `Authorization` header.
    /// This performs authorisation checks and runs the actual request. If you've already used `.is_authed()` to obtain an [`AuthVerdict`],
    /// this can be provided as the third argument to avoid running auth checks twice.
    /// This will return a [`DianaResponse`] no matter what, which simplifies error handling significantly.
    /// This function is for the subscriptions system only.
    pub async fn run_stateless_for_subscriptions<A: Into<String> + std::fmt::Display>(
        &self,
        body: String,
        raw_auth_header: Option<A>,
        given_auth_verdict: Option<AuthVerdict>,
    ) -> DianaResponse {
        self.run_stateless_req(
            SysSchema::ForSubscriptions,
            body,
            raw_auth_header,
            given_auth_verdict,
        )
        .await
    }
    /// Runs a query or mutation (stateless) given the request body and the value of the HTTP `Authorization` header.
    /// This performs authorisation checks and runs the actual request. If you've already used `.is_authed()` to obtain an [`AuthVerdict`],
    /// this can be provided as the third argument to avoid running auth checks twice.
    /// This will return a [`DianaResponse`] no matter what, which simplifies error handling significantly.
    /// This function is for the queries/mutations system only.
    pub async fn run_stateless_without_subscriptions<A: Into<String> + std::fmt::Display>(
        &self,
        body: String,
        raw_auth_header: Option<A>,
        given_auth_verdict: Option<AuthVerdict>,
    ) -> DianaResponse {
        self.run_stateless_req(
            SysSchema::WithoutSubscriptions,
            body,
            raw_auth_header,
            given_auth_verdict,
        )
        .await
    }
    // This is used internally to provide query/mutation running functionality to the systems for/without subscriptions
    // It is exposed to make testing easier, though users should not use it!
    #[doc(hidden)]
    pub async fn run_stateless_req<A: Into<String> + std::fmt::Display>(
        &self,
        which_schema: SysSchema,
        body: String,
        raw_auth_header: Option<A>,
        given_auth_verdict: Option<AuthVerdict>,
    ) -> DianaResponse {
        // Run authentication checks if we need to (they may have already been run in middleware)
        let verdict = match given_auth_verdict {
            Some(verdict) => verdict,
            None => self.is_authed(raw_auth_header),
        };

        // Based on that verdict, maybe run the request
        match verdict {
            AuthVerdict::Allow(auth_data) => {
                // Deserialise that raw JSON request into an actual request with variables etc.
                let gql_req = serde_json::from_str::<Request>(&body);
                let mut gql_req = match gql_req {
                    Ok(gql_req) => gql_req,
                    Err(err) => return DianaResponse::Error(err.to_string()),
                };
                // Insert the authentication data directly into that
                gql_req = gql_req.data(auth_data);
                // Run the request with the correct schema
                let res = match which_schema {
                    SysSchema::WithoutSubscriptions => {
                        self.schema_without_subscriptions.execute(gql_req).await
                    }
                    SysSchema::ForSubscriptions => {
                        self.schema_for_subscriptions.execute(gql_req).await
                    }
                };
                // Serialise that response into a string (the response bodies all have to be of the same type)
                let res_str = serde_json::to_string(&res);
                let res_str = match res_str {
                    Ok(res_str) => res_str,
                    Err(err) => return DianaResponse::Error(err.to_string()),
                };

                DianaResponse::Success(res_str)
            }
            AuthVerdict::Block => DianaResponse::Blocked,
            AuthVerdict::Error(err) => DianaResponse::Error(err),
        }
    }
}
