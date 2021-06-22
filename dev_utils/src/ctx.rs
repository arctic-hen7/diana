use mongodb::Client as MongoClient;
use lib::errors::*;
use crate::db::DbPool;

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
