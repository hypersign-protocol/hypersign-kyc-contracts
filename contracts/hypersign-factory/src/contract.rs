use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, WasmMsg};
use cw721_base::ContractError;

use crate::{helper, msg::InstantiateMsg, state::*};

pub fn instantiate(deps: DepsMut, msg: InstantiateMsg, info: MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;

    HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS
        .save(deps.storage, &msg.hypersign_ssi_manager_contract_address)?;

    /// Check if hypersign admin is  a registerd did
    let resolve_did = helper::resolve_a_did(
        &deps.querier,
        &msg.hypersign_admin_did,
        &msg.hypersign_ssi_manager_contract_address,
    )?;

    /// Store hypermine admin did here in state
    HYPERSIGN_ADMIN_DID.save(deps.storage, &msg.hypersign_admin_did)?;

    ISSUER_KYC_CONTRACT_CODE_ID.save(deps.storage, &msg.kyc_contract_code_id)?;
    Ok(Response::new())
}

pub mod query {
    use crate::error::ContractError;
    use crate::{
        msg::{
            HypersignAdminDIDResp, RegistredIssuerResp, SSIManagerContractAddressResp, ValueResp,
            ValueRespProxy,
        },
        state::{HYPERSIGN_ADMIN_DID, HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS, ISSUERS},
    };
    use cosmwasm_std::{Deps, Response, StdError, StdResult};
    use serde::de::value::Error;

    pub fn get_registred_issuer(deps: Deps, issuer_did: String) -> StdResult<RegistredIssuerResp> {
        Ok(RegistredIssuerResp {
            issuer: ISSUERS.load(deps.storage, issuer_did.as_str())?,
        })
    }

    pub fn get_ssi_manager_contract_address(
        deps: Deps,
    ) -> StdResult<SSIManagerContractAddressResp> {
        Ok(SSIManagerContractAddressResp {
            contract_address: HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS.load(deps.storage)?,
        })
    }

    pub fn get_hypersign_admin_did(deps: Deps) -> StdResult<HypersignAdminDIDResp> {
        Ok(HypersignAdminDIDResp {
            did: HYPERSIGN_ADMIN_DID.load(deps.storage)?,
        })
    }
}

pub mod exec {
    use super::{
        COUNTER, HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS, INSTANTIATE_TOKEN_REPLY_ID, ISSUERS,
        ISSUERS_TEMP, ISSUER_KYC_CONTRACT_CODE_ID,
    };
    use crate::{
        error::ContractError,
        helper,
        msg::{
            Cw721InstantiateMsg, ExecMsg, ExecuteNFTMsg, Issuer, IssuerKycInstantiateMsg,
            NftInstantiateMsg, ResponseD,
        },
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, Event, MessageInfo,
        QueryRequest, ReplyOn, Response, StdError, StdResult, SubMsg, WasmMsg, WasmQuery,
    };
    use serde_json::{from_slice, from_str, Value};

    pub fn onboard_issuer(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        did_doc_str: String,
        did_doc_proof_str: String,
        signature: String, // hypersign_authorization_proof: String // authorization json (string) from hypersign admin
                           // hypersign_authorization: String // proof json(string)
    ) -> Result<Response, ContractError> {
        let ssi_manager_contract_address =
            HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS.load(deps.storage)?;

        // [Optional] TODO check if this issuer did is registed in did registry, if not throw error
        // let resolve_did =
        //     helper::resolve_a_did(&deps.querier, &issuer_did, &ssi_manager_contract_address)?;

        // TODO: throw readable error if the did is not already registered
        // if resolve_did_query_resp {
        // } else {
        //     return Err(ContractError::InvalidIssuerDID { issuer_did });
        // }

        // TODO verify authorization letter from the admin
        /// TODO extract hypersign admin did, check if the hypersign admin did is whitelisted (HYPERSIGN_ADMIN_DID) already in this contract
        /// TODO resolve hypersign admin did and find the public key
        /// TODO verify signature of hypersign did , if successful let issuer onboard himself
        let did_json: Value = serde_json::from_str(&did_doc_str).expect("Invalid JSON");
        let owner_did: String = ssi_manager::lib_json_ld::get_did_value(&did_json);

        let issuer_already_exists = ISSUERS.has(deps.storage, &owner_did);
        if issuer_already_exists {
            return Err(ContractError::IssuerDIDAlreadyRegistred {
                issuer_did: owner_did.into(),
            });
        }

        // TODO: optimization: we could simply use ISSUER_TEMP keys length... may be more efficient
        let mut counter = COUNTER.load(deps.storage)?;
        let issuer_kyc_code_id = ISSUER_KYC_CONTRACT_CODE_ID.load(deps.storage)?;
        println!("--------------------------------");
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: issuer_kyc_code_id,
                msg: to_json_binary(&IssuerKycInstantiateMsg {
                    did_doc: did_doc_str,
                    did_doc_proof: did_doc_proof_str,
                    signature: signature.to_string(),
                })?,
                funds: vec![],
                admin: Some(info.sender.to_string()),
                label: String::from("Instantiate fixed price NFT contract"),
            }
            .into(),
            id: counter,
            gas_limit: None,
            reply_on: ReplyOn::Success,
        }];
        println!("--------------2------------------");

        let issuer = Issuer {
            id: "issuer-1".into(),         // TODO: make the number dynamic
            did: owner_did.clone().into(), // TODO: this need to be updated only whne contract is deployed..
            kyc_contract_address: None,
            kyc_contract_code_id: issuer_kyc_code_id,
        };

        ISSUERS_TEMP.save(deps.storage, counter, &issuer);
        counter += 1;
        COUNTER.save(deps.storage, &counter);

        let mut resp = Response::new().add_submessages(sub_msg);
        // .add_event(Event::new("admin_added").add_attribute("issuer_did", issuer_did.clone()))
        // .set_data(b"the result data");
        // .set_data(to_json_binary(&IssuerKycInstantiateMsg {
        //     owner_did: issuer_did.clone().into(),
        // })?)

        Ok(resp)
    }

    //
}
