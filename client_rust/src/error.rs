use std::fmt::{Debug, Formatter, Write, Display};
use std::error::Error;

pub type AirdropPoolClientResult<T> = Result<T, AirdropPoolClientError>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum AirdropPoolClientError {
    InsufficientBalanceForFees {
        balance: u64,
        required: u64,
    },
    RpcClientError,
    ReferrerDidNotClaim,
}

impl Error for AirdropPoolClientError {

}

impl Display for AirdropPoolClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Debug::fmt(self, f) }
}

// impl Debug for AirdropPoolClientError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AirdropPoolClientError::InsufficientBalanceForFees { balance, required } {
//                 f.write_str(&format!("InsufficientBalanceForFees(balance={}, required={})", balance, required))?;
//             },
//         }
//         Ok(())
//     }
// }