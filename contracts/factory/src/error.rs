use cosmwasm_std::{DivideByZeroError, Instantiate2AddressError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Instantiate2AddressError(#[from] Instantiate2AddressError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Contract not mutable")]
    ContractNotMutable {},

    #[error("Percentage limit exceeded")]
    PercentageLimitExceeded {},

    #[error("Percentage limit not met")]
    PercentageLimitNotMet {},

    #[error("Could not instantiate custom contract")]
    InstantiateError {},
}
