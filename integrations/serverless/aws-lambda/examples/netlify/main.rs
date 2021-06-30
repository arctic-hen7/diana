#![forbid(unsafe_code)]

use diana_aws_lambda::{run_aws_req, AwsError, IntoLambdaResponse, LambdaCtx, LambdaRequest};
use netlify_lambda_http::lambda;

// This 'dirty-imports' the code in `schema.in`
// It will literally be interpolated here
// Never use this in production unless you have a fantastic reason! Just import your code through Cargo!
// We do this here though because you can't import from another example (which is annoying)
include!("../../../../../examples/schema/schema.rs");

// Make sure you don't forget to add the JWT_SECRET and SUBSCRIPTIONS_SERVER_PUBLISH_JWT to your Netlify (based on the options in `schema.rs`)!
#[lambda(http)]
#[tokio::main]
async fn main(req: LambdaRequest, _: LambdaCtx) -> Result<impl IntoLambdaResponse, AwsError> {
    let res = run_aws_req(req, get_opts()).await?;
    Ok(res)
}
