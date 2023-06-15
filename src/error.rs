use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Contract not mutable")]
    ContractNotMutable {},

    #[error("Percentage limit exceeded")]
    PercentageLimitExceeded {},

    #[error("Percentage limit not met")]
    PercentageLimitNotMet {},
}
