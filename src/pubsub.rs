// This module defines a simple publish-subscribe structure, though one designed to run across the web
// The publishing and subscribing are done on different servers/functions

use async_stream::stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast::{channel as create_channel, Sender};
use tokio_stream::Stream;

use crate::errors::*;

const MESSAGES_TO_BE_RETAINED: usize = 5;

#[derive(Serialize)]
struct GQLQueryBody<T: Serialize> {
    query: String,
    variables: T,
}

#[derive(Deserialize)]
struct GQLPublishResponse {
    data: PublishResponse,
}
#[derive(Deserialize)]
struct PublishResponse {
    publish: bool,
}

/// The system that publishes data from the queries/mutations system to the subscriptions server.
/// These communications are secured by a JWT specified in [`Options`](crate::Options).
/// This is automatically created from the [`Options`](crate::Options) and passed to all resolvers. You should never need to manually create it.
pub struct Publisher {
    client: Client,
    address: String,
    token: String,
}
impl Publisher {
    /// Creates a new publisher. This is done for you when you create the queries/mutations system, so you should never need to call this.
    pub fn new(hostname: String, port: String, endpoint: String, token: String) -> Result<Self> {
        let address = format!(
            "{hostname}:{port}{endpoint}", // The endpoint should start with '/'
            hostname = hostname,
            port = port,
            endpoint = endpoint
        );

        let client = Client::new();

        Ok(Self {
            client,
            address,
            token,
        })
    }

    /// Sends the given data to the subscriptions server on the given channel. In-depth information about this process is available in the book.
    /// You should use [serde] to serialize anything sent here as a string (this won't be done for you). It should then be deserialized in the
    /// appropriate subscription (which will listen for messages from here indirectly).
    /// This function will return an error if the subscriptions server was unavailable or didn't correctly acknowledge the request.
    /// # Example
    /// ```
    /// let publisher = ctx.data::<Publisher>()?; // Where `ctx` is the `async_graphql` context to a resolver.
    /// publisher.publish("new_user", user_json.to_string()).await?;
    /// ```
    pub async fn publish(&self, channel: &str, data: String) -> Result<()> {
        let client = &self.client;

        // Create the query body with a HashMap of variables
        let mut variables = HashMap::new();
        variables.insert("channel", channel.to_string());
        variables.insert("data", data);

        let body = GQLQueryBody {
            query: "
                mutation PublishData($channel: String!, $data: String!) {
                    publish(
                        channel: $channel,
                        data: $data
                    )
                }
            "
            .to_string(),
            variables,
        };

        let res = client
            .post(&self.address)
            .json(&body)
            .header("Authorization", "Bearer ".to_string() + &self.token)
            .send()
            .await?;

        // Handle if the request wasn't successful on an HTTP level
        if res.status().to_string() != "200 OK" {
            bail!(ErrorKind::SubscriptionDataPublishFailed)
        }

        // Get the body out (data still stringified though, that's handled by resolvers)
        let body: GQLPublishResponse = serde_json::from_str(&res.text().await?)?;

        // Confirm nothing's gone wrong on a GraphQL level (basically only if we got `false` instead of `true`)
        match body.data.publish {
            true => Ok(()),
            _ => bail!(ErrorKind::SubscriptionDataPublishFailed),
        }
    }
}

// Everything from here down operates solely on the subscriptions server, and is stateful!
// Do NOT import these mechanisms in the serverless system!

// This is a traditional PubSub implementation using Tokio's broadcast system
// This doesn't need to be made available because it's entirely internal
pub struct PubSub {
    // A hash map of channels to their Tokio broadcasters
    channels: HashMap<String, Sender<String>>,
}
impl Default for PubSub {
    fn default() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }
}
impl PubSub {
    // Gets a channel or creates a new one if needed
    fn get_channel(&mut self, channel: &str) -> Sender<String> {
        let channel_sender = self.channels.get(channel);
        let channel_sender = match channel_sender {
            Some(sub) => sub,
            None => {
                let (channel_sender, _receiver) = create_channel(MESSAGES_TO_BE_RETAINED);
                self.channels.insert(channel.to_string(), channel_sender); // We put a clone into the HashMap because broadcast can be multi-producer
                self.channels.get(channel).unwrap() // We just added it, we know more than the compiler
            }
        };

        channel_sender.clone()
    }

    pub fn subscribe(&mut self, channel: &str) -> impl Stream<Item = String> {
        let channel_sender = self.get_channel(channel);
        let mut receiver = channel_sender.subscribe();

        stream! {
            loop {
                let message = receiver.recv().await;
                match message {
                    Ok(message) => yield message,
                    _ => continue
                }
            }
        }
    }

    // Creates a new sender for a given channel name if one doesn't exist and then sends a message using it
    pub fn publish(&mut self, channel: &str, data: String) {
        let channel_sender = self.get_channel(channel);
        // This will fail only if there are now receivers, but we don't care if that's the case
        let _ = channel_sender.send(data);
    }

    // Drops the handle to a sender for the given channel
    // All receiver calls after this point will result in a closed channel error
    // This doesn't need to be explicitly called normally
    pub fn close_channel(&mut self, channel: &str) {
        self.channels.remove(channel);
    }
}
