use diana::{create_jwt, decode_time_str, get_jwt_secret, validate_and_decode_jwt};
use std::collections::HashMap;

const JWT_SECRET: &str = "thisisaterriblesecretthatshouldberandomlygeneratedseethebook";

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
// TODO: Tests for `decode_time_str`
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
