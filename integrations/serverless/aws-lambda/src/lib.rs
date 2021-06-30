mod run_aws_req;

pub use crate::run_aws_req::{run_aws_req, AwsError};

// Users also shouldn't have to install the Netlify stuff themselves for basic usage
pub use netlify_lambda_http;
