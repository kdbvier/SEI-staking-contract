use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Staking Disabled")]
    Disabled {},

    #[error("Invalid NFT")]
    InvalidCw721 {},

    #[error("Invalid Staking Method")]
    InvalidStaking {},

    #[error("This NFT is in pending")]
    PendingUnStaking {},

    #[error("Invalid Migrating User")]
    CannotMigrate { previous_contract: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
