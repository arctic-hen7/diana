// Contains the logic to actually create the GraphQL server that the user will use
// This file does not include any logic for the subscriptions server

use async_graphql::{ObjectType, SubscriptionType};
use std::any::Any;

use crate::auth::core::AuthCheckBlockState;
use crate::errors::*;
pub use crate::graphql::{SubscriptionsServerInformation, UserSchema};

/// The options for creating the normal server, subscriptions server, and serverless function.
/// You should define your options in one file and then import them everywhere you need them.
/// You should use [OptionsBuilder] to construct this.
#[derive(Clone)]
pub struct Options<C, Q, M, S>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    /// Your custom context, often a database connection pool
    pub ctx: C,
    /// Data about the subscriptions server
    /// If you're not using subscriptions at all, the mechanics to connect to such a server will be disabled automatically.
    pub subscriptions_server_data: Option<SubscriptionsServerInformation>,
    /// Your `async_graphql` schema. See the book for details on how to create a schema.
    pub schema: UserSchema<Q, M, S>,
    /// The JWT secret for authenticating and generating client tokens and communications with the subscriptions server.
    /// This should be stored in an environment variable and randomly generated (see the book).
    pub jwt_secret: String,
    /// The blocking level that will be used for the GraphQL endpoint.
    /// See [AuthCheckBlockState] for available blocklevels and their meanings.
    /// The default here is to block anything that is not authenticated.
    pub authentication_block_state: AuthCheckBlockState,
    /// The endpoint for the GraphiQL playground.
    /// If nothing is provided here, the playground will be disabled.
    /// Not supported in production
    pub playground_endpoint: Option<String>,
    /// The GraphQL endpoint location. By default `/graphql`.
    pub graphql_endpoint: String,
}

/// A builder-style struct to create an instance of [Options] idiomatically.
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
    /// Creates a new builder. You'll need to then call the other methods to set everything up.
    pub fn new() -> Self {
        Self::default()
    }

    // Here begin the functions to build the options
    /// Defines the context to pass to all GraphQL resolvers. This is typically a database pool, but it can really be anything that can
    /// be safely sent between threads.
    /// If you don't want to use context, you'll need to set something here anyway (for now).
    pub fn ctx(mut self, ctx: C) -> Self {
        self.ctx = Some(ctx);
        self
    }
    /// Defines the JWt secret that will be used to authenticate client tokens and communication with the subscriptions server.
    /// This should be randomly generated (see the book).
    pub fn jwt_secret(mut self, jwt_secret: &str) -> Self {
        self.jwt_secret = Some(jwt_secret.to_string());
        self
    }
    /// Defines the blocklevel for the GraphQL endpoint. See [AuthCheckBlockState] for more details.
    pub fn auth_block_state(mut self, authentication_block_state: AuthCheckBlockState) -> Self {
        self.authentication_block_state = Some(authentication_block_state);
        self
    }
    /// Defines your custom schema.
    /// The query/mutation roots will be inserted into the queries/mutations server/function and the subscription root will be inserted
    /// into the subscriptions server. These should be specified using `async_graphql` as per the book.
    pub fn schema(mut self, query_root: Q, mutation_root: M, subscription_root: S) -> Self {
        self.schema = Some(UserSchema {
            query_root,
            mutation_root,
            subscription_root,
        });
        self
    }
    /// Explicitly enables the subscriptions server. You shouldn't every need to call this, as calling any methods that define settings
    /// for the subscriptions server will automatically enable it.
    pub fn use_subscriptions_server(mut self) -> Self {
        self.use_subscriptions_server = true;
        self
    }
    /// Defines the hsotname on which the subscriptions server will be contacted.
    pub fn subscriptions_server_hostname(mut self, subscriptions_server_hostname: &str) -> Self {
        self.subscriptions_server_hostname = Some(subscriptions_server_hostname.to_string());
        self.use_subscriptions_server = true;
        self
    }
    /// Defines the port on which the subscriptions server will be contacted.
    pub fn subscriptions_server_port(mut self, subscriptions_server_port: &str) -> Self {
        self.subscriptions_server_port = Some(subscriptions_server_port.to_string());
        self.use_subscriptions_server = true;
        self
    }
    /// Defines the GraphQL endpoint for the subscriptions server.
    /// The GraphiQL playground endpoint inherits from the `.playground_endpoint()` setting.
    pub fn subscriptions_server_endpoint(mut self, subscriptions_server_endpoint: &str) -> Self {
        self.subscriptions_server_endpoint = Some(subscriptions_server_endpoint.to_string());
        self.use_subscriptions_server = true;
        self
    }
    /// Specifies the JWT which will be used by the queries/mutations system to connect to the subscriptions server.
    /// This should be generated based off the same secret as you specify for the queries/mutations system (TODO security review of that architecture).
    pub fn jwt_to_connect_to_subscriptions_server(
        mut self,
        subscriptions_server_jwt_to_connect: &str,
    ) -> Self {
        self.subscriptions_server_jwt_to_connect =
            Some(subscriptions_server_jwt_to_connect.to_string());
        self.use_subscriptions_server = true;
        self
    }
    /// Defines the GraphiQL playground endpoint.
    /// In development, this is not required and will default to `/graphiql`.
    /// In production, if this has been set we'll throw an error at `.finish()`.
    pub fn playground_endpoint(mut self, playground_endpoint: &str) -> Self {
        self.playground_endpoint = Some(playground_endpoint.to_string());
        self
    }
    /// Defines the GraphQL endpoint. This is not required, and defaults to `/graphql`.
    pub fn graphql_endpoint(mut self, graphql_endpoint: &str) -> Self {
        self.graphql_endpoint = Some(graphql_endpoint.to_string());
        self
    }
    // Here end the functions to build the options

    /// Builds the final options, consuming `self`.
    /// This will return an error if you haven't set something required up.
    pub fn finish(self) -> Result<Options<C, Q, M, S>> {
        // If the playground has been enabled in production, throw
        if !cfg!(debug_assertions) && self.playground_endpoint.is_some() {
            bail!(ErrorKind::AttemptedPlaygroundInProduction);
        }

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
