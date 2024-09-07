#[cfg(any(test, feature = "tests"))]
pub mod test {

    use crate::entry::{self, *};

    use crate::msg::{
        ExecMsg, GetDIDVerStatusResp, InstantiateMsg, QueryMsg, ResolveDIDResp, VerifyProofsResp,
    };

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Empty};
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
        use crate::lib_json_ld::{get_cannonized_str, hash_string};

        use std::fs;
        use std::io;
        use std::error::Error;
        use std::path::PathBuf;
        use serde_json::Value;
        use cw721_base::ExecuteMsg;
        

        // Read the expanded did
        // let expanded_did = "./test/expanded_test_basic.json";
        // let expanded_did_str = fs::read_to_string(expanded_did).expect("Failed");
        // let expanded_did_json: Value = serde_json::from_str(&expanded_did_str).expect("Failed");
        // let cannonized_did = get_cannonized_str(expanded_did_json.clone());
        
        // // println!("{}", cannonized_did);

        // // Read the expanded did proof
        // let expanded_did_proof =  "./test/expanded_test_didproof.json";
        // let expanded_did_proof_str = fs::read_to_string(expanded_did_proof).expect("Failed");
        // let expanded_did_proof_json: Value = serde_json::from_str(&expanded_did_proof_str).expect("Failed");
        // let cannonized_did_proof = get_cannonized_str(expanded_did_proof_json.clone());
        // println!("{}", cannonized_did_proof);

        // // // Test verification
        // let public_key = "z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp";
        // let m1 = hash_string(&cannonized_did.clone()); 

        // let m2 = hash_string(&cannonized_did_proof.clone()); // Using expanded did proof
        // let message = [m2.clone(), m1.clone()].concat();
                        
        let signature = "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq";
        let msg = ExecMsg::VerifySignature { 
                            public_key: public_key.to_string(), 
                            message: message.to_string(), 
                            signature: signature.to_string()
                        };
        let info = mock_info("sender", &[]);
        let env = mock_env();
        let mut deps = mock_dependencies();
        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(res.attributes, vec![("verification", "success")]);
        // assert_eq!(true, false);
    } 
}
