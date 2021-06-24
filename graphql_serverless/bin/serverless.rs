#![forbid(unsafe_code)]

use netlify_lambda_http::{
    handler,
    lambda::{Context, run as run_lambda},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_lambda(handler(graphql)).await?;
    Ok(())
}

// TODO move this into the `lib` crate and import it here
async fn graphql(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok("Hello World!")
}
