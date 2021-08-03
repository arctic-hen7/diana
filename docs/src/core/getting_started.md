# Getting Started with Diana Core

Diana is built for use with integrations, but if you want to support a platform without an integration, you'll need to work with Diana core. This shouldn't be too daunting, as it's designed to work as well as possible with queries and mutations in particular. Subscriptions are not yet well supported in Diana Core, and we strongly advise using the [diana-actix-web](https://crates.io/crates/diana-actix-web) integration for your subscriptions server.

Diana core is just the `diana` package, which you should already have installed from [Getting Started](../getting_started.md).

This guide is designed to be as generic as possible, and it may be useful to have some perspective on how to actually build an integration, for which you should look to the [Actix Web integration](https://github.com/arctic-hen7/diana/tree/main/integrations/serverful/actix-web/src). That folder also contains examples of using `async_graphql` and its integrations more directly to support subscriptions (which is how you would probably do it if you were building your own integration).

Finally, if you build a fully-fledged integration for a serverful or serverless platform, please [submit a pull request](https://github.com/arctic-hen7/diana/pulls/new) to get it into the Diana codebase! We'd really appreciate your contribution! You can see our contributing guidelines [here](https://github.com/arctic-hen7/diana/tree/main/CONTRIBUTING.md)
