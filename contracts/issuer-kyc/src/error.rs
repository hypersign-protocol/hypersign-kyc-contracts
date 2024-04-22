use cosmwasm_std::StdError;
use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum KycContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Invalid token_id - {token_id} already claimed")]
    InvalidTokenId { token_id: u64 },

    #[error("Owner DID is required")]
    OnwerDIDRequired {},

    #[error("Code_Id is required")]
    CodeIdRequired {},
}
