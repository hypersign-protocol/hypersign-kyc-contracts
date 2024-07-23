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
        // Initialiing NFT contract
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::RegisterDID {
                did: did.to_string(),
                did_doc: did_doc_string.to_owned(),
                did_doc_proof: did_doc_proof_string.to_owned(),
            },
            &[],
        )
        .unwrap();

        // // then test is counter has been incremented
        let resp: ResolveDIDResp = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::ResolveDID {
                    did: did.to_string(),
                },
            )
            .unwrap();

        assert_eq!(
            resp,
            ResolveDIDResp {
                did_doc: did_doc_string.to_string()
            }
        );

        // check the did verification status
        let resp3: GetDIDVerStatusResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetDIDVerStatus {})
            .unwrap();

        assert_eq!(resp3, GetDIDVerStatusResp { status: true });

        // let m = "40ea48e7bfde895182f57845da0b6648de11a9f31203569d10936a3bba0b1b8f0df7abe82aef2eb7b86bb78897066dca754180a99edd692c66b6fc71d028d5f6";
        // let signature_str = "z4S8Zxko4KLtHEKGkJVSPCrK4PcchJTYmcx3gsgxq3YG8uYQ3DJfaVufTDgjozNV174mZEmmUiib6J917jirmRfnY";
        // let public_key_str = "z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B";

        let m = "300ca1bc6cda0ef58ce58f638afc759be35c39fb41ae8879687d9180e581b7201e4c6152326424ee226927ce572264fb05958df55156f8241cf2db3bc113bfb7";
        let public_key_str = "z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM";
        let signature_str = "z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa";
        let resp4: VerifyProofsResp = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::VerifySSIProof {
                    public_key_str: public_key_str.to_string(),
                    signature_str: signature_str.to_string(),
                    message: m.to_string(),
                },
            )
            .unwrap();

        assert_eq!(resp4, VerifyProofsResp { result: false });
    }

    // #[test]
    // fn donate(){
    //     // App simulates blockhain
    //     // let mut app = App::default();

    //     // Let's create a dummy account
    //     let sender = Addr::unchecked("sender");
    //     // More sophisticated way of simulating blockhain
    //     // need to put fund some tokens to this sender
    //     let initialBalance = 10000;
    //     let tokenDenom =  "uHID";
    //     let mut app = AppBuilder::new().build(|router, _api, storage| {
    //         router  // from router
    //         .bank // extract bank module
    //         .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account
    //         .unwrap()
    //     });

    //     // storing contract code on blockhain
    //     let contract_id = app.store_code(counting_contract());

    //     let contract_addr = app.instantiate_contract(
    //         contract_id,
    //         sender.clone(), // simulating a blockchain address
    //         &InstantiateMsg{
    //             counter: 0,
    //             minimal_donation: Some(coin(10, tokenDenom))
    //         },
    //         &[],
    //         "Funding contract",
    //         None,
    //     ).unwrap();

    //     // lets send some fund; which will also increase the coounter = 3
    //     let amount_to_be_sent_to_contract = 10;
    //     app.execute_contract(
    //         sender.clone(),
    //         contract_addr.clone(),
    //         &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
    //         .unwrap();

    //     // then test is counter has been incremented
    //     let resp: ValueResp = app
    //                 .wrap()
    //                 .query_wasm_smart(
    //                     contract_addr.clone(),
    //                     &QueryMsg::Value {  })
    //                     .unwrap();

    //     assert_eq!(resp, ValueResp {value: 1});

    //     // lets check the balane of the cotnract as well....
    //     assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
    //     // check if amount was deducted from sernder account or not
    //     assert_eq!(app.wrap().query_all_balances(sender).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom))
    // }

    // #[test]
    // fn withdraw(){
    //     // App simulates blockhain
    //     // let mut app = App::default();

    //     // Let's create a dummy account
    //     let sender = Addr::unchecked("sender");
    //     let sender2 = Addr::unchecked("sender"); // this guyshouuld not be able to withdraw funds from contract since he is not the owner
    //     // More sophisticated way of simulating blockhain
    //     // need to put fund some tokens to this sender
    //     let initialBalance = 10000;
    //     let tokenDenom =  "uHID";
    //     let mut app = AppBuilder::new().build(|router, _api, storage| {
    //         router  // from router
    //         .bank // extract bank module
    //         .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account
    //         .unwrap();

    //         router  // from router
    //         .bank // extract bank module
    //         .init_balance(storage, &sender2, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account
    //         .unwrap()
    //     });

    //     // storing contract code on blockhain
    //     let contract_id = app.store_code(counting_contract());

    //     let contract_addr = app.instantiate_contract(
    //         contract_id,
    //         sender.clone(), // simulating a blockchain address
    //         &InstantiateMsg{
    //             counter: 0,
    //             minimal_donation: Some(coin(10, tokenDenom))
    //         },
    //         &[],
    //         "Funding contract",
    //         None,
    //     ).unwrap();

    //     // lets send some fund; which will also increase the coounter = 3
    //     let amount_to_be_sent_to_contract = 10;
    //     app.execute_contract(
    //         sender.clone(),
    //         contract_addr.clone(),
    //         &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
    //         .unwrap();

    //     // then test is counter has been incremented
    //     let resp: ValueResp = app
    //                 .wrap()
    //                 .query_wasm_smart(
    //                     contract_addr.clone(),
    //                     &QueryMsg::Value {  })
    //                     .unwrap();

    //     assert_eq!(resp, ValueResp {value: 1});

    //     // lets check the balane of the cotnract as well....
    //     assert_eq!(app.wrap().query_all_balances(contract_addr.clone()).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
    //     // check if amount was deducted from sernder account or not
    //     assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom));

    //     // taking my funds back in my account
    //     app.execute_contract(
    //         sender.clone(),
    //         contract_addr.clone(),
    //         &ExecMsg::Withdraw {  },
    //         &[])
    //     .unwrap();

    //     // lets check the balane of the cotnract as well....
    //     assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), &[]);
    //     // check if amount was deducted from sernder account or not
    //     assert_eq!(app.wrap().query_all_balances(sender).unwrap(), coins(initialBalance, tokenDenom));
    // }

    // #[test]
    // fn unauthorize_withdraw(){
    //     // App simulates blockhain
    //     // let mut app = App::default();

    //     // Let's create a dummy account
    //     let sender = Addr::unchecked("sender");
    //     let sender2 = Addr::unchecked("sender"); // this guyshouuld not be able to withdraw funds from contract since he is not the owner
    //     // More sophisticated way of simulating blockhain
    //     // need to put fund some tokens to this sender
    //     let initialBalance = 10000;
    //     let tokenDenom =  "uHID";
    //     let mut app = AppBuilder::new().build(|router, _api, storage| {
    //         router  // from router
    //         .bank // extract bank module
    //         .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account
    //         .unwrap();

    //         router  // from router
    //         .bank // extract bank module
    //         .init_balance(storage, &sender2, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account
    //         .unwrap()
    //     });

    //     // storing contract code on blockhain
    //     let contract_id = app.store_code(counting_contract());

    //     let contract_addr = app.instantiate_contract(
    //         contract_id,
    //         sender.clone(), // simulating a blockchain address
    //         &InstantiateMsg{
    //             counter: 0,
    //             minimal_donation: Some(coin(10, tokenDenom))
    //         },
    //         &[],
    //         "Funding contract",
    //         None,
    //     ).unwrap();

    //     // lets send some fund; which will also increase the coounter = 3
    //     let amount_to_be_sent_to_contract = 10;
    //     app.execute_contract(
    //         sender.clone(),
    //         contract_addr.clone(),
    //         &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
    //         .unwrap();

    //     // then test is counter has been incremented
    //     let resp: ValueResp = app
    //                 .wrap()
    //                 .query_wasm_smart(
    //                     contract_addr.clone(),
    //                     &QueryMsg::Value {  })
    //                     .unwrap();

    //     assert_eq!(resp, ValueResp {value: 1});

    //     // lets check the balane of the cotnract as well....
    //     assert_eq!(app.wrap().query_all_balances(contract_addr.clone()).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
    //     // check if amount was deducted from sernder account or not
    //     assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom));

    //     // this should fail becuase of unauthorized withdrawal from sender2
    //     //     let err = app
    //     //     .execute_contract(sender2, contract_addr.clone(), &ExecMsg::Withdraw {}, &[])
    //     //     .unwrap_err();

    //     // println!("err = {:?}", err);

    //     //    assert_eq!(
    //     //         ContractError::Unauthorized {
    //     //             owner: sender.into()
    //     //         },
    //     //         err.downcast().unwrap()
    //     //     );
    // }
}
