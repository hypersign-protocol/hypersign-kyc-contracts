use crate::error::KycContractError;
use crate::state::{COUNTER, OWNER, SUPPORTED_DID_METHOD};
use crate::{msg::InstantiateMsg, state::*};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate(
    deps: DepsMut,
    msg: InstantiateMsg,
    info: MessageInfo,
    env: Env,
) -> Result<Response, KycContractError> {
    if msg.did_method.is_empty() {
        return Err(KycContractError::EmptyDIDMethod {});
    }

    // TODO: check did method format and through error KycContractError::UnSupportedDIDMethod in case of err
    // https://github.com/spruceid/ssi/blob/976e2607080c20cd5789b977e477e98b6417f8af/did-ethr/src/lib.rs#L21
    SUPPORTED_DID_METHOD.save(deps.storage, &msg.did_method)?;

    // save the owner wallet address
    OWNER.save(deps.storage, &info.sender)?;

    DID_VER_STATUS.save(deps.storage, &false)?;
    // TODO: cleanup states...
    // initiate the counter = 0
    COUNTER.save(deps.storage, &0)?;

    Ok(Response::new())
}

pub mod query {
    use crate::ed25519_signature_2020;
    use cosmwasm_std::{Deps, StdResult};

    use crate::{
        msg::{
            GetDIDVerStatusResp, ResolveDIDResp, SBTcontractAddressResp, ValueResp,
            VerifyProofsResp,
        },
        state::{DID_VER_STATUS, OWNERDID, SBT_CONTRACT_ADDRESS},
    };

    use super::DID_REGISTRY;

    pub fn get_owner_did(deps: Deps) -> StdResult<ValueResp> {
        Ok(ValueResp {
            owner_did: OWNERDID.load(deps.storage)?,
        })
    }

    pub fn resolve_did(deps: Deps, did: &str) -> StdResult<ResolveDIDResp> {
        Ok(ResolveDIDResp {
            did_doc: DID_REGISTRY.load(deps.storage, did)?,
        })
    }

    pub fn get_did_ver_status(deps: Deps) -> StdResult<GetDIDVerStatusResp> {
        Ok(GetDIDVerStatusResp {
            status: DID_VER_STATUS.load(deps.storage)?,
        })
    }

    pub fn get_sbt_contract_address(deps: Deps) -> StdResult<SBTcontractAddressResp> {
        Ok(SBTcontractAddressResp {
            sbt_contract_address: SBT_CONTRACT_ADDRESS.load(deps.storage)?,
        })
    }

    pub fn verify_proof(
        deps: Deps,
        public_key_str: &str,
        signature_str: &str,
        message: &str,
    ) -> StdResult<VerifyProofsResp> {
        let result = ed25519_signature_2020::verify_proof(
            &public_key_str,
            &message,
            &signature_str,
            deps.api,
        );

        Ok(VerifyProofsResp { result: result })
    }
}

pub mod exec {
    // use didkit::ssi::did::Document;
    use super::{DID_REGISTRY, DID_VER_STATUS, SUPPORTED_DID_METHOD};

    use crate::{
        ed25519_signature_2020,
        lib_json_ld,
        error::KycContractError
        // msg::{Cw721InstantiateMsg, ExecMsg, ExecuteNFTMsg},
    };
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
    
    pub fn register_did(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        did: &str,
        did_doc: &str,
        did_doc_proof: &str,
    ) -> Result<Response, KycContractError> {
        let mut resp = Response::new();

        // 1. Check if did is passed
        if did.is_empty() {
            return Err(KycContractError::EmptyDID {});
        }

        // TODO: 2. Check if did is of supported did method
        let supported_did_method = SUPPORTED_DID_METHOD.load(deps.storage)?;
        let did_string = String::from(did);
        // if did_string.contains(supported_did_method) {
        // } else {
        //     return Err(KycContractError::InvalidDIDId { did: did.into() });
        // }

        // TODO: Check if DID alredy registered, else throw error
        let did_already_exists = DID_REGISTRY.has(deps.storage, &did);
        if did_already_exists {
            return Err(KycContractError::DIDAlreadyRegistred { did: did.into() });
        }

        // TODO:: 3. verify did_doc_proof
        // remove hardcoding...
        // do canonizations
        // let m = "40ea48e7bfde895182f57845da0b6648de11a9f31203569d10936a3bba0b1b8f0df7abe82aef2eb7b86bb78897066dca754180a99edd692c66b6fc71d028d5f6";
        // let signature_str = "z4S8Zxko4KLtHEKGkJVSPCrK4PcchJTYmcx3gsgxq3YG8uYQ3DJfaVufTDgjozNV174mZEmmUiib6J917jirmRfnY";
        // let public_key_str = "z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B";

        // Bypass to register even if proof verification fails
        let m = "300ca1bc6cda0ef58ce58f638afc759be35c39fb41ae8879687d9180e581b7201e4c6152326424ee226927ce572264fb05958df55156f8241cf2db3bc113bfb7";
        // let public_key_str = "z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM";
        // let signature_str = "z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa";

        // Get pubkey
        let public_key_str = lib_json_ld::extract_after_last_delimiter(did, ':');
        let m1 = lib_json_ld::hash_string(&did_doc);
        let m2 = lib_json_ld::hash_string(&did_doc_proof);
        // let json_proof: Value = serde_json::from_str(&did_doc_proof)?;
        // let signature_str = json_proof.get("proofValue").and_then(Value::as_str).unwrap_or("");

        // let canonical_json = urdna2015::canonicalize(&json_value)?;
        // println!("Pubkey {:?}", canonical_json);

        let signature_str = "z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa";
        // let m = m1 + m2;
        println!("Pubkey {:?}", m1);

        let result =
            ed25519_signature_2020::verify_proof(&public_key_str, &m, &signature_str, deps.api);
        DID_VER_STATUS.save(deps.storage, &result)?;

        // 4. Store DID into registry ...
        // let did_document_parsed: Document = Document::from_json(did_doc).expect("JSON was not well-formatted");
        DID_REGISTRY.save(deps.storage, did, &did_doc.to_owned())?;

        // Send the response
        resp = resp
            .add_attribute("action", "register_did")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("did", did.to_string());
        Ok(resp)
    }
}
