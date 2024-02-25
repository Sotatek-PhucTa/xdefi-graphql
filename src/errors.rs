use thiserror::Error; // Import the Error trait for custom error handling

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid EVM address")]
    InvalidAddress,
    #[error("Invalid Env file")]
    InvalidEnv,
}