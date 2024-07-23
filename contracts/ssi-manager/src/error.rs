use cosmwasm_std::StdError;
use thiserror::Error;
use url::ParseError;

use crate::msg::SignatureTypes;
#[derive(Error, Debug, PartialEq)]
pub enum KycContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // FromResidual<Result<Infallible, serde_json::Error>>
    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Invalid token_id - {token_id} already claimed")]
    InvalidTokenId { token_id: u64 },

    #[error("Owner DID is required")]
    OnwerDIDRequired {},

    #[error("Code_Id is required")]
    CodeIdRequired {},

    // DID
    #[error("DID Method is required")]
    EmptyDIDMethod {},

    #[error("Invalid DID id - {did} , or did method not supported")]
    InvalidDIDId { did: String },

    #[error("DID - {did} is already registred")]
    DIDAlreadyRegistred { did: String },

    #[error("DID is required")]
    EmptyDID {},

    #[error("DID Method - {did_method} not supported")]
    UnSupportedDIDMethod { did_method: String },

    #[error("Could not verify the proof")]
    SignatureMissmatch {},

    // #[error("Could not verify the proof")]
    // SignatureMissmatch {},
    #[error("Could not transfrom proof of type ")]
    ProofTransformationError { signature_type: SignatureTypes },

    #[error("RDF graph error: {0}")]
    RdfGraphError(#[from] ParseError),

    #[error("Fragment not found")]
    FragmentNotFound,
}
