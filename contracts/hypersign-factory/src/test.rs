#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::msg::{
        ExecMsg, HypersignAdminDIDResp, InstantiateMsg, Issuer, QueryMsg, RegistredIssuerResp,
        SSIManagerContractAddressResp, ValueResp, ValueRespProxy,
    };
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    use serde_json::{from_slice, from_str, Value};
    use std::fs;

    fn hypersign_kyc_factory_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(entry::execute, entry::instantiate, entry::query)
            .with_reply(entry::reply);
        Box::new(contract)
    }

    fn issuer_kyc_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            issuer_kyc::entry::execute,
            issuer_kyc::entry::instantiate,
            issuer_kyc::entry::query,
        );
        //.with_reply(issuer_kyc::entry::reply);
        Box::new(contract)
    }

    fn ssi_manager_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            ssi_manager::entry::execute,
            ssi_manager::entry::instantiate,
            ssi_manager::entry::query,
        );
        Box::new(contract)
    }

    #[test]
    fn onboard_issuer() {
        // Register issuer did
        let did = "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp";
        // let did_doc_string = ""
        // let did_doc_proof_string = ""

        let expanded_did = "../ssi-manager/test/mock/expanded_did_doc.json";
        let did_doc_string: Value =
            from_str(&fs::read_to_string(expanded_did).unwrap()).expect("Failed");

        let expanded_did_proof = "../ssi-manager/test/mock/expanded_did_proof.json";
        let did_doc_proof_string: Value =
            from_str(&fs::read_to_string(expanded_did_proof).unwrap()).expect("Failed");

        // App simulates blockhain
        let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");

        // storing contract code on blockhain
        let hypersign_kyc_factory_contract_code_id =
            app.store_code(hypersign_kyc_factory_contract());
        println!(
            "hypersign_kyc_factory_contract_code_id = {:?}",
            hypersign_kyc_factory_contract_code_id
        );

        let kyc_contract_code_id = app.store_code(issuer_kyc_contract());
        println!("kyc_contract_code_id = {:?}", kyc_contract_code_id);

        let ssi_manager_contract_code_id = app.store_code(ssi_manager_contract());
        println!(
            "ssi_manager_contract_code_id = {:?}",
            ssi_manager_contract_code_id
        );

        let ssi_manager_contract_addr = app
            .instantiate_contract(
                ssi_manager_contract_code_id,
                sender.clone(), // simulating a blockchain address
                &ssi_manager::msg::InstantiateMsg {
                    owner_did: "did:hid:12313123123".to_string(),
                    did_method: "did:hid:testnet".to_string(),
                },
                &[],
                "SSI Maager contract",
                None,
            )
            .unwrap();

        println!(
            "ssi_manager_contract_addr = {:?}",
            ssi_manager_contract_addr.to_string()
        );

        //// Implement register_did({did, signed_did_doc})
        let signature = "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq";

        let msg = &ssi_manager::msg::ExecMsg::RegisterDID {
            did_doc: serde_json::to_string(&did_doc_string).unwrap(),
            did_doc_proof: serde_json::to_string(&did_doc_proof_string).unwrap(),
            signature: signature.to_string(),
        };
        // println!("msg = {:?}", msg.clone());
        app.execute_contract(sender.clone(), ssi_manager_contract_addr.clone(), msg, &[])
            .unwrap();

        // resolve this did
        println!("did = {:?}", did.to_string());
        let qresp2: ssi_manager::msg::ValueResp = app
            .wrap()
            .query_wasm_smart(
                ssi_manager_contract_addr.clone(),
                &ssi_manager::msg::QueryMsg::OwnerDID {},
            )
            .unwrap();
        // println!("qresp = {:?}", qresp.to_string());
        assert_eq!(
            qresp2,
            ssi_manager::msg::ValueResp {
                owner_did: "did:hid:12313123123".to_string()
            }
        );

        let qresp: ssi_manager::msg::ResolveDIDResp = app
            .wrap()
            .query_wasm_smart(
                ssi_manager_contract_addr.clone(),
                &ssi_manager::msg::QueryMsg::ResolveDID {
                    did: did.to_string(),
                },
            )
            .unwrap();

        assert_eq!(
            qresp,
            ssi_manager::msg::ResolveDIDResp {
                did_doc: did_doc_string.to_string()
            }
        );

        // ----------------------------------------------------------------

        //// Improve instantiation({SSI_manager_contract, hs_admin_did, hs_admin_did_doc, hs_admin_did_doc_proof}) of Hypersign_KYC_factory_Contract to whitelist SSI_manager_contract address and whitelist hypersign_did
        let contract_addr = app
            .instantiate_contract(
                hypersign_kyc_factory_contract_code_id,
                sender.clone(),
                &InstantiateMsg {
                    counter: 0,
                    hypersign_ssi_manager_contract_address: ssi_manager_contract_addr.to_string(),
                    kyc_contract_code_id: kyc_contract_code_id,
                    hypersign_admin_did: did.to_string(),
                },
                &[],
                "Hypersign kyc factory contract",
                None,
            )
            .unwrap();

        println!(
            "hypersign_factory_contract_addr = {:?}",
            contract_addr.to_string()
        );

        let resp_xyz: HypersignAdminDIDResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetHypersignAdminDID {})
            .unwrap();

        assert_eq!(
            resp_xyz,
            HypersignAdminDIDResp {
                did: did.to_string()
            }
        );

        // ----------------------------------------------------------------

        // Onboarding a user by deploying a contaract for him
        let mut issuer_did = did; // "did:hid:1234";
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::OnboardIssuer {
                did_doc: serde_json::to_string(&did_doc_string).unwrap(),
                did_doc_proof: serde_json::to_string(&did_doc_proof_string).unwrap(),
                signature: signature.to_string(),
            },
            &[],
        )
        .unwrap();

        // // then test is counter has been incremented
        let resp: RegistredIssuerResp = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::GetRegisteredIssuer {
                    issuer_did: issuer_did.into(),
                },
            )
            .unwrap();

        println!("resp {:?}", resp);
        assert_eq!(
            resp,
            RegistredIssuerResp {
                issuer: Issuer {
                    id: "issuer-1".into(),
                    did: issuer_did.clone().into(),
                    kyc_contract_address: Some("contract2".to_string()),
                    kyc_contract_code_id: kyc_contract_code_id
                }
            }
        );

        // re registert the same issuer should not work
        // issuer_did = "did:hid:12344";
        // let resp_fail = app
        //     .execute_contract(
        //         sender.clone(),
        //         contract_addr.clone(),
        //         &ExecMsg::OnboardIssuer {
        //             issuer_did: issuer_did.into(),
        //         },
        //         &[],
        //     )
        //     .unwrap();

        // ----------------------------------------------------------------

        let resp2: SSIManagerContractAddressResp = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::GetSSIManagerContractAddress {},
            )
            .unwrap();

        assert_eq!(
            resp2,
            SSIManagerContractAddressResp {
                contract_address: ssi_manager_contract_addr.to_string()
            }
        );
    }
}
