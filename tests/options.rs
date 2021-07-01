use diana::{Options, OptionsBuilder, AuthBlockLevel};
use async_graphql::{Object as GQLObject, EmptyMutation, EmptySubscription};

#[derive(Clone)]
struct Context {
    prop: String
}

#[derive(Clone)]
struct Query {}
#[GQLObject]
impl Query {
    async fn query(&self) -> bool { true }
}

#[test]
fn returns_valid_options() {
    let opts = OptionsBuilder::new()
        .ctx(Context {
            prop: "connection".to_string(),
        })
        .subscriptions_server_hostname("http://localhost")
        .subscriptions_server_port("9002")
        .subscriptions_server_endpoint("/graphql")
        .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret("JWT_SECRET")
        .schema(Query {}, EmptyMutation {}, EmptySubscription {})
        .graphql_endpoint("/graphql")
        .playground_endpoint("/graphiql")
        .finish();

    if !matches!(opts, Ok(Options { .. })) {
        panic!("Didn't return valid Options instance.")
    }
}
#[test]
fn returns_valid_options_without_subscriptions() {
    let opts = OptionsBuilder::new()
        .ctx(Context {
            prop: "connection".to_string(),
        })
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret("JWT_SECRET")
        .schema(Query {}, EmptyMutation {}, EmptySubscription {})
        .graphql_endpoint("/graphql")
        .playground_endpoint("/graphiql")
        .finish();

    if !matches!(opts, Ok(Options { .. })) {
        panic!("Didn't return valid Options instance.")
    }
}
#[test]
fn uses_default_graphql_endpoint() {
    let opts = OptionsBuilder::new()
        .ctx(Context {
            prop: "connection".to_string(),
        })
        .subscriptions_server_hostname("http://localhost")
        .subscriptions_server_port("9002")
        .subscriptions_server_endpoint("/graphql")
        .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret("JWT_SECRET")
        .schema(Query {}, EmptyMutation {}, EmptySubscription {})
        // .graphql_endpoint("/graphql")
        .playground_endpoint("/graphiql")
        .finish();

    if !matches!(opts, Ok(Options { .. })) {
        panic!("Didn't return valid Options instance.")
    }
    assert_eq!(opts.unwrap().graphql_endpoint, "/graphql".to_string())
}
#[test]
fn uses_default_playground_endpoint() {
    let opts = OptionsBuilder::new()
        .ctx(Context {
            prop: "connection".to_string(),
        })
        .subscriptions_server_hostname("http://localhost")
        .subscriptions_server_port("9002")
        .subscriptions_server_endpoint("/graphql")
        .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
        .auth_block_state(AuthBlockLevel::AllowAll)
        .jwt_secret("JWT_SECRET")
        .schema(Query {}, EmptyMutation {}, EmptySubscription {})
        .graphql_endpoint("/graphql")
        // .playground_endpoint("/graphiql")
        .finish();

    if !matches!(opts, Ok(Options { .. })) {
        panic!("Didn't return valid Options instance.")
    }
    assert_eq!(opts.unwrap().playground_endpoint, Some("/graphiql".to_string()))
}
#[test]
fn returns_error_on_playground_in_production() {

}
#[test]
fn returns_error_on_missing_subscriptions_server_fields() {
    if matches!(
        OptionsBuilder::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            // .subscriptions_server_hostname("http://localhost")
            .subscriptions_server_port("9002")
            .subscriptions_server_endpoint("/graphql")
            .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
            .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .graphql_endpoint("/graphql")
            .playground_endpoint("/graphiql")
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
    if matches!(
        OptionsBuilder::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            .subscriptions_server_hostname("http://localhost")
            // .subscriptions_server_port("9002")
            .subscriptions_server_endpoint("/graphql")
            .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
            .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .graphql_endpoint("/graphql")
            .playground_endpoint("/graphiql")
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
    if matches!(
        OptionsBuilder::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            .subscriptions_server_hostname("http://localhost")
            .subscriptions_server_port("9002")
            // .subscriptions_server_endpoint("/graphql")
            .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
            .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .graphql_endpoint("/graphql")
            .playground_endpoint("/graphiql")
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
    if matches!(
        OptionsBuilder::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            .subscriptions_server_hostname("http://localhost")
            .subscriptions_server_port("9002")
            .subscriptions_server_endpoint("/graphql")
            // .jwt_to_connect_to_subscriptions_server("SUBSCRIPTIONS_SERVER_PUBLISH_JWT")
            .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .graphql_endpoint("/graphql")
            .playground_endpoint("/graphiql")
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
}
#[test]
fn returns_error_on_missing_required_fields() {
    if matches!(
        OptionsBuilder::<Context, Query, EmptyMutation, EmptySubscription>::new()
            // .ctx(Context {
            //     prop: "connection".to_string(),
            // })
            .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
    if matches!(
        OptionsBuilder::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            // .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
    if matches!(
        OptionsBuilder::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            .auth_block_state(AuthBlockLevel::AllowAll)
            // .jwt_secret("JWT_SECRET")
            .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
    if matches!(
        OptionsBuilder::<Context, Query, EmptyMutation, EmptySubscription>::new()
            .ctx(Context {
                prop: "connection".to_string(),
            })
            .auth_block_state(AuthBlockLevel::AllowAll)
            .jwt_secret("JWT_SECRET")
            // .schema(Query {}, EmptyMutation {}, EmptySubscription {})
            .finish(),
        Ok(Options { .. })
    ) {
        panic!("Returned valid options instance, should've been invalid.")
    }
}
