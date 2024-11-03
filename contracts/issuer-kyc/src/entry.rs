use self::error::KycContractError;
use super::*;

use cosmwasm_std::Coin;
use cosmwasm_std::Reply;
use cosmwasm_std::SubMsgResponse;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_utils::parse_reply_instantiate_data;
use msg::InstantiateMsg;
use semver::Version;
use state::{COUNTER, INSTANTIATE_TOKEN_REPLY_ID, SBT_CONTRACT_ADDRESS};

// version info for migration info
const CONTRACT_NAME: &str = "hypersign.id:issuer-kyc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, KycContractError> {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    if storage_version < version {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("new_version", CONTRACT_VERSION))
}

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
        Mint { hypersign_proof } => exec::mint(_deps, _info, _env, hypersign_proof),
        Init {
            token_code_id,
            label,
        } => exec::init(_deps, _info, _env, token_code_id, label),
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

    let reply = cw_utils::parse_reply_instantiate_data(msg).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();

    SBT_CONTRACT_ADDRESS
        .save(deps.storage, &cw721_address)
        .unwrap();
    Ok(Response::new())
}
