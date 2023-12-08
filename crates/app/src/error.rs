use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("generic")]
    Generic(String),
}

