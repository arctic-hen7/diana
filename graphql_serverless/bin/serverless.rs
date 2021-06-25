#![forbid(unsafe_code)]

use lib::{
    OptionsBuilder,
    AuthCheckBlockState,
    AwsError,
    run_aws_req,

    create_handler,
    LambdaCtx, run_lambda,
    IntoLambdaResponse, LambdaRequest
};
use dev_utils::{
    ctx::Context,
    db::DbPool,
    schemas::users::{Query, Mutation, Subscription}
};

#[tokio::main]
async fn main() -> Result<(), AwsError> {
    run_lambda(create_handler(graphql)).await?;
    Ok(())
}

async fn graphql(req: LambdaRequest, _: LambdaCtx) -> Result<impl IntoLambdaResponse, AwsError> {
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
