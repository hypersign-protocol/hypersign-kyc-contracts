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
    #[strum(serialize = "zkProofOfPersonhood")]
    zkProofOfPersonhood,

    #[strum(serialize = "zkProofOfKYC")]
    zkProofOfKYC,

    #[strum(serialize = "zkProofOfAge")]
    zkProofOfAge,
    // #[strum(serialize = "ProofOfNonMembershipCountry")]
    // ProofOfNonMembershipCountry,
}

impl HypersignKYCProofTypes {
    // Method to get color of the proof type
    pub fn get_color(&self) -> &'static str {
        match self {
            HypersignKYCProofTypes::zkProofOfAge => "#ff0000", // red
            HypersignKYCProofTypes::zkProofOfKYC => "#00ff00", // green
            HypersignKYCProofTypes::zkProofOfPersonhood => "#0000ff", // blue
        }
    }

    pub fn get_sbt_code(&self) -> &'static str {
        match self {
            HypersignKYCProofTypes::zkProofOfAge => "T1",
            HypersignKYCProofTypes::zkProofOfKYC => "T2",
            HypersignKYCProofTypes::zkProofOfPersonhood => "T3",
        }
    }

    pub fn get_decription(&self) -> &'static str {
        match self {
            HypersignKYCProofTypes::zkProofOfAge => "Proves user is above or below certain age",
            HypersignKYCProofTypes::zkProofOfKYC => "Proves that user has finished his/her KYC",
            HypersignKYCProofTypes::zkProofOfPersonhood => "Proves that user is not a bot",
        }
    }

    // TODO: need to add logo urls here.
    pub fn get_logo(&self) -> &'static str {
        match self {
            HypersignKYCProofTypes::zkProofOfAge => "",
            HypersignKYCProofTypes::zkProofOfKYC => "",
            HypersignKYCProofTypes::zkProofOfPersonhood => "",
        }
    }
}

#[cw_serde]
#[derive(ToString)]
pub enum HsZkProtocols {
    #[strum(serialize = "groth16")]
    groth16,
}

#[cw_serde]
#[derive(ToString)]
pub enum HsZkProtocolsCurvs {
    #[strum(serialize = "bn128")]
    bn128,
}

#[cw_serde]
pub struct HsZkProof {
    pub pi_a: Vec<u8>,
    pub pi_b: Vec<u8>,
    pub pi_c: Vec<u8>,
    pub protocol: HsZkProtocols,
    pub curve: HsZkProtocolsCurvs,
}

#[cw_serde]
pub struct ZkProof {
    pub proof: HsZkProof,
    pub public_signales: Vec<String>,
    pub proof_type: HypersignKYCProofTypes,
}

#[cw_serde]
pub struct HypersignKYCProof {
    pub credential_id: Option<String>, // verifiable credential id linked with this proof
    pub zk_proof: ZkProof,
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

pub type CW721OnChainMetadataInstantiateMsg = cw721_metadata_onchain::InstantiateMsg;
