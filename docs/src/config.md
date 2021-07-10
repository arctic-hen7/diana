# Configuration

Diana is configured using the `Options` struct. This page will go through in detail what can be specified using that system. Here's an example options initialization that we'll work through:

```rust
Options::builder()
        .ctx(Context("test".to_string()))
        .subscriptions_server_hostname("http://localhost")
        .subscriptions_server_port("9002")
        .subscriptions_server_endpoint("/graphql")
        .jwt_to_connect_to_subscriptions_server(
            &env::var("SUBSCRIPTIONS_SERVER_PUBLISH_JWT").unwrap(),
        )
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret(&env::var("JWT_SECRET").unwrap())
        .schema(Query {}, Mutation {}, Subscription {})
        .graphql_endpoint("/graphql")
		.playground_endpoint("/graphiql")
        .finish()
        .expect("Failed to build options!")
```

## Context

You must provide a context struct to Diana by using the `.ctx()` function. This struct will be parsed to all resolvers, and can be trivially accessed by using this in your resolvers:

```rust
let ctx = raw_ctx.data<Context>()?;
```

A common use of the context struct is for a database pool.

## Subscriptions server configuration

You need to provide the details of the subscriptions server in your configuration so the queries/mutation system knows where it is on the internet. This is defined using these four functions:

- `.subscriptions_server_hostname()` -- the hostname of the subscriptions server (e.g. `http://localhost`
- `.subscriptions_server_port()` -- the port the subscriptions server is running on
- `.subscriptions_server_endpoint()` -- the GraphQL endpoint to connect to on the subscriptions server (e.g. `/graphql`)
- `.jwt_to_connect_to_subscriptions_server()` -- a JWT to use to authenticate against the subscriptions server, which must be signed with the secret define by `.jwt_secret()`; this JWT must have a payload which defines `role: "graphql_server"` (see [Authentication](./auth.md))

If you aren't using subscriptions at all in your setup, you don't have to use any of these functions.

## Authentication

Two properties define authentication data for Diana: `.jwt_secret()` and `.auth_block_state()`. The former defines the string secret to use to sign all JWTs (internally used for the communication channel between the two systems of Diana, you can use it too for authenticating clients). The latter defines the level of authentication required to connect to the GraphQL endpoint. This can be one of the following:

- `AuthBlockLevel::AllowAll` -- allows everything, only ever use this in development unless you have an excellent reason
- `AuthBlockLevel::BlockUnauthenticated` -- blocks anything without a valid JWT
- `AuthBlockLevel::AllowMissing` -- blocks invalid tokens, but allows requests without tokens; this is designed for development use to show authentication while also allowing GraphiQL introspection (the hints and error messages like an IDE); do NOT use this in production!

## Endpoints

The two functions `.graphql_endpoint()` and `.playground_endpoint` define the locations of your GraphQL endpoint and the endpoint for the GraphiQL playground, though you probably won't use them unless you're using something novel, they are set to `/graphql` and `/graphiql` respectively by default.

## Schema

The last function is `.schema()`, which defines the actual schema for your app. You'll need to provide your `Query`, `Mutation` and `Subscription` types here. If you're not using subscriptions, you can use `diana::async_graphql::EmptySubscription` instead. There's also an `EmptyMutation` type if you need it. At least one query is mandatory. You should initialize each of these structs for this function with this notation:

```
Query {}
```

## Building it

Once you've run all those functions, you can build the `Options` by using `.finish()`, which will return a `Result<Options, diana::errors::Error>`. Because you can't do anything at all without the options defined properly, it's typical to run `.expect()` after this to `panic!` quickly if the configuration couldn't be built.
