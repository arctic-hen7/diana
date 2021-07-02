use chrono::{Duration, Utc};
use diana::{create_jwt, decode_time_str, get_jwt_secret, validate_and_decode_jwt};
use std::collections::HashMap;

const JWT_SECRET: &str = "thisisaterriblesecretthatshouldberandomlygeneratedseethebook";
const NUM_INTERVAL_LENGTHS_TO_TEST: i64 = 1000;

// Tests for `get_jwt_secret`
#[test]
fn returns_secret_if_valid() {
    let secret_str = JWT_SECRET.to_string();
    let secret = get_jwt_secret(secret_str);
    if !matches!(secret, Ok(_)) {
        panic!("Expected Ok, found {:?}", secret);
    }
}
#[test]
fn returns_error_if_invalid() {
    let secret_str = "!@#$%^&*()".to_string(); // That's not valid base64...
    let secret = get_jwt_secret(secret_str);
    if !matches!(secret, Err(_)) {
        panic!("Expected Err, found {:?}", secret);
    }
}
// Tests for `decode_time_str`
// A testing utility macro to test all the intervals with `decode_time_str` for a given length
macro_rules! test_all_intervals_for_length(
    ($length:expr) => {
        {
            assert_eq!(
                decode_time_str(($length.to_string() + "s").as_str()).unwrap(),
                (Utc::now() + Duration::seconds($length)).timestamp() as u64
            );
            assert_eq!(
                decode_time_str(($length.to_string() + "m").as_str()).unwrap(),
                (Utc::now() + Duration::minutes($length)).timestamp() as u64
            );
            assert_eq!(
                decode_time_str(($length.to_string() + "h").as_str()).unwrap(),
                (Utc::now() + Duration::hours($length)).timestamp() as u64
            );
            assert_eq!(
                decode_time_str(($length.to_string() + "d").as_str()).unwrap(),
                (Utc::now() + Duration::days($length)).timestamp() as u64
            );
            assert_eq!(
                decode_time_str(($length.to_string() + "w").as_str()).unwrap(),
                (Utc::now() + Duration::weeks($length)).timestamp() as u64
            );
            assert_eq!(
                decode_time_str(($length.to_string() + "M").as_str()).unwrap(),
                (Utc::now() + Duration::days($length * 30)).timestamp() as u64
            );
            assert_eq!(
                decode_time_str(($length.to_string() + "y").as_str()).unwrap(),
                (Utc::now() + Duration::days($length * 365)).timestamp() as u64
            );
        }
     };
);
#[test]
fn returns_correct_datetime_for_each_interval_at_length_1() {
    // We test each interval directly with the above macro
    test_all_intervals_for_length!(1);
}
#[test]
fn returns_correct_datetime_for_each_interval_at_many_length() {
    // We test each interval directly with the above macro
    // This tests runs for many intervals
    for length in 1..NUM_INTERVAL_LENGTHS_TO_TEST {
        test_all_intervals_for_length!(length);
    }
}
#[test]
fn returns_error_on_invalid_interval() {
    let decoded = decode_time_str("1q");
    if decoded.is_ok() {
        panic!("Didn't panic on time string with invalid interval.");
    }
}
// Tests for `create_jwt`
#[test]
fn returns_valid_jwt() {
    let mut claims = HashMap::new();
    claims.insert("role".to_string(), "test".to_string());
    let secret = get_jwt_secret(JWT_SECRET.to_string()).unwrap();
    let exp = decode_time_str("1w").unwrap();
    let jwt = create_jwt(claims.clone(), &secret, exp);
    if !matches!(jwt, Ok(_)) {
        panic!("Expected Ok, found {:?}", jwt);
    }

    let extracted_claims = validate_and_decode_jwt(&jwt.unwrap(), &secret);
    assert_eq!(extracted_claims.unwrap().claims, claims);
}
// Tests for `validate_and_decode_jwt` (the basic one is done with `create_jwt`)
#[test]
fn returns_error_if_jwt_invalid() {
    let secret = get_jwt_secret(JWT_SECRET.to_string()).unwrap();

    let extracted_claims = validate_and_decode_jwt("thisisaninvalidjwt", &secret);
    if !matches!(extracted_claims, None) {
        panic!("Expected None, found {:?}", extracted_claims);
    }
}
