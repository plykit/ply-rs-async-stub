use thiserror::Error;
use anyhow;

#[derive(Error, Debug)]
pub enum PlyError {
    #[error("operation `{0}` is not known, valid operations: 'create', 'update', 'delete'")]
    UnknownOperation(String),
    #[error(transparent)]
    SendError(#[from] anyhow::Error),

    #[error("todo error")]
    Todo(String),

    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
}
