use cosmwasm_schema::write_api;
use hypersign_factory::msg::{ExecMsg, InstantiateMsg, QueryMsg};
 
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecMsg,
        query: QueryMsg,
    }
}