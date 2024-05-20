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
        let did = "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B";
        let did_doc_string = r#"
            {"@context":["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","controller":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"alsoKnownAs":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"verificationMethod":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"Ed25519VerificationKey2020","controller":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","publicKeyMultibase":"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}],"authentication":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"assertionMethod":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"keyAgreement":[],"capabilityInvocation":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"capabilityDelegation":[],"service":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"LinkedDomains","serviceEndpoint":"https://www.linkeddomains.com"}]}
            "#;
        let did_doc_proof_string = r#"
            {
            "@context": [
                "https://www.w3.org/ns/did/v1",
                "https://w3id.org/security/suites/ed25519-2020/v1"
            ],
            "type":"Ed25519Signature2020",
            "created":"2010-01-01T19:23:24Z",
            "verificationMethod":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1",
            "proofPurpose":"assertionMethod"
            }
        "#;

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

        app.execute_contract(
            sender.clone(),
            ssi_manager_contract_addr.clone(),
            &ssi_manager::msg::ExecMsg::RegisterDID {
                did: did.to_string(),
                did_doc: did_doc_string.to_owned(),
                did_doc_proof: did_doc_proof_string.to_owned(),
            },
            &[],
        )
        .unwrap();

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

        // Onboarding a user by deploying a contaract for him
        let mut issuer_did = did; // "did:hid:1234";
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::OnboardIssuer {
                issuer_did: issuer_did.into(),
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
