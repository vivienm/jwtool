use derive_more::{Display, Error, From};

#[derive(Debug, Display, From, Error)]
pub enum Error {
    IO(std::io::Error),
    JSON(serde_json::error::Error),
    JWT(jsonwebtoken::errors::Error),
}

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;
