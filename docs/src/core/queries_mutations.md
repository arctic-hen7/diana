# Handling Queries and Mutations

The main struct you'll be dealing with here is [`DianaHandler`](https://docs.rs/diana/0.2.4/diana/struct.DianaHandler.html), and the API documentation for Diana is your friend here.

You can create a new `DianaHandler` by running `DianaHandler::new()` and providing it the `Options` you're using for your setup. That will automatically create schemas internally for queries/mutation and subscriptions. The two are mutually exclusive.

## Running a request

There are two functions you can use for running queries and mutations: `.run_stateless_for_subscriptions()` and `.run_stateless_without_subscriptions()`. The first uses the schema for the subscriptions system, which would be used basically only for running the internally used `publish` mutation. The latter is used for running the user's queries. If you're building for an unsupported platform, you'll need to support both if you want to support subscriptions.

Both functions take the same arguments because they do the same thing, just with different schemas. First, they both take a string request body, which is NOT the query the user wrote! Rather, that should be the stringified JSON body that contains fields for the `query`, `variables`, etc. If you make that mistake, you'll get some very strange errors about schema validity no matter what you do!

The second argument is an `Option` of a string authentication header, which should be the raw value extracted from the HTTP `Authorization` header (which is where JWTs will be given). Do NOT try to pre-parse this in any way, even resolving it to a string, that will all be handled internally.

The third and final argument is an optional authentication verdict, which can be given to force the handling process to not run any authentication checks on the given token, but rather to use a predetermined verdict. This allows the use of authentication middleware to arrive at a verdict before all the HTTP data has been streamed in (more efficient). You can learn more about this [here](./auth.md). If you're not using middleware (not recommended unless you really can't), you should provide `None` here.
