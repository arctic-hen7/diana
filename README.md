# Diana

Diana is an out-of-the-box fully-fledged GraphQL system built in Rust with inbuilt support for commonly-used features like subscriptions and authentication. It was built to allow a simple but fully-featured GraphQL system to be very quickly created for systems that have complex data structures and no time for boilerplate.

## Structure

**TL;DR:** Diana has one server for queries/mutations and another one for subscriptions so we can do serverless stuff. This is secured and you don't have to worry about it!

Diana is a dual-structured system: one server for queries and mutations and another for subscriptions. These two systems then communicate over a JWT-authenticated channel that you don't have to worry about! The reason for this architecture is to allow the easier adoption of a serverless system in the future. Diana is currently working towards full serverless support for Netlify for all queries and mutations, which will reduce the operation costs of running Diana in production significantly!

However, serverless functions, for all their benefits, must be stateless. GraphQL subscriptions are by definition stateful, meaning the two are incompatible. It is possible to have an intermediary server manage WebSockets and then forward these to a pseudo-subscription stateless backend on a serverless function, though this idea was rejected in Diana's early development because it requires (in the cases we examined) the writing of subscriptions in a stateless way, which is not idiomatic and confusing. Further, this architecture is brittle and difficult to maintain.

Instead, if you need to publish some data to a subscription, all you have to do is send the `publish` mutation to the subscriptions server and provide an appropriate authentication token. That entire process is abstracted for you if you're doing it from within Rust. If you're doing it from something else, it's all over HTTP and thus relatively easy!

## How do I use it?

### Getting Started

Getting started with Diana is really easy! You don't even need a database to get started! Just install it by adding this to your `Cargo.toml` file:

```
diana = "0.1.0"
```

Due to the complexity of its components, Diana does have a lot of dependencies, so you may want to go and have a cup of tea while you wait for the installation and everything to be compiled for the first time!

Because of its structure, Diana needs you to run two servers in development. While it may be tempting to just combine these into one, this will not work (schema collisions)! You can either have two binaries or, using our recommended method, create a monorepo-style crate with two binary crates and a library crate to store your common logic (example TODO).

TODO further docs

## Why the name?

_Diana_ is the Roman name for the Greek goddess _Artemis_, the sister of the god _Apollo_. Apollo is also a GraphQL-focused company, and Diana is an open-source GraphQL system for Rust (a language Apollo presently has no support for). Diana has absolutely no affiliation with Apollo the company though!

## Roadmap

-   [ ] Write tests for existing code
-   [ ] Support GraphiQL over serverless
-   [ ] Support authentication over WebSockets for subscriptions

## Contributing

Contributing guidelines can be found [here](./CONTRIBUTING.md), and please make sure you follow our [code of conduct](CODE_OF_CONDUCT.md).

## License

See [`LICENSE`](./LICENSE).
