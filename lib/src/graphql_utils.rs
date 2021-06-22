// Utility functions for GraphQL resolvers
use std::sync::{Mutex, MutexGuard};
use mongodb::Client as MongoClient;
use tokio::stream::Stream;

use crate::pubsub::PubSub;
use crate::auth::auth_state::AuthState;
use crate::db::DbPool;
use crate::errors::*;

// Checks to see if the given authentication state matches the series of given claims
// Must be provided the authentication state to operate on, a series of claims, and code to execute if authenticated
// This will bail with an internal Unauthorised error if the claims aren't met
#[macro_export]
macro_rules! if_authed(
    ($auth_state:expr, { $($key:expr => $value:expr),+ }, $code:block) => {
        {
            // Create a HashMap out of the given test claims
            let mut test_claims: ::std::collections::HashMap<&str, &str> = ::std::collections::HashMap::new();
            $(
                test_claims.insert($key, $value);
            )+
            // Match the authentication state with those claims now
            match $auth_state {
                Some(auth_state) if auth_state.has_claims(test_claims) => {
                    $code
                },
                _ => crate::errors::bail!(crate::errors::ErrorKind::Unauthorised)
            }
        }
     };
);

// We make an instance of the database client accessible to all GraphQL resolvers through context
#[derive(Clone)]
pub struct Context {
    pub pool: DbPool, // This needs to be public so that schema files can use it
}

// A utility function to get a client from the given context object
pub fn get_client_from_ctx(raw_ctx: &async_graphql::Context<'_>) -> Result<MongoClient> {
    // Extract our context from the broader `async_graphql` context
    let ctx = raw_ctx.data::<Context>()
        .map_err(|_err| ErrorKind::GraphQLContextNotFound("main context".to_string()))?;
    let client = ctx.pool.get_client()?;

    Ok(client)
}

// A helper function to subscribe to events sent to the subscriptions server on a particular channel
// This returns a pre-created stream which you should manipulate if necessary (e.g. to serialise data)
// ONLY USE THIS IN SUBSCRIPTIONS! It will only run on the serverful system (stateful)
// TODO handle errors in this function
pub fn get_stream_for_channel_from_ctx(channel: &str, raw_ctx: &async_graphql::Context<'_>) -> impl Stream<Item = String> {
    // Get the PubSub mutably
    let mut pubsub = get_pubsub_from_ctx(raw_ctx).unwrap(); // FIXME
    // Return a stream on the given channel
    pubsub.subscribe(channel)
}

// A utility function to get authentication data from the context of a GraphQL resolver
pub fn get_auth_data_from_ctx<'a>(raw_ctx: &'a async_graphql::Context<'_>) -> Result<&'a Option<AuthState>> {
    let auth_state = raw_ctx.data::<Option<AuthState>>()
            .map_err(|_err| ErrorKind::GraphQLContextNotFound("auth_state".to_string()))?;

    Ok(auth_state)
}
// A utility function to get a mutable verion of the PubSub from the context of a GraphQL resolver
pub fn get_pubsub_from_ctx<'a>(raw_ctx: &'a async_graphql::Context<'_>) -> Result<MutexGuard<'a, PubSub>> {
    // We store the PubSub instance as a Mutex because we need it sent/synced between threads as a mutable
    let pubsub_mutex = raw_ctx.data::<Mutex<PubSub>>()
            .map_err(|_err| ErrorKind::GraphQLContextNotFound("pubsub".to_string()))?;
    let pubsub = pubsub_mutex.lock()
        .map_err(|_err| ErrorKind::MutexPoisoned("pubsub".to_string()))?;

    Ok(pubsub)
}
