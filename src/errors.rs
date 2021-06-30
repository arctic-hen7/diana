#![allow(missing_docs)]

pub use error_chain::bail;
use error_chain::error_chain;

// The `error_chain` setup for the whole crate
// All systems use these errors, except for GraphQL resolvers, because they have to return a particular kind of error
error_chain! {
    // The custom errors for this crate (very broad)
    errors {
        /// An environment variable had an invalid type.
        /// E.g. a port was given as a hex string for some reason.
        InvalidEnvVarType(var_name: String, expected: String) {
            description("invalid environment variable type")
            display(
                "invalid environment variable type for variable '{var_name}', expected '{expected}'",
                var_name=var_name,
                expected=expected
            )
        }

        /// A required part of the GraphQL context was not found.
        GraphQLContextNotFound(elem_name: String) {
            description("required graphql context element not found")
            display("required graphql context element '{}' not found", elem_name)
        }

        /// A Mutex was poisoned (if `.lock()` failed).
        MutexPoisoned(mutex_name: String) {
            description("mutex poisoned")
            display("mutex '{}' poisoned", mutex_name)
        }

        /// The subscriptions server failed to publish data it was asked to. This error is usually caused by an authentication failure.
        SubscriptionDataPublishFailed {
            description("failed to publish data to the subscriptions server")
            display("failed to publish data to the subscriptions server, this is most likely due to an authentication failure")
        }

        /// An invalid indicator string was used when trying to convert a timestring into a datetime.
        InvalidDatetimeIntervalIndicator(indicator: String) {
            description("invalid indicator in timestring")
            display("invalid indicator '{}' in timestring, must be one of: s, m, h, d, w, M, y", indicator)
        }

        /// There was an unauthorised access attempt.
        Unauthorised {
            description("unauthorised access attempt")
            display("unable to comply with request due to lack of valid and sufficient authentication")
        }

        /// One or more required builder fields weren't set up.
        IncompleteBuilderFields {
            description("not all required builder fields were instantiated")
            display("some required builder fields haven't been instantiated")
        }

        /// The creation of an HTTP response for Lambda or its derivatives failed.
        HttpResponseBuilderFailed {
            description("the builder for an http response (netlify_lambda_http) returned an error")
            display("the builder for an http response (netlify_lambda_http) returned an error")
        }

        /// There was an attempt to create a subscriptions server without declaring its existence or configuration in the [Options].
        InvokedSubscriptionsServerWithInvalidOptions {
            description("you tried to create a subscriptions server without configuring it in the options")
            display("you tried to create a subscriptions server without configuring it in the options")
        }

        /// There was an attempt to initialize the GraphiQL playground in a production environment
        AttemptedPlaygroundInProduction {
            description("you tried to initialize the GraphQL playground in production, which is not supported due to authentication issues")
            display("you tried to initialize the GraphQL playground in production, which is not supported due to authentication issues")
        }

        IntegrationError(message: String, integration_name: String) {
            description("an error occurred in one of Diana's integration libraries")
            display(
                "the following error occurred in the '{integration_name}' integration library: {message}",
                integration_name=integration_name,
                message=message
            )
        }
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        EnvVar(::std::env::VarError);
        Reqwest(::reqwest::Error);
        Json(::serde_json::Error);
        JsonWebToken(::jsonwebtoken::errors::Error);
    }
}

/// A wrapper around [async_graphql::Result<T>](async_graphql::Result).
/// You should use this as the return type for any of your own schemas that might return errors.
/// # Example
/// ```rust
/// use diana::errors::GQLResult;
///
/// async fn api_version() -> GQLResult<String> {
///     // Your code here
///     Ok("test".to_string())
/// }
/// ```
pub type GQLResult<T> = async_graphql::Result<T>;
/// A wrapper around [async_graphql::Error].
/// If any of your schemas need to explicitly create an error that only exists in them (and you're not using something like [mod@error_chain]),
/// you should use this.
/// # Example
/// ```rust
/// use diana::errors::{GQLResult, GQLError};
///
/// async fn api_version() -> GQLResult<String> {
///     let err = GQLError::new("Test error!");
///     // Your code here
///     Err(err)
/// }
/// ```
pub type GQLError = async_graphql::Error;
