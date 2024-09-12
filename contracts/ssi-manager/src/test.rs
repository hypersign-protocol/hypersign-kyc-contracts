#[cfg(any(test, feature = "tests"))]
pub mod test {

    use crate::entry::{self, *};

    use crate::msg::{
        ExecMsg, GetDIDVerStatusResp, InstantiateMsg, QueryMsg, ResolveDIDResp, VerifyProofsResp,
    };

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Empty, Attribute, Response};
    use cw721_base::Cw721Contract;
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    fn ssi_manager_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(entry::reply);
        Box::new(contract)
    }

    // fn ssi_manager_contract() -> Box<dyn Contract<Empty>> {
    //     let contract = ContractWrapper::new(execute, instantiate, query).with_reply(entry::reply);
    //     Box::new(contract)
    // }

    fn cw_721_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw721_base::entry::execute,
            cw721_base::entry::instantiate,
            cw721_base::entry::query,
        );
        Box::new(contract)
    }

    // #[test]
    // fn test_initalization() {
    //     let mut deps = mock_dependencies();
    //     let mut app = App::default();
    //     let sbt_contract_code_id = app.store_code(cw_721_contract());

    //     // _deps: DepsMut,
    //     // _env: Env,
    //     // _info: MessageInfo,
    //     // _msg: InstantiateMsg,

    //     // assert min expiration
    //     instantiate(
    //         deps.as_mut(),
    //         mock_env(),
    //         mock_info("mrt", &[]),
    //         InstantiateMsg {
    //             owner_did: "did:hid:12313123123".to_string(),
    //             token_code_id: sbt_contract_code_id,
    //         },
    //     )
    //     .unwrap();
    //     // assert_eq!(error, KycContractError::MinExpiration {});
    // }

    

    // Test Proof verification
    #[test]
    fn kyc_sbt_contracts_initialization() {
        // App simulates blockhain
        let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");

        // storing contract code on blockhain
        let sbt_contract_code_id = app.store_code(cw_721_contract());
        println!("sbt_contract_code_id = {:?}", sbt_contract_code_id);

        let ssi_manger_contract = app.store_code(ssi_manager_contract());
        println!("ssi_manger_contract = {:?}", ssi_manger_contract);

        let contract_addr = app
            .instantiate_contract(
                ssi_manger_contract,
                sender.clone(), // simulating a blockchain address
                &InstantiateMsg {
                    owner_did: "did:hid:12313123123".to_string(),
                    did_method: "did:hid:testnet".to_string(),
                },
                &[],
                "SSI Maager contract",
                None,
            )
            .unwrap();
    

        use crate::ed25519_signature_2020;
        use crate::lib_json_ld::{get_cannonized_str, hash_string, get_did_value };

        use std::fs;
        use std::io;
        use std::error::Error;
        use cw721_base::ExecuteMsg;
        use cosmwasm_std::{Attribute, Response, from_binary, StdResult, Binary};
        use serde_json::{Value, from_slice, from_str}; 

        // Read the expanded did
        let expanded_did = "./test/expanded_test_basic.json";
        let expanded_did_str: Value = from_str(&fs::read_to_string(expanded_did).unwrap()).expect("Failed");
      
        // Read the expanded did proof
        let expanded_did_proof =  "./test/expanded_test_didproof.json";
        let expanded_did_proof_str: Value = from_str(&fs::read_to_string(expanded_did_proof).unwrap()).expect("Failed");
        
        let signature = "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq";

        let msg = ExecMsg::RegisterDID { 
                            did_doc: serde_json::to_string(&expanded_did_str).unwrap(), 
                            did_doc_proof: serde_json::to_string(&expanded_did_proof_str).unwrap(), 
                            signature: signature.to_string()
                        };
        let info = mock_info("sender", &[]);
        let env = mock_env();
        let mut deps = mock_dependencies();

        let response = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let result_value =  response.attributes.iter()
                            .find(|attr| attr.key == "result")
                            .map(|attr| attr.value.clone());

        assert_eq!(result_value.as_deref(), Some("true"));

        // Check storage
        let did = get_did_value(&expanded_did_str);
        let query_msg = QueryMsg::ResolveDID { 
            did: did.to_string()
        };

        let resp = query(
                        deps.as_ref(),
                        env.clone(),
                        query_msg
                    ).unwrap();

        // println!("{:?}", binary_to_json_string(&resp));
        assert_eq!(expanded_did_str.to_string(),  binary_to_json_string(&resp.clone()).unwrap());

        pub fn binary_to_json_string(binary: &Binary) -> StdResult<String> {
            // Deserialize the binary data into a JSON Value
            let json_value: Value = from_slice(&binary)
                .map_err(|e| cosmwasm_std::StdError::parse_err("JSON", e))?;
            
            println!("Key vallye {:?}", get_value_by_key(&json_value, "did_doc"));
            Ok(get_value_by_key(&json_value, "did_doc").expect("REASON"))
        }

        // Function to get the value associated with a specific key
        fn get_value_by_key(v: &Value, key: &str) -> Option<String> {
            v.as_object()
                .and_then(|obj| obj.get(key))
                .map(|val| match val {
                    Value::String(s) => s.clone(), // Return the string directly
                    Value::Number(n) => n.to_string(), // Convert numbers to string
                    Value::Bool(b) => b.to_string(), // Convert boolean to string
                    _ => val.to_string(), // Fallback for other types
                })
        }
    } 
}