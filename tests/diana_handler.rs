use async_graphql::{EmptyMutation, EmptySubscription, Object as GQLObject};
use diana::{
    create_jwt, decode_time_str, get_jwt_secret, AuthBlockLevel, AuthVerdict, DianaHandler,
    DianaResponse, Options, SysSchema,
};
use std::collections::HashMap;

#[derive(Clone)]
struct Context {
    prop: String,
}

#[derive(Clone)]
struct Query {}
#[GQLObject]
impl Query {
    async fn query(&self) -> bool {
        true
    }
}

const JWT_SECRET: &str = "thisisaterriblesecretthatshouldberandomlygeneratedseethebook";
const SIMPLE_QUERY: &str = "{\"query\": \"query { query }\"}";
const SIMPLE_QUERY_RES: &str = "{\"data\":{\"query\":true}}";
const SIMPLE_INVALID_QUERY: &str = "{\"query\": \"query { thisisnotaquery }\"}";
const SIMPLE_INVALID_QUERY_RES: &str = "{\"data\":null,\"errors\":[{\"message\":\"Unknown field \\\"thisisnotaquery\\\" on type \\\"Query\\\".\",\"locations\":[{\"line\":1,\"column\":9}]}]}";

fn get_opts(
    auth_block_level: AuthBlockLevel,
) -> Options<Context, Query, EmptyMutation, EmptySubscription> {
    Options::builder()
        .ctx(Context {
            prop: "connection".to_string(),
        })
        .subscriptions_server_hostname("http://localhost")
        .subscriptions_server_port("9002")
        .subscriptions_server_endpoint("/graphql")
        .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
        .auth_block_state(auth_block_level)
        .jwt_secret(JWT_SECRET)
        .schema(Query {}, EmptyMutation {}, EmptySubscription {})
        .graphql_endpoint("/graphql")
        .playground_endpoint("/graphiql")
        .finish()
        .unwrap()
}

fn get_valid_auth_header() -> Option<String> {
    let secret = get_jwt_secret(JWT_SECRET.to_string()).unwrap();
    let mut claims = HashMap::new();
    claims.insert("role".to_string(), "test".to_string());
    let exp = decode_time_str("1m").unwrap(); // The created JWT will be valid for 1 minute
    let jwt = create_jwt(claims, &secret, exp).unwrap();
    Some("Bearer ".to_string() + &jwt)
}

fn get_invalid_auth_header<'a>() -> Option<&'a str> {
    Some("Bearer thisisaninvalidjwt")
}

// Tests for `.new()`
#[test]
fn returns_valid_handler() {
    if !matches!(
        DianaHandler::new(get_opts(AuthBlockLevel::AllowAll)),
        Ok(DianaHandler { .. })
    ) {
        panic!("Didn't return valid DianaHandler instance.")
    }
    if !matches!(
        DianaHandler::new(get_opts(AuthBlockLevel::AllowMissing)),
        Ok(DianaHandler { .. })
    ) {
        panic!("Didn't return valid DianaHandler instance.")
    }
    if !matches!(
        DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)),
        Ok(DianaHandler { .. })
    ) {
        panic!("Didn't return valid DianaHandler instance.")
    }
}
#[test]
fn returns_valid_handler_with_no_subscriptions() {
    let opts = Options::builder()
        .ctx(Context {
            prop: "connection".to_string(),
        })
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret(JWT_SECRET)
        .schema(Query {}, EmptyMutation {}, EmptySubscription {})
        .graphql_endpoint("/graphql")
        .playground_endpoint("/graphiql")
        .finish()
        .unwrap();
    let diana_handler = DianaHandler::new(opts);
    if !matches!(diana_handler, Ok(DianaHandler { .. })) {
        panic!("Didn't return valid DianaHandler instance.")
    }
}
// Tests for `.is_authed()`
#[test]
fn allows_user_if_token_valid_for_block_unauthenticated_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let verdict = diana_handler.is_authed(get_valid_auth_header());
    if !matches!(verdict, AuthVerdict::Allow(_)) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Allow, got {:?}",
            verdict
        )
    }
}
#[test]
fn blocks_user_if_token_invalid_for_block_unauthenticated_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let verdict = diana_handler.is_authed(get_invalid_auth_header());
    if !matches!(verdict, AuthVerdict::Block) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Block, got {:?}",
            verdict
        )
    }
}
#[test]
fn blocks_user_if_token_missing_for_block_unauthenticated_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let verdict = diana_handler.is_authed(Option::<String>::None);
    if !matches!(verdict, AuthVerdict::Block) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Block, got {:?}",
            verdict
        )
    }
}
#[test]
fn allows_user_if_token_valid_for_allow_all_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::AllowAll)).unwrap();
    let verdict = diana_handler.is_authed(get_valid_auth_header());
    if !matches!(verdict, AuthVerdict::Allow(_)) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Allow, got {:?}",
            verdict
        )
    }
}
#[test]
fn allow_user_if_token_invalid_for_allow_all_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::AllowAll)).unwrap();
    let verdict = diana_handler.is_authed(get_invalid_auth_header());
    if !matches!(verdict, AuthVerdict::Allow(_)) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Allow, got {:?}",
            verdict
        )
    }
}
#[test]
fn allow_user_if_token_missing_for_allow_all_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::AllowAll)).unwrap();
    let verdict = diana_handler.is_authed(Option::<String>::None);
    if !matches!(verdict, AuthVerdict::Allow(_)) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Allow, got {:?}",
            verdict
        )
    }
}
#[test]
fn allows_user_if_token_valid_for_allow_missing_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::AllowMissing)).unwrap();
    let verdict = diana_handler.is_authed(get_valid_auth_header());
    if !matches!(verdict, AuthVerdict::Allow(_)) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Allow, got {:?}",
            verdict
        )
    }
}
#[test]
fn blocks_user_if_token_invalid_for_allow_missing_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::AllowMissing)).unwrap();
    let verdict = diana_handler.is_authed(get_invalid_auth_header());
    if !matches!(verdict, AuthVerdict::Block) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Block, got {:?}",
            verdict
        )
    }
}
#[test]
fn allows_user_if_token_missing_for_allow_missing_block_state() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::AllowMissing)).unwrap();
    let verdict = diana_handler.is_authed(Option::<String>::None);
    if !matches!(verdict, AuthVerdict::Allow(_)) {
        panic!(
            "Didn't return correct AuthVerdict response. Expected AuthVerdict::Allow, got {:?}",
            verdict
        )
    }
}
// Tests for `.run_stateless_req()` (internal function that underlies other simpler querying logic)
#[tokio::test]
async fn returns_success_on_valid_auth_and_body() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let res = diana_handler
        .run_stateless_req(
            SysSchema::WithoutSubscriptions,
            SIMPLE_QUERY.to_string(),
            get_valid_auth_header(),
            None,
        )
        .await;
    if !matches!(res.clone(), DianaResponse::Success(val) if val == SIMPLE_QUERY_RES) {
        panic!("Didn't return correct DianaResponse variant. Expected DianaResponse::Success, got {:?}", res)
    }
}
#[tokio::test]
async fn returns_success_with_error_embedded_on_valid_auth_and_invalid_body() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let res = diana_handler
        .run_stateless_req(
            SysSchema::WithoutSubscriptions,
            SIMPLE_INVALID_QUERY.to_string(),
            get_valid_auth_header(),
            None,
        )
        .await;
    if !matches!(res.clone(), DianaResponse::Success(val) if val == SIMPLE_INVALID_QUERY_RES) {
        panic!("Didn't return correct DianaResponse variant. Expected DianaResponse::Success, got {:?}", res)
    }
}
#[tokio::test]
async fn returns_blocked_on_invalid_auth_and_valid_body() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let res = diana_handler
        .run_stateless_req(
            SysSchema::WithoutSubscriptions,
            SIMPLE_QUERY.to_string(),
            get_invalid_auth_header(),
            None,
        )
        .await;
    if !matches!(res.clone(), DianaResponse::Blocked) {
        panic!("Didn't return correct DianaResponse variant. Expected DianaResponse::Blocked, got {:?}", res)
    }
}
#[tokio::test]
async fn returns_blocked_on_invalid_auth_and_invalid_body() {
    let diana_handler = DianaHandler::new(get_opts(AuthBlockLevel::BlockUnauthenticated)).unwrap();
    let res = diana_handler
        .run_stateless_req(
            SysSchema::WithoutSubscriptions,
            SIMPLE_INVALID_QUERY.to_string(),
            get_invalid_auth_header(),
            None,
        )
        .await;
    if !matches!(res.clone(), DianaResponse::Blocked) {
        panic!("Didn't return correct DianaResponse variant. Expected DianaResponse::Blocked, got {:?}", res)
    }
}
