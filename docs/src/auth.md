# Authentication

Authentication is built into Diana out of the box using JWTs. It's designed to be as intuitive as possible, but there are a few things you should know when working with it.

> 🚧 Authentication is not yet supported over subscriptions, this will be added soon! 🚧

## Authentication Block Level

In your configuration, you define a required level of authentication for your GraphQL endpoints using `.auth_block_state()`. The different levels are explained on [the configuration page](./config.md), so all that will be added now is that they apply to all GraphQL endpoints, from both the queries/mutations and the subscriptions systems. `BlockUnauthenticated` is vastly preferred and recommended in production.

## JWTs

Diana has full support for JWTs out of the box, and uses them internally to allow connections between its two systems. That means that you will need to create a JWT to enable this communication, which can be done using `diana::create_jwt`! Diana provides a few function for managing JWTs: `create_jwt`, `validate_and_decode_jwt`, `get_jwt_secret`, and `decode_time_str`. Those are all pretty self-explanatory except perhaps the last one, which turns strings like `1w` into one week from the present datetime in seconds after January 1st 1970 (Unix epoch), allowing you to more conveniently define JWT expiries. This is based on Vercel's [ms](https://github.com/vercel/ms) module for JavaScript, though only implements a subset of its features.

The documentation for those functions is best seen directly in raw form [here](https://docs.rs/diana). The most important thing to know is that the JWT for connecting to the subscriptions server MUST define the `role` property in its payload to be `graphql_server`. Otherwise authentication will fail for `BlockUnauthenticated` and `AllowMissing`.

## GraphiQL

GraphiQL is currently only supported in development (it will be disabled by force in production), and so there is as yet no need for authenticating for access to it. If and when it is usable in production, this will come with an authentication system for it.

In development, you may need to provide a JWT that you've generated in order to test authentication. You can do this by opening the *Headers* panel at the bottom of the screen and typing the following:

```json
{
	"Authorization": "Bearer YOUR_TOKEN_HERE"
}
```

Please note that authentication is not yet supported for subscriptions, and so this will have no effect on them (equivalent to a permanent `AllowAll`).
