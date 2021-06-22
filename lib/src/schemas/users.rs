use tokio::stream::{StreamExt, Stream};
use serde::{Serialize, Deserialize};
use async_graphql::{
    SimpleObject as GQLSimpleObject,
    Object as GQLObject,
    InputObject as GQLInputObject,
    Subscription as GQLSubscription
};
use mongodb::{
    bson::doc,
    Client as MongoClient,
    Collection
};
use async_stream::stream;

// Note that we don't use `error_chain` here, just the GraphQL errors system
use crate::errors::{GQLResult, GQLError};
use crate::graphql_utils::{get_client_from_ctx, get_stream_for_channel_from_ctx};
use crate::oid::ObjectId;
use crate::pubsub::Publisher;

#[derive(Serialize, Deserialize, Debug, GQLSimpleObject)]
pub struct User {
    // We need to use `id` because otherwise we can't access the field properly with Rust
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub full_name: Option<String>,
    pub password: String
}
#[derive(Serialize, Deserialize, Debug, GQLInputObject)]
pub struct UserInput {
    pub username: String,
    pub full_name: Option<String>,
    pub password: String
}

fn get_users(client: MongoClient) -> Collection<User> {
    client.database("test").collection_with_type::<User>("users")
}

// Register Query methods
#[derive(Default, Clone)]
pub struct Query {}
#[GQLObject]
impl Query {
    async fn users(&self, ctx: &async_graphql::Context<'_>, username: String) -> GQLResult<Vec<User>> {
        let users = get_users(get_client_from_ctx(ctx)?);

        let mut cursor = users.find(doc! {
            "username": username
        }, None).await?;
        let mut res: Vec<User> = Vec::new();
        while let Some(user) = cursor.next().await {
            res.push(user?);
        }
        Ok(res)
    }
}

// Register Mutation methods
#[derive(Default, Clone)]
pub struct Mutation {}
#[GQLObject]
impl Mutation {
    async fn add_user(&self, ctx: &async_graphql::Context<'_>, new_user: UserInput) -> GQLResult<User> {
        let users = get_users(get_client_from_ctx(ctx)?);
        let users_input: Collection<UserInput> = users.clone_with_type();

        let insertion_res = users_input.insert_one(new_user, None).await?;
        let inserted = users.find_one(ObjectId::find_clause_from_insertion_res(insertion_res)?, None).await?;

        let insert_find_err = GQLError::new("Couldn't find inserted field");

        if inserted.is_some() {
            // Notify the subscriptions server that a new user has been added
            let publisher = ctx.data::<Publisher>()?;
            let user_json = serde_json::to_string(&inserted).unwrap(); // We just created it, it should certainly serialise
            publisher.publish("new_user", user_json.to_string()).await?;
        }

        inserted.ok_or(
            insert_find_err
        )
    }
}

// Register Subscription methods
#[derive(Default, Clone)]
pub struct Subscription;
#[GQLSubscription]
impl Subscription {
    async fn numbers(&self) -> impl Stream<Item = i32> {
        futures::stream::iter(0..10)
    }
    // Returns each new user that is added
    async fn new_users(&self, raw_ctx: &async_graphql::Context<'_>) -> impl Stream<Item = Result<User, String>> {
        // Get a direct stream from the context on a certain channel
        let stream = get_stream_for_channel_from_ctx("new_user", raw_ctx);

        // We can manipulate the stream using the stream macro from async-stream
        stream! {
            for await message in stream {
                // Serialise the data as a user
                let new_user: User = serde_json::from_str(&message).map_err(|_err| "couldn't serialize given data correctly".to_string())?;
                yield Ok(new_user);
            }
        }
    }
}
