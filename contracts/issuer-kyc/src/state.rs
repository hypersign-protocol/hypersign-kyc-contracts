use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

pub const OWNERDID: Item<String> = Item::new("owner_did");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const SBT_CONTRACT_ADDRESS: Item<String> = Item::new("sbt_contract_address");
pub const COUNTER: Item<u64> = Item::new("sbt_counter");
pub const SBT_CODE_ID: Item<u64> = Item::new("sbt_code_id");
pub const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;
