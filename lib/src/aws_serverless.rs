// This file contains serverless logic unique to AWS Lambda and its derivatives (e.g. Netlify)

use async_graphql::{ObjectType, SubscriptionType};
use aws_lambda_events::encodings::Body;
use netlify_lambda_http::{Request, Response};
use std::any::Any;

use crate::options::Options;
use crate::serverless::{run_serverless_req, ServerlessResponse};

/// A *very* generic error type that the lambda will accept as a return type.
pub type AwsError = Box<dyn std::error::Error + Send + Sync + 'static>;

// This allows us to propagate error HTTP responses more easily
enum AwsReqData {
    Valid((String, Option<String>)),
    Invalid(Response<String>), // For some reason
}

// Gets the stringified body and authentication header from an AWS request
// We use a generic error type rather than the crate's `error_chain` logic here for AWS' benefit
fn get_data_from_aws_req(req: Request) -> Result<AwsReqData, AwsError> {
    // Get the request body (query/mutation) as a string
    // Any errors are returned gracefully as HTTP responses
    let body = req.body();
    let body = match body {
        Body::Text(body_str) => body_str.to_string(),
        Body::Binary(_) => {
            let res = Response::builder()
                .status(400) // Invalid request
                .body("Found binary body, expected string".to_string())?;
            return Ok(AwsReqData::Invalid(res));
        }
        Body::Empty => {
            let res = Response::builder()
                .status(400) // Invalid request
                .body("Found empty body, expected string".to_string())?;
            return Ok(AwsReqData::Invalid(res));
        }
    };
    // Get the authorisation header as a string
    // Any errors are returned gracefully as HTTP responses
    let auth_header = req.headers().get("Authorization");
    let auth_header = match auth_header {
        Some(auth_header) => {
            let header_str = auth_header.to_str();
            match header_str {
                Ok(header_str) => Some(header_str.to_string()),
                Err(_) => {
                    let res = Response::builder()
                        .status(400) // Invalid request
                        .body("Couldn't parse authorization header as string".to_string())?;
                    return Ok(AwsReqData::Invalid(res));
                }
            }
        }
        None => None,
    };

    Ok(AwsReqData::Valid((body, auth_header)))
}

// Parses the response from `run_serverless_req` into HTTP responses that AWS Lambda (or derivatives) can handle
fn parse_aws_res(res: ServerlessResponse) -> Result<Response<String>, AwsError> {
    let res = match res {
        ServerlessResponse::Success(gql_res_str) => Response::builder()
            .status(200) // GraphQL will handle any errors within it through JSON
            .body(gql_res_str)?,
        ServerlessResponse::Blocked => Response::builder()
            .status(403) // Unauthorised
            .body("Request blocked due to invalid or insufficient authentication".to_string())?,
        ServerlessResponse::Error => Response::builder()
            .status(500) // Internal server error
            .body("An internal server error occurred".to_string())?,
    };

    Ok(res)
}

/// Runs a request for AWS Lambda or its derivatives (e.g. Netlify).
/// This just takes the entire Lambda request and does all the processing for you, but it's really just a wrapper around [run_serverless_req].
/// You should use this function in your Lambda handler.
/// # Example
/// ```
/// use diana::{
///     create_handler, run_aws_req, run_lambda, AuthCheckBlockState, AwsError, IntoLambdaResponse,
///     LambdaCtx, LambdaRequest, OptionsBuilder,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), AwsError> {
///     run_lambda(create_handler(graphql)).await?;
///     Ok(())
/// }
///
/// async fn graphql(req: LambdaRequest, _: LambdaCtx) -> Result<impl IntoLambdaResponse, AwsError> {
///     let opts = OptionsBuilder::new()
///         .ctx(Context {
///             pool: DbPool::default(),
///         })
///         .subscriptions_server_hostname("http://subscriptions-server")
///         .subscriptions_server_port("6000")
///         .subscriptions_server_endpoint("/graphql")
///         .jwt_to_connect_to_subscriptions_server("blah")
///         .auth_block_state(AuthCheckBlockState::AllowAll)
///         .jwt_secret("blah")
///         .schema(Query {}, Mutation {}, Subscription {})
///         // Endpoints are set up as `/graphql` and `/graphiql` automatically
///         .finish()
///         .expect("Options building failed!");
///
///     let res = run_aws_req(req, opts).await?;
///     Ok(res)
/// }
/// ```
pub async fn run_aws_req<C, Q, M, S>(
    req: Request,
    opts: Options<C, Q, M, S>,
) -> Result<Response<String>, AwsError>
where
    C: Any + Send + Sync + Clone,
    Q: Clone + ObjectType + 'static,
    M: Clone + ObjectType + 'static,
    S: Clone + SubscriptionType + 'static,
{
    // Process the request data into what's needed
    let req_data = get_data_from_aws_req(req)?;
    let (body, auth_header) = match req_data {
        AwsReqData::Valid(data) => data,
        AwsReqData::Invalid(http_res) => return Ok(http_res), // Propagate any HTTP responses for errors
    };

    // Run the serverless request with the extracted data and the user's given options
    // We convert the Option<String> to Option<&str> with `.as_deref()`
    let res = run_serverless_req(body, auth_header.as_deref(), opts).await;

    // Convert the result to an appropriate HTTP response
    let http_res = parse_aws_res(res)?;
    Ok(http_res)
}
