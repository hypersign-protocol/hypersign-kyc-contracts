use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Empty, StdResult, WasmMsg};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::ToString;

#[cw_serde]
pub struct InstantiateMsg {
    // pub token_code_id: u64,
    pub did_doc: String,
    pub did_doc_proof: String,
    pub signature: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    OwnerDID {},

    #[returns(SBTcontractAddressResp)]
    SBTContractAddress {},
}

#[cw_serde]
#[derive(ToString)]
pub enum HypersignKYCProofTypes {
    // supported
    #[strum(serialize = "ProofOfPersonhood")]
    ProofOfPersonhood,

    #[strum(serialize = "ProofOfKYC")]
    ProofOfKYC,
    // not-supported-yet
    #[strum(serialize = "ProofOfCitizenship")]
    ProofOfCitizenship,

    #[strum(serialize = "ProofOfDateofBirth")]
    ProofOfDateofBirth,

    #[strum(serialize = "ProofOfAge")]
    ProofOfAge,

    #[strum(serialize = "ProofOfNonMembershipCountry")]
    ProofOfNonMembershipCountry,

    #[strum(serialize = "ProofOfOnchainAML")]
    ProofOfOnchainAML,

    #[strum(serialize = "ProofOfTransaction")]
    ProofOfTransaction,

    #[strum(serialize = "ProofOfUSAccrediatedInvestor")]
    ProofOfUSAccrediatedInvestor,

    #[strum(serialize = "ProofOfNonPEP")]
    ProofOfNonPEP,
}

// fn animal_to_string(animal: &Animal) -> &str {
//     match animal {
//         Animal::Dog => "Dog",
//         Animal::Cat => "Cat",
//         Animal::Bird => "Bird",
//     }
// }

#[cw_serde]
pub struct HypersignKYCProof {
    pub proof_type: HypersignKYCProofTypes, // Proof Of Personhood
    pub description: String,                // Proves that user is not a bot
    pub sbt_code: String,                   // T1
    pub credential_id: Option<String>,      // verifiable credential id linked with this proof
    pub data: Option<String>,               // an optional field which may contain any data like zkp
    pub proof_type_image: Option<String>,   // optional field which store image
}

#[cw_serde]
pub enum ExecMsg {
    Mint { hypersign_proof: HypersignKYCProof },
    Init { token_code_id: u64 },
}

#[cw_serde]
pub struct ValueResp {
    pub owner_did: String,
}

#[cw_serde]
pub struct SBTcontractAddressResp {
    pub sbt_contract_address: String,
}

// use std::string::ToString;
// pub impl ToString for HypersignKYCProofTypes {
//     fn to_string(&self) -> String {
//         match self {
//             HypersignKYCProofTypes::ProofOfPersonhood => "ProofOfPersonhood".to_string(),
//             HypersignKYCProofTypes::ProofOfKYC => "ProofOfKYC".to_string(),
//             HypersignKYCProofTypes::ProofOfCitizenship => "ProofOfCitizenship".to_string(),
//             HypersignKYCProofTypes::ProofOfDateofBirth => "ProofOfDateofBirth".to_string(),
//             HypersignKYCProofTypes::ProofOfAge => "ProofOfAge".to_string(),
//             HypersignKYCProofTypes::ProofOfNonMembershipCountry => {
//                 "ProofOfNonMembershipCountry".to_string()
//             }
//             HypersignKYCProofTypes::ProofOfOnchainAML => "ProofOfOnchainAML".to_string(),
//             HypersignKYCProofTypes::ProofOfTransaction => "ProofOfTransaction".to_string(),
//             HypersignKYCProofTypes::ProofOfUSAccrediatedInvestor => {
//                 "ProofOfUSAccrediatedInvestor".to_string()
//             }
//             HypersignKYCProofTypes::ProofOfNonPEP => "ProofOfNonPEP".to_string(),
//         }
//     }
// }

pub type CW721OnChainMetadataInstantiateMsg = cw721_metadata_onchain::InstantiateMsg;
