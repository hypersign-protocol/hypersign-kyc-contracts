use self::error::ContractError;
use super::*;
use msg::InstantiateMsg;
use state::{COUNTER, COUNTER_PROXY_ADDR, INSTANTIATE_TOKEN_REPLY_ID};

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
        Poke {
            proxy_contract_addr,
        } => exec::poke(_deps, _info, proxy_contract_addr).map_err(ContractError::from),
        Donate {} => exec::donate(_deps, _info, _env).map_err(ContractError::from),
        Withdraw {} => exec::widthdraw(_deps, _env, _info),
        Deploy { token_code_id } => exec::deploy_nft_contract(_deps, _info, _env, token_code_id)
            .map_err(ContractError::from),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&query::value(deps)?),
        GetProxyMessage {} => to_binary(&query::getProxyMessage(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenId { token_id: msg.id });
    }

    let value: u64 = COUNTER.load(deps.storage)? + 1;
    COUNTER.save(deps.storage, &value)?;

    let reply = parse_reply_instantiate_data(msg).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();

    COUNTER_PROXY_ADDR.save(deps.storage, &cw721_address)?;

    Ok(Response::new())
}
