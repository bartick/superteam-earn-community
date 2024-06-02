use dotenv;

/**
 * This function is used to load the environment variables from the .env file
 */
pub fn load_env() {
    dotenv::from_filename(".env").expect("Failed to load .env file");
}