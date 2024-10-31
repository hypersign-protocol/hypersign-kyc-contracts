#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::error::KycContractError;
    use crate::msg::{
        ExecMsg, HsZkProof, HsZkProtocols, HsZkProtocolsCurvs, HypersignKYCProof,
        HypersignKYCProofTypes, InstantiateMsg, QueryMsg, SBTcontractAddressResp, ValueResp,
        ZkProof,
    };
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw721;
    use cw721::msg::{NumTokensResponse, OwnerOfResponse};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
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

        let hs_zk_proof = HsZkProof {
            pi_a: vec![
                13, 217, 14, 229, 255, 80, 6, 7, 88, 98, 9, 220, 185, 162, 141, 90, 135, 140, 101,
                191, 29, 89, 82, 117, 68, 245, 117, 102, 144, 77, 171, 104, 22, 107, 200, 36, 138,
                164, 0, 251, 109, 167, 129, 143, 154, 34, 120, 23, 20, 118, 12, 12, 182, 201, 137,
                168, 202, 199, 159, 75, 54, 253, 30, 225,
            ],
            pi_b: vec![
                23, 175, 152, 25, 244, 84, 161, 42, 208, 177, 72, 224, 76, 175, 243, 168, 173, 76,
                69, 248, 62, 126, 144, 139, 82, 2, 153, 70, 109, 41, 201, 204, 6, 243, 136, 40,
                148, 84, 203, 195, 106, 7, 137, 71, 241, 120, 40, 146, 199, 143, 93, 13, 200, 229,
                37, 225, 29, 163, 140, 227, 178, 7, 220, 154, 42, 109, 234, 103, 35, 233, 166, 127,
                143, 131, 100, 160, 109, 33, 74, 154, 138, 200, 210, 131, 56, 206, 18, 120, 56,
                123, 51, 30, 136, 200, 225, 80, 23, 11, 84, 7, 107, 86, 4, 60, 128, 15, 229, 137,
                22, 206, 69, 99, 54, 63, 160, 235, 176, 67, 0, 195, 33, 202, 243, 132, 248, 47,
                251, 222,
            ],
            pi_c: vec![
                1, 196, 237, 169, 186, 214, 135, 209, 184, 3, 43, 101, 139, 78, 230, 249, 220, 53,
                232, 194, 195, 12, 69, 137, 242, 185, 228, 202, 225, 176, 126, 245, 44, 60, 205,
                29, 193, 59, 43, 34, 163, 215, 50, 217, 217, 9, 47, 108, 25, 201, 73, 217, 54, 0,
                100, 90, 179, 220, 20, 61, 14, 166, 44, 45,
            ],
            protocol: HsZkProtocols::groth16,
            curve: HsZkProtocolsCurvs::bn128,
        };

        let zk_proof = ZkProof {
            proof: hs_zk_proof,
            public_signales: vec![
                "1".to_string(),
                "12040884699199694350430421040574883160903448743611754661868587601688521091572"
                    .to_string(),
                "17402076351219241481156702044498624695626933249600419891544118840897244553492"
                    .to_string(),
                "10407708521482612808325680154257593139024275871119660778918312440159182133587"
                    .to_string(),
                "5198474275750008277983235026401485600136802015956123361274085143332394226688"
                    .to_string(),
                "18".to_string(),
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
            &ExecMsg::Mint { hypersign_proof },
            &[],
        )
        .unwrap();

        let resp: NumTokensResponse = app
            .wrap()
            .query_wasm_smart(
                "contract1".clone(),
                &cw721_metadata_onchain::QueryMsg::NumTokens {},
            )
            .unwrap();

        assert_eq!(resp, NumTokensResponse { count: 1 });

        let resp: OwnerOfResponse = app
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
            OwnerOfResponse {
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
