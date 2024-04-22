use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

/**
 * IssuerId:
 *  - id
 *  - did
 *  - walletaddress
 *  - kyc_contract_address
 */
pub struct Issuer {
    pub id: String,
    pub did: String,
    pub walletaddress: String,
    pub kyc_contract_address: String,
}
pub const ISSUERS: Item<Issuer> = Item::new("issuers");

// obsolute
pub const COUNTER: Item<u64> = Item::new("counter");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const COUNTER_PROXY_ADDR: Item<String> = Item::new("counter_proxy_contract_address");
pub const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;
