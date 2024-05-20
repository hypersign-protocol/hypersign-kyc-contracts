use crate::msg::Issuer;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

// https://docs.cosmwasm.com/docs/smart-contracts/state/cw-plus#map
// did <> Issuer
pub const ISSUERS: Map<&str, Issuer> = Map::new("issuers");
pub const ISSUERS_TEMP: Map<u64, Issuer> = Map::new("issuers_temp");
pub const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;
pub const DUMMY_ISSUER_ID: &str = "issuer-1";

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// struct Data {
//     pub name: String,
//     pub age: i32,
// }
// pub const PEOPLE: Map<&str, Data> = Map::new("people");

// obsolute
pub const COUNTER: Item<u64> = Item::new("counter");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const COUNTER_PROXY_ADDR: Item<String> = Item::new("counter_proxy_contract_address");

pub const HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS: Item<String> =
    Item::new("hypersign_ssi_manager_contract_address");

pub const ISSUER_KYC_CONTRACT_CODE_ID: Item<u64> = Item::new("issuer_kyc_contract_code_id");

// pub hypersign_ssi_manager_code_id: u64,
