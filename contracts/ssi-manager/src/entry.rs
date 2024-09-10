use self::error::KycContractError;
use super::*;

// use cosmwasm_std::SubMsgResponse;
use msg::InstantiateMsg;
use state::{INSTANTIATE_TOKEN_REPLY_ID, SBT_CONTRACT_ADDRESS};

// use cosmwasm_std::Coin;
use cosmwasm_std::Reply;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use crate::ed25519_signature_2020::verify_signature;
// use cw_utils::parse_reply_instantiate_data;

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
            did_doc,
            did_doc_proof,
            signature
        } => exec::register_did(_deps, _info, _env, &did_doc, &did_doc_proof, &signature),
        VerifySignature { public_key, message, signature } => {
            Ok(verify_signature(public_key, message, signature, &_deps)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        OwnerDID {} => to_binary(&query::get_owner_did(deps)?),
        SBTContractAddress {} => to_binary(&query::get_sbt_contract_address(deps)?),
        ResolveDID { did } => to_binary(&query::resolve_did(deps, &did)?),
        GetDIDVerStatus {} => to_binary(&query::get_did_ver_status(deps)?),
        VerifySSIProof {
            public_key_str,
            signature_str,
            message,
        } => to_binary(&query::verify_proof(
            deps,
            &public_key_str,
            &signature_str,
            &message,
        )?),
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
