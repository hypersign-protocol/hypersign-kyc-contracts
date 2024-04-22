use thiserror::Error;
use cosmwasm_std::StdError;
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized {
        owner: String
    },

    #[error("Invalid token_id - {token_id} already claimed")]
    InvalidTokenId {
        token_id: u64
    },
}