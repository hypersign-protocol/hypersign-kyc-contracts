#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::error::KycContractError;
    use crate::msg::{
        ExecMsg, HypersignKYCProof, HypersignKYCProofTypes, InstantiateMsg, QueryMsg,
        SBTcontractAddressResp, ValueResp, ZkProof,
    };
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw721;
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use serde_json::{from_slice, from_str, Value};
    use std::fs;
    fn issuer_kyc_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(entry::reply);
        Box::new(contract)
    }

    fn cw_721_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw721_base::entry::execute,
            cw721_base::entry::instantiate,
            cw721_base::entry::query,
        );
        Box::new(contract)
    }

    fn get_did_key_materials() -> (Value, Value, String) {
        // Read the expanded did
        let expanded_did = "../ssi-manager/test/mock/expanded_did_doc.json";
        let expanded_did_str: Value =
            from_str(&fs::read_to_string(expanded_did).unwrap()).expect("Failed");

        // Read the expanded did proof
        let expanded_did_proof = "../ssi-manager/test/mock/expanded_did_proof.json";
        let expanded_did_proof_str: Value =
            from_str(&fs::read_to_string(expanded_did_proof).unwrap()).expect("Failed");

        let signature = "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq".to_string();

        (expanded_did_str, expanded_did_proof_str, signature)
    }

    fn initalize_get_blockchain() -> (App, Addr, u64, u64) {
        // App simulates blockhain
        let mut app: App = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("user");

        // storing contract code on blockhain
        let sbt_contract_code_id = app.store_code(cw_721_contract());
        println!("sbt_contract_code_id = {:?}", sbt_contract_code_id);

        let kyc_contract_code_id = app.store_code(issuer_kyc_contract());
        println!("kyc_contract_code_id = {:?}", kyc_contract_code_id);
        (app, sender, sbt_contract_code_id, kyc_contract_code_id)
    }

    #[test]
    fn issuer_contract_instantiation() {
        let (my_app, sender, sbt_contract_code_id, kyc_contract_code_id) =
            initalize_get_blockchain();

        let mut app = my_app;

        let (expanded_did_str, expanded_did_proof_str, signature) = get_did_key_materials();

        let contract_addr = app
            .instantiate_contract(
                kyc_contract_code_id,
                sender.clone(), // simulating a blockchain address
                &InstantiateMsg {
                    did_doc: serde_json::to_string(&expanded_did_str).unwrap(),
                    did_doc_proof: serde_json::to_string(&expanded_did_proof_str).unwrap(),
                    signature: signature.to_string(),
                },
                &[],
                "Issuer contract",
                None,
            )
            .unwrap();

        // check if owner did properly set
        let qresp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::OwnerDID {})
            .unwrap();

        assert_eq!(
            qresp,
            ValueResp {
                owner_did: "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                    .to_string()
            }
        );
    }

    #[test]
    fn sbt_contract_instantiation() {
        let (my_app, sender, sbt_contract_code_id, kyc_contract_code_id) =
            initalize_get_blockchain();

        let mut app = my_app;

        let (expanded_did_str, expanded_did_proof_str, signature) = get_did_key_materials();

        let contract_addr = app
            .instantiate_contract(
                kyc_contract_code_id,
                sender.clone(), // simulating a blockchain address
                &InstantiateMsg {
                    did_doc: serde_json::to_string(&expanded_did_str).unwrap(),
                    did_doc_proof: serde_json::to_string(&expanded_did_proof_str).unwrap(),
                    signature: signature.to_string(),
                },
                &[],
                "Issuer contract",
                None,
            )
            .unwrap();

        // check if owner did properly set
        let qresp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::OwnerDID {})
            .unwrap();

        assert_eq!(
            qresp,
            ValueResp {
                owner_did: "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                    .to_string()
            }
        );

        // Initialiing NFT contract
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Init {
                token_code_id: sbt_contract_code_id,
            },
            &[],
        )
        .unwrap();

        // // then test is counter has been incremented
        let resp: SBTcontractAddressResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::SBTContractAddress {})
            .unwrap();

        assert_eq!(
            resp,
            SBTcontractAddressResp {
                sbt_contract_address: "contract1".to_string()
            }
        );
    }

    #[test]
    fn mint_sbt() {
        let (my_app, sender, sbt_contract_code_id, kyc_contract_code_id) =
            initalize_get_blockchain();

        let mut app = my_app;

        let (expanded_did_str, expanded_did_proof_str, signature) = get_did_key_materials();

        let contract_addr = app
            .instantiate_contract(
                kyc_contract_code_id,
                sender.clone(), // simulating a blockchain address
                &InstantiateMsg {
                    did_doc: serde_json::to_string(&expanded_did_str).unwrap(),
                    did_doc_proof: serde_json::to_string(&expanded_did_proof_str).unwrap(),
                    signature: signature.to_string(),
                },
                &[],
                "Issuer contract",
                None,
            )
            .unwrap();

        // check if owner did properly set
        let qresp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::OwnerDID {})
            .unwrap();

        assert_eq!(
            qresp,
            ValueResp {
                owner_did: "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                    .to_string()
            }
        );

        // Initialiing NFT contract
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Init {
                token_code_id: sbt_contract_code_id,
            },
            &[],
        )
        .unwrap();

        // // then test is counter has been incremented
        let resp: SBTcontractAddressResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::SBTContractAddress {})
            .unwrap();

        assert_eq!(
            resp,
            SBTcontractAddressResp {
                sbt_contract_address: "contract1".to_string()
            }
        );

        let zk_proof = ZkProof {
            proof: r#"{"pi_a":[40,92,159,239,135,71,131,180,248,147,169,222,232,97,48,105,217,165,250,185,7,60,74,90,135,68,142,168,205,253,76,96,15,217,143,188,100,205,0,41,220,68,189,168,247,105,81,239,21,251,38,244,193,42,110,83,49,160,238,190,131,198,159,67],"pi_b":[24,247,33,247,208,58,206,103,45,36,80,164,234,255,191,187,147,112,19,133,188,230,6,38,69,69,64,139,233,90,118,8,27,225,72,30,105,245,158,141,143,237,117,50,31,254,51,110,158,224,8,185,60,212,8,113,168,227,149,144,77,216,105,105,24,210,243,58,123,237,21,248,101,190,236,130,230,29,162,115,116,24,162,247,140,111,129,87,114,50,97,221,35,162,146,90,31,252,83,232,106,217,108,29,137,233,11,150,187,45,90,212,232,8,251,86,187,112,123,29,64,182,237,107,169,28,129,145],"pi_c":[7,227,99,82,182,142,207,181,216,239,108,223,37,105,149,62,227,167,64,136,119,23,180,153,245,38,38,254,54,10,71,99,48,64,56,8,200,111,39,153,41,97,2,11,48,230,70,149,245,40,15,48,29,74,92,191,234,202,117,80,119,168,252,2],"protocol":"groth16","curve":"bn128"}"#.to_string(),
            public_signales: vec![
                "1".to_string(),
                "18955587923911110975324593921788466916679894646588172021082202393332121293343"
                    .to_string(),
                "11370393776179332609488947571879226318156480814724305073726489837302371244311"
                    .to_string(),
                "3502129987681126598706754762542340737175834041097740797030651868926291943299"
                    .to_string(),
                "16689638488897210389721526189894955938148630429690598708199340667708642425048"
                    .to_string(),
                "18".to_string(),
            ],
            proof_type: HypersignKYCProofTypes::zkProofOfAge,
        };

        // TODO: asset that a token was minited
        // Minitnig NFT contract
        let hypersign_proof = HypersignKYCProof {
            // proof_type: HypersignKYCProofTypes::zkProofOfKYC,
            description: "Proves that user has finished his/her KYC".to_string(),
            sbt_code: "T2".to_string(),
            credential_id: None,
            data: None,
            proof_type_image: Some("".to_string()),
            zk_proof: zk_proof,
        };
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Mint { hypersign_proof },
            &[],
        )
        .unwrap();

        let resp: cw721::NumTokensResponse = app
            .wrap()
            .query_wasm_smart(
                "contract1".clone(),
                &cw721_metadata_onchain::QueryMsg::NumTokens {},
            )
            .unwrap();

        assert_eq!(resp, cw721::NumTokensResponse { count: 1 });

        let resp: cw721::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                "contract1".clone(),
                &cw721_metadata_onchain::QueryMsg::OwnerOf {
                    token_id: "1".to_string(),
                    include_expired: Some(true),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            cw721::OwnerOfResponse {
                owner: "user".to_string(),
                approvals: [].to_vec(),
            }
        );

        // let resp2: cw721_metadata_onchain::Metadata = app
        //     .wrap()
        //     .query_wasm_smart(
        //         "contract1".clone(),
        //         &cw721_metadata_onchain::QueryMsg::NftInfo {
        //             token_id: "1".to_string(),
        //         },
        //     )
        //     .unwrap();

        // assert_eq!(resp2.description, Some("description1".into()));

        // TODO: assert taht token was transfered to the user
    }

    // #[test]
    // fn verify_zk_proofs() {
    //     //crate::zkpverify::verify_zkp();

    //     match crate::zkpverify::verify_zkp() {
    //         Ok(result) => {
    //             assert_eq!(result, true);
    //         }
    //         Err(err) => {
    //             println!("{:?}", err);
    //         }
    //     }
    // }
}
