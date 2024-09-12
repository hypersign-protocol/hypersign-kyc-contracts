use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Empty, StdResult, WasmMsg};
use cw721_base::Extension;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use secret_toolkit::utils::InitCallback;

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
pub enum ExecMsg {
    Mint {},
    Init { token_code_id: u64 },
    //     Deploy { token_code_id: u64 },
}

#[cw_serde]
pub struct ValueResp {
    pub owner_did: String,
}

#[cw_serde]
pub struct SBTcontractAddressResp {
    pub sbt_contract_address: String,
}

pub type ExecuteNFTMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub type Cw721InstantiateMsg = cw721_base::InstantiateMsg;
