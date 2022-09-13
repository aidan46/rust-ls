use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Path \"{0}\" not found")]
    PathNotFound(String),
}
