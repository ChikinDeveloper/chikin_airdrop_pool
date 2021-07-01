//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AirdropPoolError {
    #[error("ProgramKeyMismatch")]
    ProgramKeyMismatch,
    #[error("TokenProgramKeyMismatch")]
    TokenProgramKeyMismatch,

    #[error("PoolAccountKeyMismatch")]
    PoolAccountKeyMismatch,
    #[error("PoolTokenAccountKeyMismatch")]
    PoolTokenAccountKeyMismatch,

    #[error("UserAccountKeyMismatch")]
    UserAccountKeyMismatch,
    #[error("UserTokenAccountKeyMismatch")]
    UserTokenAccountKeyMismatch,

    #[error("ReferrerAccountKeyMismatch")]
    ReferrerAccountKeyMismatch,
    #[error("ReferrerTokenAccountKeyMismatch")]
    ReferrerTokenAccountKeyMismatch,

    #[error("InsufficientBalance")]
    InsufficientBalance,
    #[error("AlreadyClaimed")]
    AlreadyClaimed,

    #[error("TransferToUserFailed")]
    TransferToUserFailed,
    #[error("TransferToReferrerFailed")]
    TransferToReferrerFailed,
}

impl From<AirdropPoolError> for ProgramError {
    fn from(e: AirdropPoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for AirdropPoolError {
    fn type_of() -> &'static str { "AirdropPoolError" }
}