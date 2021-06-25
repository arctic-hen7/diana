use std::fs;

use crate::errors::*;

// Loads all necessary environment files with `dotenv`
pub fn load_env() -> Result<()> {
    // Load the environment-specific environment variable files (checking whether we're in development/debug or production/release)
    if cfg!(debug_assertions) {
        load_env_file_if_present("../.env.development")?; // Non-secret
        load_env_file_if_present("../.env.development.local")?; // Secret
    } else {
        load_env_file_if_present("../.env.production")?; // Non-secret
        load_env_file_if_present("../.env.production.local")?; // Secret
    }
    // Load the files for all environments
    load_env_file_if_present("../.env")?; // Non-secret
    load_env_file_if_present("../.env.local")?; // Secret

    Ok(())
}

// Loads the given environment file if it's present, otherwise does nothing (side-effect based function)
fn load_env_file_if_present(filename: &str) -> Result<()> {
    // If the file exists, load its variables
    if fs::metadata(filename).is_ok() {
        dotenv::from_filename(filename)?;
    }
    // If it doesn't exist, we don't worry about it, that's the point of this function
    Ok(())
}
