use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, WasmMsg};
use cw721_base::ContractError;

use crate::{msg::InstantiateMsg, state::*};

pub fn instantiate(deps: DepsMut, msg: InstantiateMsg, info: MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;

    // TODO check if ssi manager contract address is passed
    // if msg.hypersign_ssi_manager_contract_address.is_ascii(){
    //     return Std::Err("Hypersign SSI manager contract address must be passed".to_string());
    // }

    HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS
        .save(deps.storage, &msg.hypersign_ssi_manager_contract_address)?;

    ISSUER_KYC_CONTRACT_CODE_ID.save(deps.storage, &msg.kyc_contract_code_id)?;
    Ok(Response::new())
}

pub mod query {
    use crate::error::ContractError;
    use crate::{
        msg::{RegistredIssuerResp, SSIManagerContractAddressResp, ValueResp, ValueRespProxy},
        state::{HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS, ISSUERS},
    };
    use cosmwasm_std::{Deps, Response, StdError, StdResult};
    use serde::de::value::Error;

    pub fn get_registred_issuer(deps: Deps, issuer_did: String) -> StdResult<RegistredIssuerResp> {
        //// TODO: check if the key does not exist in the map and thrown error otherwise
        // let issuer_already_exists = ISSUERS.has(deps.storage, &issuer_did.clone());
        // if !issuer_already_exists {
        //     return Err(ContractError::InvalidIssuerDID {
        //         issuer_did: issuer_did.into(),
        //     });
        // }
        Ok(RegistredIssuerResp {
            issuer: ISSUERS.load(deps.storage, issuer_did.as_str())?,
        })

        // match ISSUERS.load(deps.storage, issuer_did.as_str()) {
        //     Ok(value) => value,
        //     Err(error) => Err("Invalid issuer DID"),
        // };
    }

    pub fn get_ssi_manager_contract_address(
        deps: Deps,
    ) -> StdResult<SSIManagerContractAddressResp> {
        Ok(SSIManagerContractAddressResp {
            contract_address: HYPERSIGN_SSI_MANAGER_CONTRACT_ADDRESS.load(deps.storage)?,
        })
    }
}

pub mod exec {
    use super::{
        COUNTER, INSTANTIATE_TOKEN_REPLY_ID, ISSUERS, ISSUERS_TEMP, ISSUER_KYC_CONTRACT_CODE_ID,
    };
    use crate::{
        error::ContractError,
        msg::{
            Cw721InstantiateMsg, ExecMsg, ExecuteNFTMsg, Issuer, IssuerKycInstantiateMsg,
            NftInstantiateMsg, ResponseD,
        },
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, Event, MessageInfo, ReplyOn,
        Response, StdError, StdResult, SubMsg, WasmMsg,
    };

    pub fn onboard_issuer(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        issuer_did: String,
    ) -> Result<Response, ContractError> {
        // TODO: check if issuer_did already exists in the ISSUER map
        let issuer_already_exists = ISSUERS.has(deps.storage, &issuer_did.clone());
        if issuer_already_exists {
            return Err(ContractError::IssuerDIDAlreadyRegistred {
                issuer_did: issuer_did.into(),
            });
        }

        // TODO: optimization: we could simply use ISSUER_TEMP keys length... may be more efficient
        let mut counter = COUNTER.load(deps.storage)?;
        let issuer_kyc_code_id = ISSUER_KYC_CONTRACT_CODE_ID.load(deps.storage)?;
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: issuer_kyc_code_id,
                msg: to_json_binary(&IssuerKycInstantiateMsg {
                    owner_did: issuer_did.clone().into(),
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
        println!("Issuer {}", issuer_did.clone());

        // let isuerID = "issuer-"
        //     .to_owned()
        //     .push_str(&counter.to_string())
        //     .to_owned();

        let issuer = Issuer {
            id: "issuer-1".into(), // TODO: make the number dynamic
            did: issuer_did.clone().into(),
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
