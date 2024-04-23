use crate::msg::Issuer;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

// https://docs.cosmwasm.com/docs/smart-contracts/state/cw-plus#map
pub const ISSUERS: Map<&str, Issuer> = Map::new("issuers");
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
