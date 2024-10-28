use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, WasmMsg};
use cw721_base::ContractError;

use crate::error::FactoryContractError;
use crate::{helper, msg::InstantiateMsg, state::*};
use serde_json::{from_slice, from_str, Value};
pub fn instantiate(
    deps: DepsMut,
    msg: InstantiateMsg,
    info: MessageInfo,
) -> Result<Response, FactoryContractError> {
    COUNTER.save(deps.storage, &msg.counter)?;

    // HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS
    //     .save(deps.storage, &msg.hypersign_ssi_manager_contract_address)?;

    // checking id DID is registered or not is not required actually, we can simply verify if did proofs are provided
    // or not - you ARE the owner of this did, thats it!
    match ssi_manager::ed25519_signature_2020::verify(
        msg.did_doc.to_owned(),
        msg.did_doc_proof.to_owned(),
        msg.signature.to_owned(),
        &deps,
    ) {
        Ok(is_valid) => {
            if is_valid {
                // let mut resp = Response::new();
                let did_json: Value = serde_json::from_str(&msg.did_doc).expect("Invalid JSON");
                let hypersign_admin_did: String =
                    ssi_manager::lib_json_ld::get_did_value(&did_json);

                if hypersign_admin_did.is_empty() {
                    return Err(FactoryContractError::OnwerDIDRequired {});
                }

                /// Store hypermine admin did here in state
                HYPERSIGN_ADMIN_DID.save(deps.storage, &hypersign_admin_did)?;

                ISSUER_KYC_CONTRACT_CODE_ID.save(deps.storage, &msg.kyc_contract_code_id)?;

                Ok(Response::new())
            } else {
                // If invalid, return a response with a failure attribute
                // Ok(Response::new().add_attribute("verification", is_valid.to_string()))
                Err(FactoryContractError::SignatureMissmatch {})
            }
        }
        Err(err) => {
            // If there's an error, propagate it as a StdError
            Err(FactoryContractError::UnexpectedFailure {})
        }
    }
}

pub mod query {
    use crate::error::FactoryContractError;
    use crate::msg::IssuerKycContractCodeResp;
    use crate::{
        msg::{HypersignAdminDIDResp, RegistredIssuerResp, ValueResp, ValueRespProxy},
        state::{HYPERSIGN_ADMIN_DID, ISSUERS},
    };
    use cosmwasm_std::{Deps, Response, StdError, StdResult};
    use serde::de::value::Error;

    use super::ISSUER_KYC_CONTRACT_CODE_ID;

    pub fn get_registred_issuer(deps: Deps, issuer_did: String) -> StdResult<RegistredIssuerResp> {
        Ok(RegistredIssuerResp {
            issuer: ISSUERS.load(deps.storage, issuer_did.as_str())?,
        })
    }

    pub fn get_hypersign_admin_did(deps: Deps) -> StdResult<HypersignAdminDIDResp> {
        Ok(HypersignAdminDIDResp {
            did: HYPERSIGN_ADMIN_DID.load(deps.storage)?,
        })
    }

    pub fn get_issuer_kyc_contract_code_id(deps: Deps) -> StdResult<IssuerKycContractCodeResp> {
        Ok(IssuerKycContractCodeResp {
            kyc_contract_code_id: ISSUER_KYC_CONTRACT_CODE_ID.load(deps.storage)?,
        })
    }
}

pub mod exec {
    use super::{COUNTER, HYPERSIGN_ADMIN_DID, ISSUERS, ISSUERS_TEMP, ISSUER_KYC_CONTRACT_CODE_ID};
    use crate::{
        error::FactoryContractError,
        helper,
        msg::{ExecMsg, Issuer, IssuerKycInstantiateMsg, NftInstantiateMsg, ResponseD},
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
        signature: String,
        // hypersign_authorization_proof: String // authorization json (string) from hypersign admin
        // hypersign_authorization: String // proof json(string)
        // Take Issuer DID_doc
    ) -> Result<Response, FactoryContractError> {
        // [Optional] TODO check if this issuer did is registed in did registry, if not throw error
        // let resolve_did =
        //     helper::resolve_a_did(&deps.querier, &issuer_did, &ssi_manager_contract_address)?;

        // TODO: throw readable error if the did is not already registered
        // if resolve_did_query_resp {
        // } else {
        //     return Err(FactoryContractError::InvalidIssuerDID { issuer_did });
        // }

        // TODO verify authorization letter from the admin
        /// TODO extract hypersign admin did, check if the hypersign admin did is whitelisted (HYPERSIGN_ADMIN_DID) already in this contract
        /// TODO resolve hypersign admin did and find the public key
        /// TODO verify signature of hypersign did , if successful let issuer onboard himself
        let did_json: Value = serde_json::from_str(&did_doc_str).expect("Invalid JSON");
        let owner_did: String = ssi_manager::lib_json_ld::get_did_value(&did_json);

        let issuer_already_exists = ISSUERS.has(deps.storage, &owner_did);
        if issuer_already_exists {
            return Err(FactoryContractError::IssuerDIDAlreadyRegistred {
                issuer_did: owner_did.into(),
            });
        }

        // TODO: optimization: we could simply use ISSUER_TEMP keys length... may be more efficient
        let mut counter = COUNTER.load(deps.storage)?;
        let issuer_kyc_code_id = ISSUER_KYC_CONTRACT_CODE_ID.load(deps.storage)?;
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

        counter += 1;
        let issuer = Issuer {
            id: "hs-issuer-".to_owned() + &counter.to_string(), // TODO: make the number dynamic
            did: owner_did.clone().into(), // TODO: this need to be updated only whne contract is deployed..
            kyc_contract_address: None,
            kyc_contract_code_id: issuer_kyc_code_id,
        };

        ISSUERS_TEMP.save(deps.storage, counter, &issuer);
        COUNTER.save(deps.storage, &counter);
        let mut resp = Response::new().add_submessages(sub_msg);

        // .add_event(Event::new("admin_added").add_attribute("issuer_did", issuer_did.clone()))
        // .set_data(b"the result data");
        // .set_data(to_json_binary(&IssuerKycInstantiateMsg {
        //     owner_did: issuer_did.clone().into(),
        // })?)

        Ok(resp)
    }

    pub fn update_issuer_kyc_contract_code(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        did_doc_str: String,
        did_doc_proof_str: String,
        signature: String,
        kyc_contract_code_id: u64,
    ) -> Result<Response, FactoryContractError> {
        match ssi_manager::ed25519_signature_2020::verify(
            did_doc_str.to_owned(),
            did_doc_proof_str.to_owned(),
            signature.to_owned(),
            &deps,
        ) {
            Ok(is_valid) => {
                if is_valid {
                    let did_json: Value = serde_json::from_str(&did_doc_str).expect("Invalid JSON");
                    let did: String = ssi_manager::lib_json_ld::get_did_value(&did_json);
                    let hypersign_admin_did = HYPERSIGN_ADMIN_DID.load(deps.storage)?;
                    if hypersign_admin_did != did {
                        return Err(FactoryContractError::Unauthorized {
                            owner: hypersign_admin_did,
                        });
                    }

                    ISSUER_KYC_CONTRACT_CODE_ID.save(deps.storage, &kyc_contract_code_id)?;
                    Ok(Response::new())
                } else {
                    return Err(FactoryContractError::SignatureMissmatch {});
                }
            }
            Err(err) => Err(FactoryContractError::UnexpectedFailure {}),
        }
    }
}
