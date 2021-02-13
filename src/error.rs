use derive_more::{Display, Error, From};

#[derive(Debug, Display, From, Error)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::error::Error),
    Jwt(jsonwebtoken::errors::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
