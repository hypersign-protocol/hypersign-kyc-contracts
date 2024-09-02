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

        // Define paths to local context files
        let context_paths = [
            "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context.json", 
        ];

        // Read all JSON files in the directory and handle errors
        let dir_path = "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test";
        let documents = match read_json_files_from_dir(dir_path, "test") {
            Ok(docs) => docs,
            Err(e) => {
                eprintln!("Error reading JSON files: {}", e);
                return; // Exit early on error
            }
        };
        
        // Load and parse local contexts
        // let context_map = load_test_context(&context_paths);
        for (i, doc) in documents.iter().enumerate() {
            // Read and parse the JSON file
            match read_json_file(doc.to_path_buf()) {
                Ok(document) => {
                    
                        let merged_context_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context.json";
                       
                        // Read the expanded did
                        let expanded_did =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_test_basic.json";
                        let expanded_did_str = fs::read_to_string(expanded_did).expect("Failed");
                        let expanded_did_json: Value = serde_json::from_str(&expanded_did_str).expect("Failed");
                        let cannonized_did = get_cannonized_str(expanded_did_json.clone());

                        // Read the expanded did proof
                        let expanded_did_proof =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_test_didproof.json";
                        let expanded_did_proof_str = fs::read_to_string(expanded_did_proof).expect("Failed");
                        let expanded_did_proof_json: Value = serde_json::from_str(&expanded_did_proof_str).expect("Failed");
                        let cannonized_did_proof = get_cannonized_str(expanded_did_proof_json.clone());

                        // Testing intermediate string
                        // let canon_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/canon_vc.txt";
                        // let canon_string = fs::read_to_string(canon_path).expect("Failed");
                      
                        // Test verification
                        let public_key = "z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp";
                        let m1 = hash_string(&cannonized_did.clone()); 

                        let m2 = hash_string(&cannonized_did_proof.clone()); // Using expanded did proof
                        let message = [m2.clone(), m1.clone()].concat();
                        
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
                }
                Err(e) => {
                    eprintln!("Failed to read or parse JSON-LD document: {}", e);
                }
            }
        }

        fn read_json_file(path: PathBuf) -> io::Result<String> {
            fs::read_to_string(path)
        }

        fn read_json_files_from_dir(dir_path: &str, prefix: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
            let mut documents = Vec::new();
            
            // Read all entries in the directory
            let entries = fs::read_dir(dir_path)?;
        
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
        
                // Only process JSON files with the specified prefix
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                        if file_name.starts_with(prefix) {
                            // Add document to the vector
                            documents.push(path);
                        }
                    }
                }
            }
        
            Ok(documents)
        }
    } 
}
