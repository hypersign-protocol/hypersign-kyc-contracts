use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Empty, StdResult, WasmMsg};
use cw721_base::Extension;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use secret_toolkit::utils::InitCallback;

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Option<Coin>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
    #[returns(ValueRespProxy)]
    GetProxyMessage {},
}

#[cw_serde]
pub enum ExecMsg {
    Poke { proxy_contract_addr: String },
    Donate {},
    Withdraw {},
    Deploy { token_code_id: u64 },
}

#[cw_serde]
pub struct ValueResp {
    pub value: u64,
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

pub type ExecuteNFTMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub type Cw721InstantiateMsg = cw721_base::InstantiateMsg;
