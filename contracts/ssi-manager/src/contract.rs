use crate::error::KycContractError;
use crate::state::{
    COUNTER, INSTANTIATE_TOKEN_REPLY_ID, OWNER, OWNERDID, SBT_CODE_ID, SUPPORTED_DID_METHOD,
};
use crate::{msg::Cw721InstantiateMsg, msg::InstantiateMsg, state::*};
use cosmwasm_std::{
    to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response,
    StdError, StdResult, SubMsg, WasmMsg,
};

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

    // TODO: cleanup states...
    // initiate the counter = 0
    COUNTER.save(deps.storage, &0)?;

    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{
        msg::{ResolveDIDResp, SBTcontractAddressResp, ValueResp},
        state::COUNTER,
        state::OWNERDID,
        state::SBT_CONTRACT_ADDRESS,
    };

    use super::DID_REGISTRY;

    pub fn getOwnerDID(deps: Deps) -> StdResult<ValueResp> {
        Ok(ValueResp {
            owner_did: OWNERDID.load(deps.storage)?,
        })
    }

    pub fn resolve_did(deps: Deps, did: &str) -> StdResult<ResolveDIDResp> {
        Ok(ResolveDIDResp {
            did_doc: DID_REGISTRY.load(deps.storage, did)?,
        })
    }

    pub fn getSbtContractAddress(deps: Deps) -> StdResult<SBTcontractAddressResp> {
        Ok(SBTcontractAddressResp {
            sbt_contract_address: SBT_CONTRACT_ADDRESS.load(deps.storage)?,
        })
    }
}

pub mod exec {
    use didkit::ssi::did::Document;
    use tokio::runtime::Runtime;

    use super::{
        COUNTER, DID_REGISTRY, INSTANTIATE_TOKEN_REPLY_ID, OWNER, SBT_CODE_ID,
        SBT_CONTRACT_ADDRESS, SUPPORTED_DID_METHOD,
    };
    use crate::{
        ed25519_signature_2020,
        error::KycContractError,
        msg::{Cw721InstantiateMsg, DIDDocumentProof, ExecMsg, ExecuteNFTMsg},
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn,
        Response, StdError, StdResult, SubMsg, WasmMsg,
    };

    pub fn init(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_code_id: u64,
    ) -> Result<Response, KycContractError> {
        // let token_code_id = SBT_CODE_ID.load(deps.storage)?;
        // println!("token_code_id {:?}", token_code_id);

        SBT_CODE_ID.save(deps.storage, &token_code_id)?;

        // Instantiating a new SBT contract
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: token_code_id,
                msg: to_json_binary(&Cw721InstantiateMsg {
                    name: "SBT".to_owned(),
                    symbol: "SBT".to_owned(),
                    minter: env.contract.address.clone().into(), //
                })?,
                funds: vec![],
                admin: Some(info.sender.to_string()),
                label: String::from("Instantiate fixed price NFT contract"),
            }
            .into(),
            id: INSTANTIATE_TOKEN_REPLY_ID,
            gas_limit: None,
            reply_on: ReplyOn::Success,
        }];

        let resp = Response::new().add_submessages(sub_msg);
        Ok(resp)
    }

    pub fn mint(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, KycContractError> {
        let mut resp = Response::new();

        let sbt_contract_address = SBT_CONTRACT_ADDRESS.load(deps.storage)?;

        // fetch the counter
        let value: u64 = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;

        // Mint SBT to the issuer_kyc_contract
        let msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&ExecuteNFTMsg::Mint {
                token_id: value.clone().to_string(),
                owner: env.contract.address.to_string(),
                token_uri: None,
                extension: None,
            })?,
            funds: (&[]).to_vec(),
        };
        resp = resp.add_message(msg);

        // transfer SBT to the user
        let transfer_msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&ExecuteNFTMsg::TransferNft {
                recipient: info.sender.to_string(),
                token_id: value.clone().to_string(),
            })?,
            funds: (&[]).to_vec(),
        };

        resp = resp.add_message(transfer_msg);

        resp = resp
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str());
        // .add_attribute("counter", counter.to_string());
        Ok(resp)
    }

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
        let did_already_exists = DID_REGISTRY.has(deps.storage, &did.clone());
        if did_already_exists {
            return Err(KycContractError::DIDAlreadyRegistred { did: did.into() });
        }

        // TODO:: 3. verify did_doc_proof
        //let message = ed25519_signature_2020::transform_proof_message(did_doc, did_doc_proof).await;
        // let parsed_did_doc_proof: DIDDocumentProof = serde_json::from_str(did_doc_proof)?;
        // match serde_json::from_str(did_doc_proof) {
        //     Ok(parsed_doc) => {
        //         let parsed_did_doc_proof: DIDDocumentProof = parsed_doc;
        //         println!(
        //             "VerificationMethod: {}",
        //             parsed_did_doc_proof.verification_method
        //         );

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
