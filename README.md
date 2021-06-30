# Diana

Diana is an out-of-the-box fully-fledged GraphQL system built in Rust with inbuilt support for commonly-used features like subscriptions and authentication. It was built to allow a simple but fully-featured GraphQL system to be very quickly created for systems that have complex data structures and no time for boilerplate.

## Structure

**TL;DR:** Diana has one server for queries/mutations and another one for subscriptions so we can do serverless stuff. This is secured and you don't have to worry about it!

Diana is a dual-structured system: one server for queries and mutations and another for subscriptions. These two systems then communicate over a JWT-authenticated channel that you don't have to worry about! The reason for this architecture is to allow the use of serverless functions for queries and mutations, which can be executed in this way at often lower hosting cost.

However, serverless functions, for all their benefits, must be stateless. GraphQL subscriptions are by definition stateful, meaning the two are incompatible. It is possible to have an intermediary server manage WebSockets and then forward these to a pseudo-subscription stateless backend on a serverless function, though this idea was rejected in Diana's early development because it requires (in the cases we examined) the writing of subscriptions in a stateless way, which is not idiomatic and confusing. Further, this architecture is brittle and difficult to maintain.

Instead, if you need to publish some data to a subscription, all you have to do is send the `publish` mutation to the subscriptions server and provide an appropriate authentication token. That entire process is abstracted for you if you're doing it from within Rust. If you're doing it from something else, it's all over HTTP and thus relatively easy!

## Integrations

The Diana core library just exposes basic primitives that can be used to execute GraphQL requests from limited data (a request body and the HTTP `Authorization` header), but this can be cumbersome to use, especially once moving authentication into middleware pops up. As a solution, Diana offers separate *integration* crates (much like `async_graphql`) for various serverful and serverless frameworks, with the aim of creating wrappers that allow you to deploy a fully-featured GraphQL server in literally seconds (after you've made your schema and compiled everything that is). If you're using a system that isn't yet supported as an integration, you can still use the core Diana library to build your own integration relatively simply. If you do this, please move that integration into its own repository and open a pull request here! We'd love to have more integrations supported in future, and communal development is the best way to achieve that goal.

Note also that each integration has its own set of examples within this repository (under the `integrations` folder) and documentation in the book.

These are currently available integrations:

- Server*ful* systems
	- [Actix Web](https://crates.io/crates/diana-actix-web)
- Server*less* systems
	- [AWS Lambda and derivatives](https://crates.io/crates/diana-aws-lambda) (this supports anything that wraps AWS Lambda as well, like Netlify)

## Caveats

Right now, there are a few things Diana does not support. These are listed here, and will all be finalised before v1.0.0 unless otherwise indicated.

- Authentication over WebSockets (the protocol has no support for bearer tokens, so a more novel solution is required)
- GraphiQL playground in production (this requires tweaking a few things to provide more generic authentication middleware that can work with integrations to appropriately do this)
- GraphiQL playground over serverless (won't work on unless requested)

## How do I use it?

### Getting Started

Getting started with Diana is really easy! You don't even need a database to get started! Just install it by adding this to your `Cargo.toml` file:

```
diana = "0.2.0"
```

Due to the complexity of its components, Diana does have a lot of dependencies, so you may want to go and have a cup of tea while you wait for the installation and everything to be compiled for the first time!

Because of its structure, Diana needs you to run two servers in development. While it may be tempting to just combine these into one, this will not work whatsoever and it will blow up in your face (schema collisions)! You can either have two binaries or, using our recommended method, create a monorepo-style crate with two binary crates and a library crate to store your common logic (example in the book).

All further documentation can be found in [the book](https://diana-graphql.github.io), which was made with [mdBook](https://rust-lang.github.io/mdBook/index.html).

## Credit to `async_graphql`

[`async_graphql`](https://github.com/async-graphql/async-graphql) must be acknowledged as the primary dependency of Diana, as well as the biggest inspiration for the project. It is a fantastic GraphQL library for Rust, and if you want to go beyond the scope of Diana (which is more high-level), this should be your first port of call. Without it, Diana would not be possible at all.

## Why the name?

_Diana_ is the Roman name for the Greek goddess _Artemis_, the sister of the god _Apollo_. [Apollo GraphQL](https://www.apollographql.com/) is a company that builds excellent GraphQL products (with which Diana is NOT in any way affiliated), so we may as well be in the same nominal family (if that's a thing).

## Roadmap

* [ ] Write tests for existing code
* [ ] Support GraphiQL in production
-   [ ] Support authentication over WebSockets for subscriptions
-   [ ] Support GraphiQL over serverless

## Contributing

If you want to make a contribution to Diana, that's great! Thanks so much! Contributing guidelines can be found [here](./CONTRIBUTING.md), and please make sure you follow our [code of conduct](CODE_OF_CONDUCT.md).

## License

See [`LICENSE`](./LICENSE).
