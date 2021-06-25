pub use error_chain::bail;
use error_chain::error_chain;

// All systems use these errors, except for GraphQL resolvers, because they have to return a particular kind of error
error_chain! {
    // The custom errors for this crate
    errors {
        // For when an environment variable has an invalid type
        // For example if a port is given as a hex string for some reason
        InvalidEnvVarType(var_name: String, expected: String) {
            description("invalid environment variable type")
            display(
                "invalid environment variable type for variable '{var_name}', expected '{expected}'",
                var_name=var_name,
                expected=expected
            )
        }

        // For if the required part of the GraphQL context object is not found
        GraphQLContextNotFound(elem_name: String) {
            description("required graphql context element not found")
            display("required graphql context element '{}' not found", elem_name)
        }

        // For when some Mutex is poisoned
        // This error is used if `.lock()` fails on a Mutex
        MutexPoisoned(mutex_name: String) {
            description("mutex poisoned")
            display("mutex '{}' poisoned", mutex_name)
        }

        OidSerializationFailed {
            description("failed to serialize string as object id")
            display("failed to serialize string as object id")
        }
    }
    // Link to the core library
    links {
        DianaError(::lib::errors::Error, ::lib::errors::ErrorKind);
    }
    // We work with many external libraries, all of which have their own errors
    foreign_links {
        Io(::std::io::Error);
        EnvFile(::dotenv::Error); // For `dev_utils` only
        EnvVar(::std::env::VarError);
        Mongo(::mongodb::error::Error); // For `dev_utils` only
        BsonOid(::mongodb::bson::oid::Error); // For `dev_utils` only
        Json(::serde_json::Error);
    }
}
