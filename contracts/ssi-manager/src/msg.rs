use cosmwasm_schema::{cw_serde, QueryResponses};
// use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Empty, StdResult, WasmMsg};
use cosmwasm_std::Empty;
use cw721_base::Extension;
use cosmwasm_std::{Binary, StdError};
// use cw_storage_plus::Item;
// use didkit::ssi::did::Contexts;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
// use secret_toolkit::utils::InitCallback;

// use didkit::ssi::did::Document;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner_did: String,
    // pub token_code_id: u64,
    pub did_method: String,
}

// #[derive(Deserialize, Debug)]
// pub struct DIDDocumentProof {
//     #[serde(rename = "@context")]
//     pub context: Contexts,

//     #[serde(rename = "@type")]
//     pub type_: String,
//     pub created: String,

//     #[serde(rename = "verificationMethod")]
//     pub verification_method: String,

//     #[serde(rename = "proofPurpose")]
//     pub proof_purpose: String,

//     #[serde(rename = "proofValue")]
//     pub proof_value: String,
// }

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    OwnerDID {},

    #[returns(SBTcontractAddressResp)]
    SBTContractAddress {},

    #[returns(ResolveDIDResp)]
    ResolveDID { did: String },

    #[returns(GetDIDVerStatusResp)]
    GetDIDVerStatus {},

    #[returns(VerifyProofsResp)]
    VerifySSIProof {
        public_key_str: String,
        signature_str: String,
        message: String,
    },
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
    RegisterDID {
        did: String,
        did_doc: String,
        did_doc_proof: String,
        signature: String
    },
    VerifySignature {
        public_key: String,
        message: String,
        signature: String,
    }
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
pub struct GetDIDVerStatusResp {
    pub status: bool,
}

#[cw_serde]
pub struct SBTcontractAddressResp {
    pub sbt_contract_address: String,
}

#[cw_serde]
pub struct VerifyProofsResp {
    pub result: bool,
}

#[cw_serde]
pub struct Proof {
    pub challenge: String,
    pub created: String,
    pub domain: String,
    pub proof_purpose: String,
    pub proof_value: String,
    // "@type": "Ed25519Signature2020",
    // "challenge": "1231231231",
    // "created": "2024-05-09T08:01:46Z",
    // "domain": "www.adbv.com",
    // "proofPurpose": "https://w3id.org/security#authenticationMethod",
    // "proofValue": "z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa",
    // "verificationMethod": "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1"
}

pub type ExecuteNFTMsg = cw721_base::ExecuteMsg<Extension, Empty>;

pub type Cw721InstantiateMsg = cw721_base::InstantiateMsg;
