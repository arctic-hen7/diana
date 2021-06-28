// This example illustrates how to generate a JWT
// This method is the same to create a JWT to connect to the subscriptions server as it is to generate one for a user

use diana::{create_jwt, get_jwt_secret, decode_time_str};
use std::env;
use std::collections::HashMap;

fn main() {
    dotenv::from_filename("examples/.env").expect("Failed to load environment variables!");
    let secret = get_jwt_secret(
        env::var("JWT_SECRET").unwrap()
    ).expect("Couldn't parse JWT secret!");

    let mut claims: HashMap<String, String> = HashMap::new();
    claims.insert("role".to_string(), "graphql_server".to_string()); // Role must be 'graphql_server' for connecting to the subscriptions server
    let jwt = create_jwt(
        claims,
        &secret,
        decode_time_str("1w").unwrap() // This token will be valid for one week
    ).expect("Couldn't create JWT!");

    println!("{}", jwt);
}
