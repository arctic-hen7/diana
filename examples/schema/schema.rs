// This file just defines the common schema and options that everything else will use
// All examples import it dirtily by using the `include!` macro, which you should never use unless you have a very good reason to!

use diana::{
    Options, OptionsBuilder, AuthCheckBlockState,
    GQLObject, GQLSubscription,
    GQLSimpleObject,
    errors::GQLResult,
    Stream, stream,
    graphql_utils::get_stream_for_channel_from_ctx,
    Publisher
};
use std::env;
use serde::{Serialize, Deserialize};

#[derive(GQLSimpleObject, Serialize, Deserialize)]
pub struct User {
    username: String
}

#[derive(Default, Clone)]
pub struct Query {}
#[GQLObject]
impl Query {
    async fn api_version(&self) -> &str {
        "0.1.0"
    }
}
#[derive(Default, Clone)]
pub struct Mutation {}
#[GQLObject]
impl Mutation {
    // This only exists to illustrate sending data to the subscriptions server
    async fn update_blah(
        &self,
        raw_ctx: &async_graphql::Context<'_>,
    ) -> GQLResult<bool> {
        // Imagine we've acquired this from the user's input
        let user = User {
            username: "This is a username".to_string()
        };
        // Stringify and publish the data to the subscriptions server
        let publisher = raw_ctx.data::<Publisher>()?;
        let user_json = serde_json::to_string(&user)?;
        publisher.publish("new_blah", user_json).await?;
        Ok(true)
    }
}
#[derive(Default, Clone)]
pub struct Subscription;
#[GQLSubscription]
impl Subscription {
    async fn new_blahs(
        &self,
        raw_ctx: &async_graphql::Context<'_>,
    ) -> impl Stream<Item = GQLResult<User>> {
        // Get a direct stream from the context on a certain channel
        let stream_result = get_stream_for_channel_from_ctx("new_blah", raw_ctx);

        // We can manipulate the stream using the stream macro from async-stream
        stream! {
            let stream = stream_result?;
            for await message in stream {
                // Serialise the data as a user
                let new_user: User = serde_json::from_str(&message).map_err(|_err| "couldn't serialize given data correctly".to_string())?;
                yield Ok(new_user);
            }
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pub pool: String // This might be an actual database pool
}

pub fn get_opts() -> Options<Context, Query, Mutation, Subscription> {
    // Load the environment variable file if we're not in Netlify
    // If we are, the variables should be directly available
    if env::var("NETLIFY").is_err() {
        dotenv::from_filename("examples/.env").expect("Failed to load environment variables!");
    }

    OptionsBuilder::new()
        .ctx(Context {
            pool: "connection".to_string(),
        })
        .subscriptions_server_hostname("http://localhost")
        .subscriptions_server_port("9002")
        .subscriptions_server_endpoint("/graphql")
        .jwt_to_connect_to_subscriptions_server(
            &env::var("SUBSCRIPTIONS_SERVER_PUBLISH_JWT").unwrap(),
        )
        .auth_block_state(AuthCheckBlockState::AllowAll)
        .jwt_secret(&env::var("JWT_SECRET").unwrap())
        .schema(Query {}, Mutation {}, Subscription {})
        // Endpoints are set up as `/graphql` and `/graphiql` automatically
        .finish()
        .expect("Failed to build options!")
}
