use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::collections::HashMap;
pub const OWNERDID: Item<String> = Item::new("owner_did");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const SBT_CONTRACT_ADDRESS: Item<String> = Item::new("sbt_contract_address");
pub const COUNTER: Item<u64> = Item::new("sbt_counter");
pub const SBT_CODE_ID: Item<u64> = Item::new("sbt_code_id");
pub const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;
pub const SBT_NAME: &str = "Hypersign KYC Token";
pub const SBT_SYMBOL: &str = "HKYCToken";

// pub let mut proof_background_color_map: &BTreeMap = BTreeMap::new();
// pub const HYPERSIGN_PROOF_TYPES_SBT_CODE: HashMap<&str, str> = HashMap::from([
//     (&"ProofOfPersonhood", "T1"),
//     (&"ProofOfKYC", "T2"),
//     (&"ProofOfCitizenship", "T3"),
//     (&"ProofOfDateofBirth", "T4"),
//     (&"ProofOfAge", "T5"),
//     (&"ProofOfNonMembershipCountry", "T6"),
//     (&"ProofOfOnchainAML", "T7"),
//     (&"ProofOfTransaction", "T8"),
//     (&"ProofOfUSAccrediatedInvestor", "T9"),
//     (&"ProofOfNonPEP", "T10"),
// ]);

// pub const HYPERSIGN_PROOF_TYPES_DESCRIPTION: HashMap<&str, str> = HashMap::from([
//     (&"ProofOfPersonhood", "Proves that user is not a bot"),
//     (&"ProofOfKYC", "Proves that user has finished his/her KYC"),
//     (&"ProofOfCitizenship", "Proves user is citizen of a country"),
//     (&"ProofOfDateofBirth", "Proves user's date of birth"),
//     (&"ProofOfAge", "Proves user is above or below certain age"),
//     (&"ProofOfNonMembershipCountry", "Proves user are not citizen of non member country"),
//     (&"ProofOfOnchainAML", "Proves that the user's transactions and activities on a blockchain are compliant with anti-money laundering regulations and standards"),
//     (&"ProofOfTransaction", "Proves that a user has done transactions on a specific blockchain "),
//     (&"ProofOfUSAccrediatedInvestor", "Proves that user met criteria set by US security exchange (SEC) to qualify as an accredited investor"),
//     (&"ProofOfNonPEP", "Proves that user is not on any international sanction list or is not classified as PEP"),
// ]);

// pub const HYPERSIGN_PROOF_TYPES_SBT_CODE_BG_COLOR: HashMap<&str, &str> = HashMap::from([
//     ("ProofOfPersonhood", "E5FFCC"),
//     ("ProofOfKYC", "CCE5FF"),
//     ("ProofOfCitizenship", "CCCCFF"),
//     ("ProofOfDateofBirth", "CCFFE5"),
//     ("ProofOfAge", "E5FFCC"),
//     ("ProofOfNonMembershipCountry", "FFCCCC"),
//     ("ProofOfOnchainAML", "FFCCE5"),
//     ("ProofOfTransaction", "FFFFFF"),
//     ("ProofOfUSAccrediatedInvestor", "E5FFCC"),
//     ("ProofOfNonPEP", "FFCCCC"),
// ]);
