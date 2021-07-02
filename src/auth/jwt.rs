use chrono::{prelude::Utc, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::*;

// We make the claims very generic
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Claims {
    pub exp: u64,
    pub claims: HashMap<String, String>, // The additional claims the user makes (as generic as possible)
}

#[derive(Debug)]
pub struct JWTSecret<'a> {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'a>,
}

/// Transforms a string JWT secret into a form in which it can be used for operations.
pub fn get_jwt_secret<'a>(secret_str: String) -> Result<JWTSecret<'a>> {
    let encoding_key = EncodingKey::from_base64_secret(&secret_str)?;
    let decoding_key = DecodingKey::from_base64_secret(&secret_str)?;

    Ok(JWTSecret {
        encoding_key,
        decoding_key,
    })
}

/// Decodes time strings like '1w' into actual datetimes from the present moment. If you've ever used NodeJS's [`jsonwebtoken`](https://www.npmjs.com/package/jsonwebtoken) module, this is
/// very similar (based on Vercel's [`ms`](https://github.com/vercel/ms) module for JavaScript).
/// Accepts strings of the form 'xXyYzZ...', where the lower-case letters are numbers meaning a number of the intervals X/Y/Z (e.g. 1m4d -- one month four days).
/// The available intervals are:
///
/// - s: second,
/// - m: minute,
/// - h: hour,
/// - d: day,
/// - w: week,
/// - M: month (30 days used here, 12M ≠ 1y!),
/// - y: year (365 days always, leap years ignored, if you want them add them as days)
pub fn decode_time_str(time_str: &str) -> Result<u64> {
    let mut duration_after_current = Duration::zero();
    // Get the current datetime since Unix epoch, we'll add to that
    let current = Utc::now();
    // A working variable to store the '123' part of an interval until we reach the idnicator and can do the full conversion
    let mut curr_duration_length = String::new();
    // Iterate through the time string's characters to get each interval
    for c in time_str.chars() {
        // If we have a number, append it to the working cache
        // If we have an indicator character, we'll match it to a duration
        if c.is_numeric() {
            curr_duration_length.push(c);
        } else {
            // Parse the working variable into an actual number
            let interval_length = curr_duration_length.parse::<i64>().unwrap(); // It's just a string of numbers, we know more than the compiler
            let duration = match c {
                's' => Duration::seconds(interval_length),
                'm' => Duration::minutes(interval_length),
                'h' => Duration::hours(interval_length),
                'd' => Duration::days(interval_length),
                'w' => Duration::weeks(interval_length),
                'M' => Duration::days(interval_length * 30), // Multiplying the number of months by 30 days (assumed length of a month)
                'y' => Duration::days(interval_length * 365), // Multiplying the number of years by 365 days (assumed length of a year)
                c => bail!(ErrorKind::InvalidDatetimeIntervalIndicator(c.to_string())),
            };
            duration_after_current = duration_after_current + duration;
            // Reset that working variable
            curr_duration_length = String::new();
        }
    }
    // Form the final duration by reducing the durations vector into one
    let datetime = current + duration_after_current;

    Ok(datetime.timestamp() as u64) // As Unix timestamp in u64 because that's what the JWT demands (we can't have expiries before January 1st 1970, let me know if that's a problem!)
}

/// Creates a new JWT. You should use this to issue all client JWTs and create the initial JWT for communication with the subscriptions
/// server (more information in the book).
pub fn create_jwt(
    user_claims: HashMap<String, String>,
    secret: &JWTSecret,
    exp: u64,
) -> Result<String> {
    // Create the claims
    let claims = Claims {
        exp,
        claims: user_claims,
    };
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &secret.encoding_key,
    )?;

    Ok(token)
}

/// Validates a JWT and returns the payload. All client JWTs are automatically validated and their payloads are sent (parsed) to your resolvers,
/// but if you have a system on top of that you'll want to use this function (not required for normal Diana usage though).
pub fn validate_and_decode_jwt(jwt: &str, secret: &JWTSecret) -> Option<Claims> {
    let decoded = decode::<Claims>(
        jwt,
        &secret.decoding_key,
        &Validation::new(Algorithm::HS512),
    );

    match decoded {
        Ok(decoded) => Some(decoded.claims),
        Err(_) => None,
    }
}
