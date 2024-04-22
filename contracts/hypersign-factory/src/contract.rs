use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, WasmMsg};

use crate::{msg::InstantiateMsg, state::*};

pub fn instantiate(deps: DepsMut, msg: InstantiateMsg, info: MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;
    if let Some(min_donation) = msg.minimal_donation {
        MINIMAL_DONATION.save(deps.storage, &min_donation)?;
    }
    //
    OWNER.save(deps.storage, &info.sender);
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{
        msg::{ValueResp, ValueRespProxy},
        state::COUNTER,
        state::COUNTER_PROXY_ADDR,
    };
    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value: u64 = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }

    pub fn getProxyMessage(deps: Deps) -> StdResult<ValueRespProxy> {
        Ok(ValueRespProxy {
            proxyContractAddress: COUNTER_PROXY_ADDR.load(deps.storage)?,
        })
    }
}

pub mod exec {
    use super::{COUNTER, COUNTER_PROXY_ADDR, INSTANTIATE_TOKEN_REPLY_ID, MINIMAL_DONATION, OWNER};
    use crate::{
        error::ContractError,
        msg::{Cw721InstantiateMsg, ExecMsg, ExecuteNFTMsg, NftInstantiateMsg},
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn,
        Response, StdError, StdResult, SubMsg, WasmMsg,
    };

    pub fn poke(
        deps: DepsMut,
        info: MessageInfo,
        proxy_contract_addr: String,
    ) -> StdResult<Response> {
        let value: u64 = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;

        COUNTER_PROXY_ADDR.save(deps.storage, &proxy_contract_addr)?;

        // we can use closure in update() of
        // COUNTER.update(deps.storage, |counter| -> StdResult<_> { Ok(counter + 1)})? ;

        // lets try to emit some event that the state has been updated
        // Events are emitted from execution using the Response::add_event function, passing the constructed Event type.
        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", value.to_string());
        Ok(resp)
    }

    pub fn donate(deps: DepsMut, info: MessageInfo, env: Env) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;
        let mut resp = Response::new();
        // Funds can be addressed by `info.funds` argument
        if info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            counter += 1;

            COUNTER.save(deps.storage, &counter)?;
            let value: u64 = COUNTER.load(deps.storage)? + 1;
            COUNTER.save(deps.storage, &value)?;

            // call exec Poke() of the counting_contract_proxy
            // get the contractaddress counting_contract_proxy
            // form the message
            //

            let proxy_contract_addr = COUNTER_PROXY_ADDR.load(deps.storage)?;
            println!(
                "proxy_contract_addr ---------- {:?}",
                proxy_contract_addr.clone().to_string()
            );

            // let msg = WasmMsg::Execute {
            //     contract_addr: proxy_contract_addr.clone(),
            //     msg: to_json_binary(&ExecMsg::Poke {
            //         proxy_contract_addr: proxy_contract_addr.clone().to_string()
            //     })?,
            //     funds: (&[]).to_vec(),
            // };

            // mint NFT
            let msg = WasmMsg::Execute {
                contract_addr: proxy_contract_addr.clone(),
                msg: to_json_binary(&ExecuteNFTMsg::Mint {
                    token_id: counter.clone().to_string(),
                    owner: env.contract.address.to_string(),
                    token_uri: None,
                    extension: None,
                })?,
                funds: (&[]).to_vec(),
            };

            println!("msg ------------ {:?}", msg);
            resp = resp.add_message(msg);

            // transfer NFT
            let transfer_msg = WasmMsg::Execute {
                contract_addr: proxy_contract_addr.clone(),
                msg: to_json_binary(&ExecuteNFTMsg::TransferNft {
                    recipient: info.sender.to_string(),
                    token_id: counter.clone().to_string(),
                })?,
                funds: (&[]).to_vec(),
            };
            println!("msg ------------ {:?}", transfer_msg);

            resp = resp.add_message(transfer_msg);
        };
        resp = resp
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str());
        // .add_attribute("counter", counter.to_string());
        Ok(resp)
    }

    pub fn widthdraw(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        // only owner can call this method
        if info.sender != owner {
            return Err(ContractError::Unauthorized {
                owner: owner.to_string(),
            });
        }

        // fetch all balances of this contract
        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        // sending the message to transfer all amount to the info.sender in the blockcain
        let bank_msg = BankMsg::Send {
            to_address: owner.to_string(),
            amount: balance,
        };

        // emmiting event
        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }

    pub fn deploy_nft_contract(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_code_id: u64,
    ) -> StdResult<Response> {
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: token_code_id,
                msg: to_json_binary(&Cw721InstantiateMsg {
                    name: "issuerNFT".to_owned(),
                    symbol: "issuerNFT".to_owned(),
                    minter: env.contract.address.clone().to_string(), //
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
        Ok(resp)
    }

    //
}
