// This file contains the end-to-end tests for the AWS Lambda integration
// These assume that the latest version of the function in `examples/netlify` has been built and deployed to the correct location
// These tests should be run locally, but MUST NOT be run on CI

use diana::{create_jwt, decode_time_str, get_jwt_secret};
use reqwest::Client;
use std::collections::HashMap;
use std::env;

fn get_valid_auth_header() -> Option<String> {
    dotenv::from_filename("../../../examples/.env").unwrap(); // For the JWT secret
                                                              // We assume `get_opts` has been called, which will load the environment variable if necessary
    let secret = get_jwt_secret(env::var("JWT_SECRET").unwrap()).unwrap();
    let mut claims = HashMap::new();
    claims.insert("role".to_string(), "test".to_string());
    let exp = decode_time_str("1m").unwrap(); // The created JWT will be valid for 1 minute
    let jwt = create_jwt(claims, &secret, exp).unwrap();
    Some("Bearer ".to_string() + &jwt)
}

// A utility testing macro that handles boilerplate for tests requests
macro_rules! req_test(
    ($client:expr, $req_body:expr, $expected_res:expr) => {
        {
            let res = $client
                .post(&env::var("NETLIFY_DEPLOYMENT").unwrap())
                .body($req_body)
                .header("Authorization", &get_valid_auth_header().unwrap()) // We don't offer an option to change the token validity because the auth system is integration tested for DianaHandler
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            if res != $expected_res {
                panic!("Invalid response to test request on serverless queries/mutations system. Found '{}'", res)
            }
        }
     };
);

#[tokio::test]
#[ignore] // This test connects to an active deployment on Netlify (not ideal for CI!)
async fn e2e() {
    dotenv::from_filename("./tests/.env.local").unwrap(); // For the Netlify deployment URL (excluded from Git)
    let client = Client::new();
    // We only run a single simple query test on the serverless system because otherwise we'd need to set up a subscriptions server etc.
    req_test!(
        client,
        "{\"query\": \"query { apiVersion }\"}",
        "{\"data\":{\"apiVersion\":\"0.1.0\"}}"
    );
}
