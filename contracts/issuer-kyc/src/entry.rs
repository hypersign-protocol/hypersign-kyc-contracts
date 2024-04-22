use self::error::KycContractError;
use super::*;
use msg::InstantiateMsg;
use state::{COUNTER, COUNTER_PROXY_ADDR, INSTANTIATE_TOKEN_REPLY_ID, SBT_CONTRACT_ADDRESS};

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
        Mint {} => exec::mint(_deps, _info, _env),
        Init {} => exec::init(_deps, _info, _env),
        // Deploy { token_code_id } => exec::deploy_nft_contract(_deps, _info, _env, token_code_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        OwnerDID {} => to_binary(&query::getOwnerDID(deps)?),
        SBTContractAddress {} => to_binary(&query::getSbtContractAddress(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, KycContractError> {
    println!("Inside reply function........");
    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(KycContractError::InvalidTokenId { token_id: msg.id });
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();

    SBT_CONTRACT_ADDRESS.save(deps.storage, &cw721_address)?;
    Ok(Response::new())
}

// // Reply callback triggered from cw721 contract instantiation
// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {

//     if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
//         return Err(ContractError::InvalidTokenId { token_id: msg.id});
//     }

//     let value: u64 = COUNTER.load(deps.storage)? + 1;
//     COUNTER.save(deps.storage, &value)?;

//     let reply = parse_reply_instantiate_data(msg).unwrap();
//     let cw721_address = Addr::unchecked(reply.contract_address).into();

//     COUNTER_PROXY_ADDR.save(deps.storage, &cw721_address)?;

//     Ok(Response::new())
// }
