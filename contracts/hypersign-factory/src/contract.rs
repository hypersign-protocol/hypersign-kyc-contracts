use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, WasmMsg};

use crate::{msg::InstantiateMsg, state::*};

pub fn instantiate(deps: DepsMut, msg: InstantiateMsg, info: MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{
        msg::{RegistredIssuerResp, ValueResp, ValueRespProxy},
        state::{DUMMY_ISSUER_ID, ISSUERS},
    };

    pub fn get_registred_issuer(deps: Deps) -> StdResult<RegistredIssuerResp> {
        Ok(RegistredIssuerResp {
            issuer: ISSUERS.load(deps.storage, DUMMY_ISSUER_ID)?,
        })
    }
}

pub mod exec {
    use super::{INSTANTIATE_TOKEN_REPLY_ID, ISSUERS};
    use crate::{
        error::ContractError,
        msg::{
            Cw721InstantiateMsg, ExecMsg, ExecuteNFTMsg, Issuer, IssuerKycInstantiateMsg,
            NftInstantiateMsg,
        },
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn,
        Response, StdError, StdResult, SubMsg, WasmMsg,
    };

    pub fn onboard_issuer(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        issuer_did: String,
        issuer_kyc_code_id: u64,
    ) -> StdResult<Response> {
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: issuer_kyc_code_id,
                msg: to_json_binary(&IssuerKycInstantiateMsg {
                    owner_did: issuer_did.to_string(),
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

        // let issuer = Issuer {
        //     id: "issuer-1".into(),
        //     did: issuer_did.into(),
        //     kyc_contract_address: None,
        // };

        // ISSUERS.save(deps.storage, issuer_did.as_str(), &issuer)?;

        Ok(resp)
    }

    //
}
