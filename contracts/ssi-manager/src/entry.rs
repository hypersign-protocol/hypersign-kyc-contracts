use self::error::KycContractError;
use super::*;

use cosmwasm_std::SubMsgResponse;
use msg::InstantiateMsg;
use state::{COUNTER, INSTANTIATE_TOKEN_REPLY_ID, SBT_CONTRACT_ADDRESS};

use cosmwasm_std::Coin;
use cosmwasm_std::Reply;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cw_utils::parse_reply_instantiate_data;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, KycContractError> {
    contract::instantiate(_deps, _msg, _info, _env)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: msg::ExecMsg,
) -> Result<Response, KycContractError> {
    use contract::exec;
    use msg::ExecMsg::*;

    match _msg {
        RegisterDID {
            did,
            did_doc,
            did_doc_proof,
        } => exec::register_did(_deps, _info, _env, &did, &did_doc, &did_doc_proof),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        OwnerDID {} => to_binary(&query::getOwnerDID(deps)?),
        SBTContractAddress {} => to_binary(&query::getSbtContractAddress(deps)?),
        ResolveDID { did } => to_binary(&query::resolve_did(deps, &did)?),
        GetDIDVerStatus {} => to_binary(&query::get_did_ver_status(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, KycContractError> {
    println!("Inside reply function........");
    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(KycContractError::InvalidTokenId { token_id: msg.id });
    }

    let reply = cw_utils::parse_reply_instantiate_data(msg).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();

    SBT_CONTRACT_ADDRESS
        .save(deps.storage, &cw721_address)
        .unwrap();
    Ok(Response::new())
}
