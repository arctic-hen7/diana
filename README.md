<h1 align="center">Diana</h1>

> **Pragmatic GraphQL that just works.**

[Book][book] • [Crate Page][crate] • [API Documentation][docs] • [Contributing][contrib]

Diana is a GraphQL system for Rust that's designed to work as simply as possible out of the box, without sacrificing configuration ability. Unlike other GraphQL systems, Diana **fully supports serverless functions and automatically integrates them with a serverful subscriptions system** as needed, and over an authenticated channel. GraphQL subscriptions are state*ful*, and so have to be run in a server*ful* way. Diana makes this process as simple as possible.

Diana's documentation can be found in [the book][book].

## Installation

Getting started with Diana is really easy! Just install it by adding this to your `Cargo.toml` file:

```
diana = "0.2.4"
```

Due to the complexity of its components, Diana does have a lot of dependencies, so you may want to go and have a cup of tea while you wait for the installation and everything to be compiled for the first time!

Because of its structure, Diana needs you to run two servers in development. While it may be tempting to just combine these into one, this will not work whatsoever and it will blow up in your face (schema collisions)! You can either have two binaries or, using our recommended method, create a monorepo-style crate with two binary crates and a library crate to store your common logic (example in the book).

All further documentation can be found in [the book][book], which was made with [mdBook](https://rust-lang.github.io/mdBook/index.html).

## Versioning

Each Diana integration depends on the core library, so any change of the core library will result in a version change for an integration. That is also applied backwards in that any version change in an integration also results in a version change of the core and all other integrations. Essentially, the whole of Diana will always be at a certain version, the latest tag of this repository.

When a new version is added, it will begin in `v0.1.0`. Once it moves to a stable release, what would otherwise be `v1.0.0`, it is immediately bumped to the same version as the rest of the Diana ecosystem.

## Stability

Diana is under active development, and still requires the particular addition of support for authentication over WebSockets. The project will hopefully move to v1.0.0 by 2022!

## Credit to `async_graphql`

[`async_graphql`](https://github.com/async-graphql/async-graphql) must be acknowledged as the primary dependency of Diana, as well as the biggest inspiration for the project. It is a fantastic GraphQL library for Rust, and if you want to go beyond the scope of Diana (which is more high-level), this should be your first port of call. Without it, Diana would not be possible at all.

## Why the name?

_Diana_ is the Roman name for the Greek goddess _Artemis_, the sister of the god _Apollo_. [Apollo GraphQL](https://www.apollographql.com/) is a company that builds excellent GraphQL products (with which Diana is NOT in any way affiliated), so we may as well be in the same nominal family (if that's a thing).

## Roadmap

-   [ ] Support GraphiQL in production

*   [ ] Support authentication over WebSockets for subscriptions
*   [ ] Support GraphiQL over serverless

## Contributing

If you want to make a contribution to Diana, that's great! Thanks so much! Contributing guidelines can be found [here](contrib), and please make sure you follow our [code of conduct](CODE_OF_CONDUCT.md).

## License

See [`LICENSE`](./LICENSE).

[book]: https://diana-graphql.github.io
[crate]: https://crates.io/crates/diana
[docs]: https://docs.rs/diana
[contrib]: ./CONTRIBUTING.md
