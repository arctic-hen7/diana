#![forbid(unsafe_code)]

mod schema;

use diana::{
    run_aws_req, AwsError, IntoLambdaResponse,
    LambdaCtx, LambdaRequest,
};
use netlify_lambda_http::{lambda};
use crate::schema::get_opts;

// Make sure you don't forget to add the JWT_SECRET and SUBSCRIPTIONS_SERVER_PUBLISH_JWT to your Netlify (based on the options in `schema.rs`)!
#[lambda(http)]
#[tokio::main]
async fn main(req: LambdaRequest, _: LambdaCtx) -> Result<impl IntoLambdaResponse, AwsError> {
    let res = run_aws_req(req, get_opts()).await?;
    Ok(res)
}
