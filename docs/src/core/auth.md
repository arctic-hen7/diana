# Authentication

If you're not using any middleware, you can entirely ignore this page and get on with building your custom system, but if you want to authenticate users more efficiently, this is for you.

`DianaHandler` has the function `.is_authed()` that you can call in middleware, parsing in a raw authentication header just as you would if you were [handling queries and mutations](./queries_mutations.md) without middleware. That will return an [`AuthVerdict`](https://docs.rs/diana/0.2.6/diana/enum.AuthVerdict.html), which tells you if the client is allowed, blocked, or if an error occurred. Typically, you would continue the request on `Allow`, return a 403 on `Block`, and return a 500 on `Error` (though this could be caused by a bad request, it occurs in the context of the server). In future, a distinction may be made between server and client caused errors, which would allow reasonable returning of a 400 in some cases, but that's not yet implemented.

After you have an `AuthVerdict`, you can send that to your final handler in some way (Actix Web uses request extensions) and then extract it there to provide to `run_stateless_without_subscriptions()` or `.run_stateless_for_subscriptions`. If you do that, you don't need to provide the raw authentication header, as it won't be used, but you still can.
