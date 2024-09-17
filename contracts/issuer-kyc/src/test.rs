#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::error::KycContractError;
    use crate::msg::{
        ExecMsg, HypersignKYCProof, HypersignKYCProofTypes, InstantiateMsg, QueryMsg,
        SBTcontractAddressResp, ValueResp,
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

        // TODO: asset that a token was minited
        // Minitnig NFT contract
        let hypersign_proof = HypersignKYCProof {
            proof_type: HypersignKYCProofTypes::ProofOfKYC,
            description: "Proves that user has finished his/her KYC".to_string(),
            sbt_code: "T2".to_string(),
            credential_id: None,
            data: None,
            proof_type_image: Some("".to_string()),
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
}
