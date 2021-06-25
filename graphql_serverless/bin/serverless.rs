#![forbid(unsafe_code)]

use netlify_lambda_http::{
    handler,
    lambda::{Context as LambdaCtx, run as run_lambda},
    IntoResponse, Request
};
use lib::{
    OptionsBuilder,
    AuthCheckBlockState,
    AwsError,
    run_aws_req
};
use dev_utils::{
    ctx::Context,
    db::DbPool,
    schemas::users::{Query, Mutation, Subscription}
};

#[tokio::main]
async fn main() -> Result<(), AwsError> {
    run_lambda(handler(graphql)).await?;
    Ok(())
}

async fn graphql(req: Request, _: LambdaCtx) -> Result<impl IntoResponse, AwsError> {
    let opts = OptionsBuilder::new()
                    .ctx(Context {
                        pool: DbPool::default()
                    })
                    .subscriptions_server_hostname("http://subscriptions-server")
                    .subscriptions_server_port("6000")
                    .subscriptions_server_endpoint("/graphql")
                    .jwt_to_connect_to_subscriptions_server("blah")
                    .auth_block_state(AuthCheckBlockState::AllowAll)
                    .jwt_secret("blah")
                    .schema(Query {}, Mutation {}, Subscription {})
                    // Endpoints are set up as `/graphql` and `/graphiql` automatically
                    .finish().expect("Options building failed!");

    let res = run_aws_req(req, opts).await?;
    Ok(res)
}
