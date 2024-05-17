use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Empty, StdResult, WasmMsg};
use cw721_base::Extension;
use cw_storage_plus::Item;
use didkit::ssi::did::Contexts;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use secret_toolkit::utils::InitCallback;

use didkit::ssi::did::Document;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner_did: String,
    // pub token_code_id: u64,
    pub did_method: String,
}

#[derive(Deserialize, Debug)]
pub struct DIDDocumentProof {
    #[serde(rename = "@context")]
    pub context: Contexts,

    #[serde(rename = "type")]
    pub type_: String,
    pub created: String,

    #[serde(rename = "verificationMethod")]
    pub verification_method: String,

    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,

    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    OwnerDID {},

    #[returns(SBTcontractAddressResp)]
    SBTContractAddress {},

    #[returns(ResolveDIDResp)]
    ResolveDID { did: String },
}

#[cw_serde]
pub enum VerificationKeys {
    Ed25519VerificationKey2020,
}

#[cw_serde]
pub enum SignatureTypes {
    Ed25519Signature2020,
}

#[cw_serde]
pub enum ProofPurpose {
    assertionMethod,
}

#[cw_serde]
pub enum ExecMsg {
    RegisterDID { did: String, did_doc: String, did_doc_proof: String }
}

#[cw_serde]
pub struct ValueResp {
    pub owner_did: String,
}

#[cw_serde]
pub struct Issuer {
    pub id: String,
    pub did: String,
    pub kyc_contract_address: Option<String>,
}

#[cw_serde]
pub struct ResolveDIDResp {
    pub did_doc: String,
}

#[cw_serde]
pub struct SBTcontractAddressResp {
    pub sbt_contract_address: String,
}

pub type ExecuteNFTMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub type Cw721InstantiateMsg = cw721_base::InstantiateMsg;
