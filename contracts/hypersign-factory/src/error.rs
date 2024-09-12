use cosmwasm_std::StdError;
use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum FactoryContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Invalid token_id - {token_id} already claimed")]
    InvalidTokenId { token_id: u64 },

    #[error("Issuer DID - {issuer_did} is not registered")]
    InvalidIssuerDID { issuer_did: String },

    #[error("Issuer DID - {issuer_did} is already registered")]
    IssuerDIDAlreadyRegistred { issuer_did: String },

    #[error("Could not verify the proof")]
    SignatureMissmatch {},

    #[error("Unexpected failure")]
    UnexpectedFailure {},

    #[error("Owner DID is required")]
    OnwerDIDRequired {},
}
