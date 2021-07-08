# Writing Schemas

The most critical part of GraphQL is schemas, and they're easily the most complex part of Diana. This page will explain how to write schemas that work with Diana, and it will explain how to get subscriptions to work properly, however it will not explain how to write basic schemas from scratch. For that, please refer to [async_graphql's documentation](https://async-graphql.github.io).

## Subscriptions

A basic Diana subscription would look something like this:

```rust
use diana::async_graphql::{Subscription as GQLSubscription, Context as GQLCtx};
use diana::errors::GQLResult;
use diana::stream;

#[derive(Default, Clone)]
pub struct Subscription;
#[GQLSubscription]
impl Subscription {
    async fn new_blahs(
        &self,
        raw_ctx: &GQLCtx<'_>,
    ) -> impl Stream<Item = GQLResult<String>> {
        let stream_result = get_stream_for_channel_from_ctx("channel_name", raw_ctx);

        stream! {
            let stream = stream_result?;
            for await message in stream {
                yield Ok(message);
            }
        }
    }
}
```

All this does is sets up a subscription that will return the strings on a particular channel. And this shows perfectly how subscriptions in Diana work -- channels. You publish something on a channel from the queries/mutations system and then receive it as above. You can then use the re-exported `stream!` macro to return a stream for it.

Note that if you're trying to send a struct across channels you'll need to serialize/deserialize it into/out of a string for transport. However, as subscriptions can return errors in their streams, this shouldn't be a problem!

## Mutations that link with subscriptions

The most common thing to trigger a subscription is some kind of mutation on the queries/mutations system, and so Diana provides a simple programmatic way of publishing data on a particular channel:

```rust
use diana::async_graphql::{Subscription as GQLSubscription, Context as GQLCtx};
use diana::errors::GQLResult;
use diana::stream;
use diana::Publisher;

#[derive(Default, Clone)]
pub struct Mutation {}
#[GQLObject]
impl Mutation {
    async fn update_blah(
        &self,
        raw_ctx: &async_graphql::Context<'_>,
    ) -> GQLResult<bool> {
        let publisher = raw_ctx.data::<Publisher>()?;
        publisher.publish("channel_name", "important message").await?;
        Ok(true)
    }
}
```

In the above example, we get a `Publisher` out of the GraphQL context (it's automatically injected), and we use it to easily send a message to the subscriptions server on the `channel_name` channel. Our subscription from the previous example would pick this up and stream it to the client.

## Linking other services to subscriptions

Of course, it's entirely possible that services well beyond GraphQL may need to trigger a subscription message, and so you can easily push a message from anywhere where you can execute a basic HTTP request. Diana's subscriptions server has an inbuilt mutation `publish`, which takes a channel to publish on and a string message to publish. This can be called over a simple HTTP request from anywhere. However, this endpoint requires authentication, and you must have a valid JWT signed with the secret you've provided to be able to access it.
