use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Empty, StdResult, WasmMsg};
use cw721_base::Extension;
use cw_storage_plus::Item;
use issuer_kyc::msg::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use secret_toolkit::utils::InitCallback;

#[cw_serde]
pub struct Issuer {
    pub id: String,
    pub did: String,
    pub kyc_contract_address: Option<String>,
    pub kyc_contract_code_id: u64,
}

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    pub hypersign_ssi_manager_contract_address: String,
    pub kyc_contract_code_id: u64,
    pub hypersign_admin_did: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RegistredIssuerResp)]
    GetRegisteredIssuer { issuer_did: String },

    #[returns(SSIManagerContractAddressResp)]
    GetSSIManagerContractAddress {},

    #[returns(HypersignAdminDIDResp)]
    GetHypersignAdminDID {},
}

#[cw_serde]
pub enum ExecMsg {
    OnboardIssuer {
        did_doc: String,
        did_doc_proof: String,
        signature: String,
    },
}

#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}

#[cw_serde]
pub struct RegistredIssuerResp {
    pub issuer: Issuer,
}

#[cw_serde]
pub struct SSIManagerContractAddressResp {
    pub contract_address: String,
}

#[cw_serde]
pub struct ValueRespProxy {
    pub proxyContractAddress: String,
}

#[cw_serde]
pub struct NftInstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
}
#[cw_serde]
pub struct HypersignAdminDIDResp {
    pub did: String,
}

// impl InitCallback for  NftInstantiateMsg {
//     // https://github.com/srdtrk/secret-factory-contract/blob/e438495d79b4953c52044e668fa1b9362bfc2cfd/factory/src/state.rs#L10C44-L10C74
//     const BLOCK_SIZE: usize = 256;
// }

pub type ExecuteNFTMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub type Cw721InstantiateMsg = cw721_base::InstantiateMsg;

pub type IssuerKycInstantiateMsg = issuer_kyc::msg::InstantiateMsg;

#[cw_serde]
pub struct ResponseD {
    pub issuer_did: String,
}
