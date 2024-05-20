use cosmwasm_std::{to_binary, QuerierWrapper, QueryRequest, StdResult, WasmQuery};

pub fn resolve_a_did(
    querier: &QuerierWrapper,
    did: &str,
    ssi_manager_contract_address: &str,
) -> StdResult<ssi_manager::msg::ResolveDIDResp> {
    let query_message = ssi_manager::msg::QueryMsg::ResolveDID {
        did: did.to_string(),
    };
    let request = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: ssi_manager_contract_address.to_string(),
        msg: to_binary(&query_message)?,
    });
    let resolve_did_query_resp = querier.query(&request);
    return resolve_did_query_resp;
}
