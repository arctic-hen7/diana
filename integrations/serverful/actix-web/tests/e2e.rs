// This file contains full end-to-end tests of Diana for Actix Web
// This is the verbatim process of manual testing that would be gone through to tests these systems
// This testing is brittle only to the schema, and tests the full E2E result of the system, removing the need to test many smaller components

use diana::{create_jwt, decode_time_str, get_jwt_secret};
use diana_actix_web::{
    actix_web::{App, HttpServer},
    create_graphql_server, create_subscriptions_server,
};
use reqwest::Client;
use std::collections::HashMap;
use tungstenite::{connect, Message};

// This 'dirty-imports' the code in `schema.in`
// It will literally be interpolated here
// Never use this in production unless you have a fantastic reason! Just import your code through Cargo!
// We do this here though because you can't import from another example (which is annoying)
include!("../../../../examples/schema/schema.rs");

fn get_valid_auth_header() -> Option<String> {
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
                .post("http://localhost:9001/graphql")
                .body($req_body)
                .header("Authorization", &get_valid_auth_header().unwrap()) // We don't offer an option to change the token validity because the auth system is integration tested for DianaHandler
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            if res != $expected_res {
                panic!("Invalid response to test request on queries/mutations system. Found '{}'", res)
            }
        }
     };
);
// A testing utility macro for expecting a certain GraphQL subscription response
// This doesn't actuall send anything to the endpoint, it just checks on the given socket
macro_rules! expect_subscription_res(
    ($socket:expr, $expected_res:expr) => {
        {
            let msg = $socket.read_message().unwrap();
            // The expected response will have the following WS metadata around it
            // We don't have braces around the payload because the user should include those (see usage)
            let expectation_in_ctx = Message::Text("{\"type\":\"data\",\"id\":\"1\",\"payload\":".to_string() + $expected_res + "}");
            if msg != expectation_in_ctx {
                panic!("Invalid subscription response. Expected '{}', found '{}'", expectation_in_ctx, msg);
            }
        }
     };
);
// A testing utility macro for confirming that the first confirmational response for a GraphQL subscription happens
macro_rules! expect_subscription_to_connect(
    ($socket:expr) => {
        {
            let msg = $socket.read_message().unwrap();
            let expectation = Message::Text("{\"type\":\"connection_ack\"}".to_string());
            if msg != expectation {
                panic!("Expected initial connection acknowledgement message, found '{}'", msg);
            }
        }
     };
);
// A testing utility macro to close the given WS connection cleanly and shut down Actix Web servers
macro_rules! shutdown_all(
    ($socket:expr, $graphql_server:expr, $subscriptions_server:expr) => {
        {
            $socket.close(None).unwrap();
            loop {
                let msg = $socket.read_message();
                if msg.is_err() {
                    break;
                }
            }

            // Stop the Actix Web servers gracefully
            $graphql_server.stop(true).await;
            $subscriptions_server.stop(true).await;
        }
     };
);

#[actix_web::rt::test]
async fn e2e() {
    // Get the configurations for the two servers
    let graphql_configurer =
        create_graphql_server(get_opts()).expect("Failed to set up queries/mutations configurer!");
    let susbcriptions_configurer = create_subscriptions_server(get_opts())
        .expect("Failed to set up subscriptions configurer!");

    // Initialise both of them
    let graphql_server = HttpServer::new(move || App::new().configure(graphql_configurer.clone()))
        .bind("0.0.0.0:9001")
        .expect("Couldn't bind to port 9001 in test.")
        .run();
    let subscriptions_server =
        HttpServer::new(move || App::new().configure(susbcriptions_configurer.clone()))
            .bind("0.0.0.0:9002")
            .expect("Couldn't bind to port 9002 in test.")
            .run();

    // Testing code from here on
    // We establish a connection to the subscriptions server first so we can listen to the messages it receives after mutations
    let (mut socket, _) = connect("ws://localhost:9002/graphql").unwrap();
    socket
        .write_message(Message::Text(
            "{\"type\": \"connection_init\", \"payload\": {}}".to_string(),
        ))
        .unwrap();
    socket.write_message(Message::Text("{\"id\": \"1\", \"type\": \"start\", \"payload\": {\"query\": \"subscription { newBlahs { username } }\"}}".to_string())).unwrap();
    expect_subscription_to_connect!(socket);

    let client = Client::new();
    req_test!(
        client,
        "{\"query\": \"query { apiVersion }\"}",
        "{\"data\":{\"apiVersion\":\"0.1.0\"}}"
    );
    req_test!(
        client,
        "{\"query\": \"mutation { updateBlah }\"}",
        "{\"data\":{\"updateBlah\":true}}"
    ); // This will also test communication with the subscriptions server (it would return an error on failure)
    expect_subscription_res!(
        socket,
        "{\"data\":{\"newBlahs\":{\"username\":\"This is a username\"}}}"
    );

    // Close the WS connection
    // We need to continue reading messages until the server confirms because that's how WS handshakes work
    shutdown_all!(socket, graphql_server, subscriptions_server);

    println!("Connection closed successfully! All tests have passed!");

    // The below two lines will keep this program open for further manual testing
    // This should only be uncommented in development, and must not be active on CI!
    // let never = futures::future::pending::<()>();
    // never.await;
}
