#![forbid(unsafe_code)]

use netlify_lambda_http::{
    handler,
    lambda::{Context as LambdaCtx, run as run_lambda},
    IntoResponse, Request, Response
};
use aws_lambda_events::encodings::Body;

use lib::{
    AuthCheckBlockState,
    OptionsBuilder,
    ServerlessResponse,
    run_serverless_req
};
use dev_utils::{
    ctx::Context,
    db::DbPool,
    schemas::users::{Query, Mutation, Subscription}
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_lambda(handler(graphql)).await?;
    Ok(())
}

// TODO move this into the `lib` crate and import it here
async fn graphql(req: Request, _: LambdaCtx) -> Result<impl IntoResponse, Error> {
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

    // Get the request body (query/mutation) as a string
    // Any errors are returned gracefully as HTTP responses
    let body = req.body();
    let body = match body {
        Body::Text(body_str) => body_str.to_string(),
        Body::Binary(_) => {
            let res = Response::builder()
                        .status(400) // Invalid request
                        .body("Found binary body, expected string".to_string())?;
            return Ok(res);
        },
        Body::Empty => {
            let res = Response::builder()
                        .status(400) // Invalid request
                        .body("Found empty body, expected string".to_string())?;
            return Ok(res);
        },
    };
    // Get the authorisation header as a string
    // Any errors are returned gracefully as HTTP responses
    let auth_header = req.headers().get("Authorization");
    let auth_header = match auth_header {
        Some(auth_header) => {
            let header_str = auth_header.to_str();
            match header_str {
                Ok(header_str) => Some(header_str),
                Err(_) => {
                    let res = Response::builder()
                                .status(400) // Invalid request
                                .body("Couldn't parse authorization header as string".to_string())?;
                    return Ok(res)
                }
            }
        },
        None => None
    };

    // Run the serverless request with those options
    // We convert the result to an appropriate HTTP response
    let res = run_serverless_req(body, auth_header, opts).await; // FIXME
    let res = match res {
        ServerlessResponse::Success(gql_res_str) => Response::builder()
                                                        .status(200) // GraphQL will handle any errors within it through JSON
                                                        .body(gql_res_str)?,
        ServerlessResponse::Blocked => Response::builder()
                                            .status(403) // Unauthorised
                                            .body("Request blocked due to invalid or insufficient authentication".to_string())?,
        ServerlessResponse::Error => Response::builder()
                                        .status(500) // Internal server error
                                        .body("An internal server error occurred".to_string())?
    };

    Ok(res)
}
