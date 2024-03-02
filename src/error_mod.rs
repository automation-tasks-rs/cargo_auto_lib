// error_mod.rs

//! Error library for this crate using thiserror
//!
//! I am using the crate thiserror to create an enum for all library errors.
//! It mostly forwards the source "from" error.
//! The library never writes to the screen, because it contains only the logic.
//! Is the bin project that knows if it is CLI, TUI or GUI and it presents the errors to the user and developer.
//! Then in the bin project I use the crate anyhow.

/// list of possible errors from this library
#[derive(thiserror::Error, Debug)]
pub enum LibError {
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("InfallibleError: {0}")]
    InfallibleError(#[from] std::convert::Infallible),

    #[error("StdIoError: {0}")]
    StdIoError(#[from] std::io::Error),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    //#[error("ErrorFromString: {0}")]
    //ErrorFromString(String),
    #[error("ErrorFromStaticStr: {0}")]
    ErrorFromStr(&'static str),
    //#[error("unknown error")]
    //UnknownError,
}

/// Result<> type with fixed LibError using thiserror
pub type ResultWithLibError<T, E = LibError> = core::result::Result<T, E>;
