# Getting Started

Diana is a high-level wrapper around [async_graphql](https://crates.io/crates/async-graphql), and is designed to be as easy as possible to get started with! This page is a basic tutorial of how to get started with a full and working setup.

## Installation

Assuming you have Rust already installed (if not, [here's](https://www.rust-lang.org/tools/install) a guide on how to do so), you can add Diana as a dependency to your project easily by adding the following to your project's `Cargo.toml` under `[dependencies]`:

```
diana = "0.2.6"
async-graphql = "2.8.2"
```

We also install `async_graphql` directly to prevent errors with asynchronous usage. Now run `cargo build` to download all dependencies. Diana is large and complex, so this will take quite a while!

If you're new to GraphQL, we highly recommend reading more about it before diving further into Diana. You can see more about it on the [official GraphQL website](https://graphql.org).

## Project Structure

We recommend a specific project structure for new projects using Diana, but it's entirely optional! It is however designed to minimize code duplication and maximize efficiency by allowing you to run both the queries/mutations system and the subscriptions server simultaneously.

Critically, we recommend having three binary crates for your two servers and the serverless function, as well as a library crate for your schemas and configurations. These should all be Cargo workspaces.

```
lib/
	src/
		lib.rs
	Cargo.toml
server/
	src/
		main.rs
	Cargo.toml
serverless/
	src/
		main.rs
	Cargo.toml
subscriptions/
	src/
		main.rs
	Cargo.toml
Cargo.lock
Cargo.toml
```

Set this up for now if possible, and we'll add to it later across the book (it will be assumed that you're using this or something similar).

You should also have the following in your root `Cargo.toml` to set up workspaces (which you can read more about [here](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)):

```toml
[workspace]

members = [
	"lib",
    "server",
    "serverless",
	"subscriptions"
]
```

Then you can make all the binary crates (everything except `lib`) dependent on your shared logic by adding this to the `Cargo.toml` files in `server`, `serverless`, and `subscriptions` under `[dependencies]`:

```toml
lib = { path = "../lib" }
```

You can then reference `lib` in those crates as if it were just another external module!

## Your first schema

If you're familiar with GraphQL, then the first thing you'll need to do to set up Diana is to write a basic schema. Diana depends entirely on [async_graphql](https://crates.io/crates/async-graphql) for this, so [their documentation](https://async-graphql.github.io) may also help you (particularly in more advanced cases), though this book should be enough for the simple stuff.

Your first schema can be really simple, we'll just make a simple query that reports the API version when queried (we won't add any mutations or subscriptions for now). Try adding this somewhere in your shared logic:

```rust
use diana::{
	async_graphql::{
		Object as GQLObject
	}
}

#[derive(Default, Clone)]
pub struct Query {}
#[GQLObject]
impl Query {
    async fn api_version(&self) -> &str {
        "0.1.0"
    }
}
```

This is probably the simplest schema you'll ever create! Crucially though, you MUST derive the `Default` and `Clone` traits on it. The former is required by `async_graphql`, and the latter for Diana.

Hopefully you can see that our `Query` object is simply defining one query, `api_version`, which just returns `0.1.0`, the version of our API! Conveniently, `async_graphql` automatically parses this into the more conventional `apiVersion` when we call this, so you can conform to Rust and GraphQL conventions at the same time!

## Your first options

Every part of Diana is configured using the `Options` struct, which can be created with `Options::builder()`. For now, we'll set up a simple configuration without any subscriptions support. Add this to your shared logic:

```rust
use diana::{Options, AuthBlockState};
use diana::async_graphql::{EmptyMutation, EmptySubscription};
use crate::Query; // Or wherever you put your `Query` object from the previous section

#[derive(Clone)]
pub struct Context(String);

pub fn get_opts() -> Options<Context, Query, EmptyMutation, EmptySubscription> {
    Options::builder()
        .ctx(Context("test".to_string()))
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret("this is a secret")
        .schema(Query {}, Mutation {}, Subscription {})
        .finish()
        .expect("Failed to build options!")
}
```

Notice that we define a `Context` struct here. This will get passed around to every GraphQL resolver and you'll always be able to access it. As long as it's `Clone`able, you can put anything in here safely. A common use-case of this in reality would be as a database connection pool. Here, we just define it with a random string inside.

Next, we define a function `get_opts()` that initializes our `Options`. We set the context, define our schema, and do two other things that need some explaining. The first is `.auth_block_state()`, which sets the required authentication level to access our GraphQL endpoint. Diana has authentication built-in, so this is fundamental. Here, we allow anything, authenticated or not, for educational purposes. In a production app, set this to block everything! You can read more about authentication [here](./auth.md). The second thing that needs explaining is `.jwt_secret()`. Diana's authentication systems is based on JWTs, which are basically tokens that clients send to servers to prove their identity (the server signed them earlier, and so can verify them). JWTs need a secret to be based on, and we define a very silly one here. In a production app, you should read this from an environment variable and it should be randomly generated (more on that [here](./auth.md)).

## Your first server

Let's try plugging this into a basic Diana server! Diana is based around integrations for different platforms, and it currently supports only Actix Web for serverful systems, so that's what we'll use! You should add this to your `Cargo.toml` in the `server` crate under `[dependencies]`:

```toml
diana-actix-web = "0.2.6"
```

Now add the following to your `main.rs` in the `server` crate:

```rust
use diana_actix_web::{
    actix_web::{App, HttpServer},
    create_graphql_server,
};
use diana::async_graphql::{EmptyMutation, EmptySubscription};
use lib::{get_opts, Query}

#[diana_actix_web::actix_web::main]
async fn main() -> std::io::Result<()> {
    let configurer = create_graphql_server(get_opts()).expect("Failed to set up configurer!");

    HttpServer::new(move || App::new().configure(configurer.clone()))
        .bind("0.0.0.0:9000")?
        .run()
        .await
}
```

Firstly, we're pulling in the dependencies we need, including the schema and the function to get our `Options`. Then, we define an asynchronous `main` function marked as the entrypoint for `actix_web`, in which we set up our entire GraphQL server using `create_graphql-server()`, parsing in our `Options`. After that, we start up a new Actix Web server, using `.configure()` to configure the entire thing. Pretty convenient, huh?

If you also have some REST endpoints or the like, you can easily add them to this server as well, `.configure()` is inbuilt into Actix Web to enable this kind of modularization.

## Firing it up

The last step of all this is to actually run your server! Go into the `server` crate and run `cargo run` to see it in action! If all has gone well, you should see be able to see the GraphiQL playground (GUI for GraphQL development) in your browser at <http://localhost:9000/graphiql>! Try typing in the following and then run it!

```
query {
	apiVersion
}
```

You should see `0.1.0` faithfully printed on the right-hand side of the screen.

Congratulations! You've just set up your first GraphQL server with Diana! The rest of this book will help you to understand how to extend this setup to include mutations, subscriptions, and authentication, as well as helping you to deploy it all serverlessly!
