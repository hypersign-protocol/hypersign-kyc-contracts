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

    #[error("Could not verify the proof")]
    SignatureMissmatch {},

    #[error("Unexpected failure")]
    UnexpectedFailure {},

    #[error("Zk Proof Verification Failed")]
    ZkProofVerificationFailure {},

    // #[error("{0}")]
    // ZkProofError(#[from] String),
    #[error("Zk Proof Failed")]
    ZkProofFailure { err: String },

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("This signature was already verified")]
    ChallengeInvalid {},
}

impl From<semver::Error> for KycContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
