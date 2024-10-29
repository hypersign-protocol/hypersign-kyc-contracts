use self::error::FactoryContractError;
use self::state::ISSUERS_TEMP;
use super::*;
use msg::{InstantiateMsg, Issuer};
use state::ISSUERS;

use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cosmwasm_std::{Coin, Reply};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, FactoryContractError> {
    contract::instantiate(_deps, _msg, _info)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: msg::ExecMsg,
) -> Result<Response, FactoryContractError> {
    use contract::exec;
    use msg::ExecMsg::*;

    match _msg {
        OnboardIssuer {
            did_doc,
            did_doc_proof,
            signature,
            label,
        } => exec::onboard_issuer(_deps, _info, _env, did_doc, did_doc_proof, signature, label)
            .map_err(FactoryContractError::from),

        UpdateIssuerContractCode {
            did_doc,
            did_doc_proof,
            signature,
            kyc_contract_code_id,
        } => exec::update_issuer_kyc_contract_code(
            _deps,
            _info,
            _env,
            did_doc,
            did_doc_proof,
            signature,
            kyc_contract_code_id,
        )
        .map_err(FactoryContractError::from),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        GetRegisteredIssuer { issuer_did } => {
            to_binary(&query::get_registred_issuer(deps, issuer_did)?)
        }

        GetHypersignAdminDID {} => to_binary(&query::get_hypersign_admin_did(deps)?),

        GetIssuerKYCContractCodeID {} => to_binary(&query::get_issuer_kyc_contract_code_id(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, FactoryContractError> {
    let reply = cw_utils::parse_reply_instantiate_data(msg.clone()).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();

    // load the temporary issuer id
    let mut issuer_temp: Issuer = ISSUERS_TEMP.load(deps.storage, msg.id.clone())?;

    // TODO: check if key = msg.id exists in the ISSUERS_TEMP

    issuer_temp.kyc_contract_address = Some(cw721_address);

    // store in the permanent issuer location
    ISSUERS.save(deps.storage, issuer_temp.did.as_str(), &issuer_temp)?;

    // clean up item from temp storage
    ISSUERS_TEMP.remove(deps.storage, msg.id);

    Ok(Response::new())
}
