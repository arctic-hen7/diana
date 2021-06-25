use std::env;
use mongodb::{
    Client as MongoClient,
    options::{ClientOptions, StreamAddress, Credential},
};

use crate::errors::*;
use crate::load_env::load_env;

// A helper function for implementations of the DbClient trait that gets a handle to a DB client from environment variables
// All errors are given in GraphQL format, seeing as this function will be called in resolver logic and conversion is annoying
pub fn get_client() -> Result<MongoClient> {
    load_env()?;
    // Get all the necessary configuration from environment variables
    let hostname = env::var("DB_HOSTNAME")?;
    let port = env::var("DB_PORT")?
        .parse::<u16>() // Ports are not going to be larger than a 16-bit integer
        .map_err(|_err| ErrorKind::InvalidEnvVarType("DB_PORT".to_string(), "number".to_string()))?;
    let username = env::var("DB_USERNAME")?;
    let password = env::var("DB_PASSWORD")?;
    let options =
        ClientOptions::builder()
            .hosts(vec![
                StreamAddress {
                    hostname,
                    port: Some(port),
                }
            ])
            .credential(
                Credential::builder()
                    .username(username)
                    .password(password)
                    .build()
            )
            .build();
    let client = MongoClient::with_options(options)?;

    Ok(client)
}

// The MongoDB crate handles pooling internally, so we don't have to worry about it here
// We just need a struct that exposes methods to get a client
// If extra pooling logic ever needs to be added, it can be done from here
#[derive(Clone, Default)]
pub struct DbPool {}
impl DbPool {
    pub fn get_client(&self) -> Result<MongoClient> {
        // Check if we already have a client cached
        let client = get_client()?;

        Ok(client)
    }
}
