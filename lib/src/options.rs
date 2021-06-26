// Contains the logic to actually create the GraphQL server that the user will use
// This file does not include any logic for the subscriptions server

use async_graphql::{ObjectType, SubscriptionType};
use std::any::Any;

pub use crate::auth::middleware::AuthCheckBlockState; // Users should be able to easily access this
use crate::errors::*;
pub use crate::graphql::{SubscriptionsServerInformation, UserSchema};

// Options to create a GraphQL server
#[derive(Clone)]
pub struct Options<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    pub ctx: C, // The user's custom context, often a database connection pool
    pub subscriptions_server_data: Option<SubscriptionsServerInformation>, // Some systems may not actually use a subscriptions server
    pub schema: UserSchema<Q, M, S>, // The schema the user defines
    pub jwt_secret: String, // This allows checking client tokens (should be different to the subscription server's secret)
    pub authentication_block_state: AuthCheckBlockState, // The level of blocking that will be used for the endpoint
    pub playground_endpoint: Option<String>, // The endpoint for GraphiQL (if None, no playground will be created)
    pub graphql_endpoint: String,            // The endpoint for GraphQL itself
}

// A builder for the options to be provided to the GraphQL server creator
#[derive(Clone)]
pub struct OptionsBuilder<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    ctx: Option<C>,
    use_subscriptions_server: bool,
    subscriptions_server_hostname: Option<String>, // The real property actually does take an Option<String> for this one
    subscriptions_server_port: Option<String>, // The real property actually does take an Option<String> for this one
    subscriptions_server_endpoint: Option<String>, // The real property actually does take an Option<String> for this one
    subscriptions_server_jwt_to_connect: Option<String>, // The real property actually does take an Option<String> for this one
    schema: Option<UserSchema<Q, M, S>>,
    jwt_secret: Option<String>,
    authentication_block_state: Option<AuthCheckBlockState>,
    playground_endpoint: Option<String>, // The real property actually does take an Option<String> for this one
    graphql_endpoint: Option<String>,
}
impl<C, Q, M, S> Default for OptionsBuilder<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    fn default() -> Self {
        // By default, if we're in development we'll have a playground, and not in production
        let playground_endpoint = match cfg!(debug_assertions) {
            true => Some("/graphiql".to_string()),
            false => None,
        };

        Self {
            ctx: None,
            use_subscriptions_server: false, // Most systems won't actually use subscriptions
            subscriptions_server_hostname: None,
            subscriptions_server_port: None,
            subscriptions_server_endpoint: None,
            subscriptions_server_jwt_to_connect: None,
            schema: None,
            jwt_secret: None,
            authentication_block_state: None,
            playground_endpoint,
            graphql_endpoint: Some("/graphql".to_string()),
        }
    }
}
impl<C, Q, M, S> OptionsBuilder<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    // Here begin the functions to build the options
    pub fn ctx(mut self, ctx: C) -> Self {
        self.ctx = Some(ctx);
        self
    }
    pub fn jwt_secret(mut self, jwt_secret: &str) -> Self {
        self.jwt_secret = Some(jwt_secret.to_string());
        self
    }
    pub fn auth_block_state(mut self, authentication_block_state: AuthCheckBlockState) -> Self {
        self.authentication_block_state = Some(authentication_block_state);
        self
    }
    pub fn schema(mut self, query_root: Q, mutation_root: M, subscription_root: S) -> Self {
        self.schema = Some(UserSchema {
            query_root,
            mutation_root,
            subscription_root,
        });
        self
    }
    // This shouldn't need to be called explicitly
    // Whenever any of the functions that specify subscriptions server options are called, said server is automatically enabled
    pub fn use_subscriptions_server(mut self) -> Self {
        self.use_subscriptions_server = true;
        self
    }
    pub fn subscriptions_server_hostname(mut self, subscriptions_server_hostname: &str) -> Self {
        self.subscriptions_server_hostname = Some(subscriptions_server_hostname.to_string());
        self.use_subscriptions_server = true;
        self
    }
    pub fn subscriptions_server_port(mut self, subscriptions_server_port: &str) -> Self {
        self.subscriptions_server_port = Some(subscriptions_server_port.to_string());
        self.use_subscriptions_server = true;
        self
    }
    pub fn subscriptions_server_endpoint(mut self, subscriptions_server_endpoint: &str) -> Self {
        self.subscriptions_server_endpoint = Some(subscriptions_server_endpoint.to_string());
        self.use_subscriptions_server = true;
        self
    }
    pub fn jwt_to_connect_to_subscriptions_server(
        mut self,
        subscriptions_server_jwt_to_connect: &str,
    ) -> Self {
        self.subscriptions_server_jwt_to_connect =
            Some(subscriptions_server_jwt_to_connect.to_string());
        self.use_subscriptions_server = true;
        self
    }
    pub fn playground_endpoint(mut self, playground_endpoint: &str) -> Self {
        self.playground_endpoint = Some(playground_endpoint.to_string());
        self
    }
    pub fn graphql_endpoint(mut self, graphql_endpoint: &str) -> Self {
        self.graphql_endpoint = Some(graphql_endpoint.to_string());
        self
    }
    // Here end the functions to build the options

    // Builds the final options, consuming self
    pub fn finish(self) -> Result<Options<C, Q, M, S>> {
        let opts = Options {
            ctx: self.ctx.ok_or(ErrorKind::IncompleteBuilderFields)?,
            subscriptions_server_data: match self.use_subscriptions_server {
                true => Some(SubscriptionsServerInformation {
                    hostname: self
                        .subscriptions_server_hostname
                        .ok_or(ErrorKind::IncompleteBuilderFields)?,
                    port: self
                        .subscriptions_server_port
                        .ok_or(ErrorKind::IncompleteBuilderFields)?,
                    endpoint: self
                        .subscriptions_server_endpoint
                        .ok_or(ErrorKind::IncompleteBuilderFields)?,
                    jwt_to_connect: self
                        .subscriptions_server_jwt_to_connect
                        .ok_or(ErrorKind::IncompleteBuilderFields)?,
                }),
                false => None,
            },
            schema: self.schema.ok_or(ErrorKind::IncompleteBuilderFields)?,
            jwt_secret: self.jwt_secret.ok_or(ErrorKind::IncompleteBuilderFields)?,
            authentication_block_state: self
                .authentication_block_state
                .ok_or(ErrorKind::IncompleteBuilderFields)?,
            playground_endpoint: self.playground_endpoint, // This can be an option (we may not have a playground at all)
            graphql_endpoint: self
                .graphql_endpoint
                .ok_or(ErrorKind::IncompleteBuilderFields)?,
        };

        Ok(opts)
    }
}
