use self::error::ContractError;
use super::*;
use msg::{InstantiateMsg, Issuer};
use state::{DUMMY_ISSUER_ID, INSTANTIATE_TOKEN_REPLY_ID, ISSUERS};

use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cosmwasm_std::{Coin, Reply};
use cw_utils::parse_reply_instantiate_data;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(_deps, _msg, _info)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: msg::ExecMsg,
) -> Result<Response, ContractError> {
    use contract::exec;
    use msg::ExecMsg::*;

    match _msg {
        OnboardIssuer {
            issuer_did,
            issuer_kyc_code_id,
        } => exec::onboard_issuer(_deps, _info, _env, issuer_did, issuer_kyc_code_id)
            .map_err(ContractError::from),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        GetRegisteredIssuer {} => to_binary(&query::get_registred_issuer(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenId { token_id: msg.id });
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();

    // let issuer = |d: Option<Issuer>| -> StdResult<Issuer> {
    //     match d {
    //         Some(issuer) => Ok(Issuer {
    //             id: "issuer-1".into(),
    //             did: "did:hid:12123123".into(),
    //             kyc_contract_address: cw721_address,
    //         }),
    //         None => Ok(Issuer {
    //             id: "issuer-1".into(),
    //             did: "did:hid:12123123".into(),
    //             kyc_contract_address: cw721_address,
    //         }),
    //     }
    // };

    let issuer = Issuer {
        id: "issuer-1".into(),
        did: "did:hid:12123123".into(),
        kyc_contract_address: Some(cw721_address),
    };

    ISSUERS.save(deps.storage, DUMMY_ISSUER_ID, &issuer)?;
    Ok(Response::new())
}
