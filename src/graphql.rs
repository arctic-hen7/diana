use async_graphql::{EmptySubscription, Object as GQLObject, ObjectType, Schema, SubscriptionType};
use std::any::Any;
use std::sync::Mutex;

use crate::errors::*;
use crate::graphql_utils::{get_auth_data_from_ctx, get_pubsub_from_ctx};
use crate::is_authed;
use crate::pubsub::{PubSub, Publisher};

// The base query type simply allows us to set up the subscriptions schema (has to have at least one query)
#[derive(Default, Clone)]
pub struct SubscriptionQuery;
#[GQLObject]
impl SubscriptionQuery {
    // TODO disable introspection on this endpoint
    async fn _query(&self) -> String {
        "This is a meaningless endpoint needed only for initialisation.".to_string()
    }
}

// This mutation type is utilised by the subscriptions server to allow the publishing of data
// We pass around the PubSub state internally to that GraphQL system (see get_schema_for_subscriptions)
#[derive(Default, Clone)]
pub struct PublishMutation;
#[GQLObject]
impl PublishMutation {
    // We accept string data because this is a highly generic type that serialises in the subscriptions handler
    // That may seem to subvert some of the purpose of GraphQL, but this resolver is to be INTERNALLY ONLY!
    // That provides a system-level data integrity guarantee, as only full mutations will call this, and through a PubSub abstraction
    // There should be very little reason for users to implement it themselves, but this type could easily be extended with custom logic
    async fn publish(
        &self,
        raw_ctx: &async_graphql::Context<'_>,
        channel: String,
        data: String,
    ) -> Result<bool> {
        if is_authed!(
            get_auth_data_from_ctx(raw_ctx)?,
            {
                "role" => "graphql_server"
            }
        ) {
            let mut pubsub = get_pubsub_from_ctx(raw_ctx)?;
            pubsub.publish(&channel, data);
            Ok(true)
        } else {
            bail!(ErrorKind::Unauthorised)
        }
    }
}

// Information about the subscriptions server for the rest of the system
#[derive(Clone)]
pub struct SubscriptionsServerInformation {
    pub hostname: String,
    pub port: String, // It'll be mixed in to create a URL, may as well start as a string
    pub endpoint: String,
    pub jwt_to_connect: String, // This should be signed with the secret the subscriptions server knows
}

// A type for the schema that the user will submit
#[derive(Clone)]
pub struct UserSchema<Q, M, S>
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    pub query_root: Q,
    pub mutation_root: M,
    pub subscription_root: S,
}

pub fn get_schema_without_subscriptions<C, Q, M, S>(
    user_schema: UserSchema<Q, M, S>,
    subscription_server_info: Option<SubscriptionsServerInformation>,
    user_ctx: C,
) -> Result<Schema<Q, M, EmptySubscription>>
where
    C: Any + Send + Sync,
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    // We just use an empty subscription root here because subscriptions are handled by the dedicated subscriptions server
    let schema = Schema::build(
        user_schema.query_root,
        user_schema.mutation_root,
        EmptySubscription,
    )
    // We add some custom user-defined context (e.g. a database connection pool)
    .data(user_ctx);

    // Conditionally extend that schema with a publisher if we're using a subscriptions server
    let schema = match subscription_server_info {
        Some(subscription_server_info) => schema
            .data(Publisher::new(
                subscription_server_info.hostname,
                subscription_server_info.port,
                subscription_server_info.endpoint,
                subscription_server_info.jwt_to_connect,
            )?)
            .finish(),
        None => schema.finish(),
    };

    Ok(schema)
}
pub fn get_schema_for_subscriptions<C, Q, M, S>(
    user_schema: UserSchema<Q, M, S>,
    user_ctx: C,
) -> Schema<SubscriptionQuery, PublishMutation, S>
where
    C: Any + Send + Sync,
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    // The schema for the subscriptions server should only have subscriptions, and a mutation to allow publishing
    // Unfortunately, we have to have at least one query, so we implement a meaningless one that isn't introspected
    Schema::build(
        SubscriptionQuery,
        PublishMutation,
        user_schema.subscription_root,
    )
    // We add some custom user-defined context (e.g. a database connection pool)
    .data(user_ctx)
    // We add a mutable PubSub instance for managing subscriptions internally
    .data(Mutex::new(PubSub::default())) // We add a PubSub instance to internally manage state in the serverful subscriptions system
    .finish()
}
