use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("build error: {0}")]
    Build(#[from] pyohwa_core::BuildError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("server error: {0}")]
    Server(String),

    #[error("watcher error: {0}")]
    Watcher(String),
}
