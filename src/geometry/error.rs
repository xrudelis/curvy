use std::backtrace::Backtrace;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}\n{backtrace}")]
pub struct CurvyError {
    pub message: String,
    pub backtrace: Backtrace,
}

macro_rules! curvy_err {
    ($msg:expr) => {
        Err(CurvyError {
            message: ($msg).to_string(),
            backtrace: Backtrace::capture(),
        })
    };
}

pub type CurvyResult<T> = Result<T, CurvyError>;
