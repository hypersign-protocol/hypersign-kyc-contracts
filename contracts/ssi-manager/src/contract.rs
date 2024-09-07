use crate::error::KycContractError;
use crate::state::{COUNTER, OWNER, SUPPORTED_DID_METHOD};
use crate::{msg::InstantiateMsg, state::*};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::lib_json_ld::get_cannonized_str;

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
    use super::{DID_REGISTRY, DID_VER_STATUS, SUPPORTED_DID_METHOD};

    use crate::{
        ed25519_signature_2020,
        lib_json_ld,
        lib_json_ld::get_cannonized_str,
        error::KycContractError
    };
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
    use crate::msg::ExecMsg;

    pub fn register_did(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        did: &str,
        did_doc: &str,
        did_doc_proof: &str,
        signature: &str
    ) -> Result<Response, KycContractError> {
        let mut resp = Response::new();

        // 1. Check if did is passed
        if did.is_empty() {
            return Err(KycContractError::EmptyDID {});
        }

        // TODO: 2. Check if did is of supported did method
        // let supported_did_method = SUPPORTED_DID_METHOD.load(deps.storage)?;
        let did_string = String::from(did);
        

        // TODO: Check if DID alredy registered, else throw error
        // let did_already_exists = DID_REGISTRY.has(deps.storage, &did);
        // if did_already_exists {
        //     return Err(KycContractError::DIDAlreadyRegistred { did: did.into() });
        // }

        // TODO:: 3. verify did_doc_proof
        // Get cannonized strings
        let cannonized_did  = get_cannonized_str(did_doc.to_string());
        let cannonized_did_proof  = get_cannonized_str(did_doc_proof.to_string());

        // Get pubkey
        let public_key = lib_json_ld::extract_after_last_delimiter(did, ':');
        let m1 = lib_json_ld::hash_string(&cannonized_did);
        let m2 = lib_json_ld::hash_string(&cannonized_did_proof);

        // Get the signature from the did proof
        let message = [m2.clone(), m1.clone()].concat();

        let result =
            ed25519_signature_2020::try_verify_signature(
                    public_key.to_string(), 
                    message.to_string(), 
                    signature.to_string(), 
                    deps
                );
        // DID_VER_STATUS.save(deps.storage, &result?.clone())?;

        // let did_document_parsed: Document = Document::from_json(did_doc).expect("JSON was not well-formatted");
        // DID_REGISTRY.save(deps.storage, did, &did_doc.to_owned())?;
        
        // Send the response
        resp = resp
            .add_attribute("action", "register_did")
            .add_attribute("result", result?.to_string())
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("did", did.to_string());
        Ok(resp)
    }
}
