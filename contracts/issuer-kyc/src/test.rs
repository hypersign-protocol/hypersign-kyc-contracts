#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::error::KycContractError;
    use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, SBTcontractAddressResp, ValueResp};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw721;
    use cw721::msg::{NumTokensResponse, OwnerOfResponse};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use hypersign_zk_verifier::msg::{
        HsZkProof, HsZkProtocols, HsZkProtocolsCurvs, HypersignKYCProof, HypersignKYCProofTypes,
        ZkProof,
    };
    use serde_json::{from_slice, from_str, Value};
    use std::fs;
    fn issuer_kyc_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(entry::reply);
        Box::new(contract)
    }

    fn cw_721_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            hypersign_kyc_token::entry::execute,
            hypersign_kyc_token::entry::instantiate,
            hypersign_kyc_token::entry::query,
        );
        Box::new(contract)
    }

    fn get_did_key_materials() -> (Value, Value, String) {
        // Read the expanded did
        let expanded_did = "../../packages/ssi-manager/test/mock/expanded_did_doc.json";
        let expanded_did_str: Value =
            from_str(&fs::read_to_string(expanded_did).unwrap()).expect("Failed");

        // Read the expanded did proof
        let expanded_did_proof = "../../packages/ssi-manager/test/mock/expanded_did_proof.json";
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
                label: None,
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
                label: None,
            },
            &[],
        )
        .unwrap();

        // // then test is counter has been incremented
        let sbt_contract_address_resp: SBTcontractAddressResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::SBTContractAddress {})
            .unwrap();

        assert_eq!(
            sbt_contract_address_resp,
            SBTcontractAddressResp {
                sbt_contract_address: "contract1".to_string()
            }
        );

        let hs_zk_proof = HsZkProof {
            pi_a: vec![
                44, 164, 158, 79, 180, 145, 176, 92, 162, 135, 214, 154, 127, 173, 114, 7, 62, 1,
                39, 18, 109, 221, 202, 67, 158, 65, 118, 67, 41, 212, 103, 53, 32, 5, 81, 63, 8,
                129, 46, 65, 62, 87, 144, 111, 86, 83, 201, 101, 208, 161, 66, 114, 13, 78, 52,
                127, 192, 168, 127, 81, 93, 72, 219, 45,
            ],
            pi_b: vec![
                20, 184, 247, 24, 21, 2, 16, 191, 122, 248, 193, 223, 129, 148, 118, 1, 52, 116,
                53, 12, 20, 213, 141, 1, 80, 23, 39, 153, 186, 210, 109, 104, 1, 46, 120, 194, 54,
                176, 183, 171, 5, 161, 58, 64, 105, 175, 251, 153, 25, 236, 189, 103, 159, 243, 57,
                255, 135, 207, 94, 212, 194, 133, 80, 105, 18, 215, 182, 156, 78, 246, 101, 185,
                92, 222, 206, 65, 10, 92, 221, 158, 255, 33, 159, 111, 219, 103, 112, 48, 69, 242,
                255, 98, 218, 85, 183, 218, 34, 36, 51, 199, 54, 75, 5, 178, 115, 146, 103, 236,
                161, 9, 217, 254, 3, 201, 157, 96, 2, 204, 17, 155, 90, 220, 145, 20, 143, 190, 50,
                131,
            ],
            pi_c: vec![
                43, 78, 45, 68, 210, 245, 106, 181, 68, 92, 95, 196, 5, 103, 85, 161, 171, 45, 189,
                143, 146, 51, 163, 149, 254, 0, 145, 123, 242, 81, 248, 73, 43, 52, 113, 154, 24,
                111, 93, 122, 123, 170, 253, 156, 212, 93, 81, 123, 33, 117, 180, 38, 130, 169, 97,
                33, 74, 146, 229, 233, 167, 28, 202, 123,
            ],
            protocol: HsZkProtocols::groth16,
            curve: HsZkProtocolsCurvs::bn128,
        };

        let zk_proof = ZkProof {
            proof: hs_zk_proof,
            public_signales: vec![
                "139262560256827982113042929037067461531998672303572717191237106170700511945"
                    .to_string(),
                "1".to_string(),
                "18".to_string(),
                "11370393776179332609488947571879226318156480814724305073726489837302371244311"
                    .to_string(),
                "3502129987681126598706754762542340737175834041097740797030651868926291943299"
                    .to_string(),
                "16689638488897210389721526189894955938148630429690598708199340667708642425048"
                    .to_string(),
            ],
            proof_type: HypersignKYCProofTypes::zkProofOfAge,
        };

        // TODO: asset that a token was minited
        // Minitnig NFT contract
        let hypersign_proof = HypersignKYCProof {
            credential_id: Some("123123".to_string()),
            zk_proof: zk_proof,
        };
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Mint {
                hypersign_proof: hypersign_proof.clone(),
            },
            &[],
        )
        .unwrap();

        //// this should give error since proof is nullified
        // app.execute_contract(
        //     sender.clone(),
        //     contract_addr.clone(),
        //     &ExecMsg::Mint {
        //         hypersign_proof: hypersign_proof.clone(),
        //     },
        //     &[],
        // )
        // .unwrap();

        let resp: NumTokensResponse = app
            .wrap()
            .query_wasm_smart(
                "contract1".clone(),
                &hypersign_kyc_token::msg::QueryMsg::NumTokens {},
            )
            .unwrap();

        assert_eq!(resp, NumTokensResponse { count: 1 });

        let resp: OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                "contract1".clone(),
                &hypersign_kyc_token::msg::QueryMsg::OwnerOf {
                    token_id: "1".to_string(),
                    include_expired: Some(true),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            OwnerOfResponse {
                owner: "user".to_string(),
                approvals: [].to_vec(),
            }
        );

        // lets try to transfer to user2 and see if it fails...it should fail since transfer is blocked
        // let user2 = Addr::unchecked("user2");
        // let sbtcontractaddr =
        //     Addr::unchecked(sbt_contract_address_resp.sbt_contract_address.clone());
        // let transfer_resp = app
        //     .execute_contract(
        //         sender.clone(),
        //         sbtcontractaddr,
        //         &hypersign_kyc_token::msg::ExecuteMsg::TransferNft {
        //             recipient: user2.to_string(),
        //             token_id: "1".to_string(),
        //         },
        //         &[],
        //     )
        //     .unwrap();
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
