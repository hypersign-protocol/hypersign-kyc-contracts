#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::error::KycContractError;
    use crate::msg::{
        ExecMsg, HypersignKYCProof, HypersignKYCProofTypes, InstantiateMsg, QueryMsg,
        SBTcontractAddressResp, ValueResp,
    };
    use crate::state::COUNTER;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw721_base::Cw721Contract;
    // use cw721_metadata_onchain::QueryMsg;
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

        let kyc_contract_code_id = app.store_code(issuer_kyc_contract());
        println!("kyc_contract_code_id = {:?}", kyc_contract_code_id);

        // Read the expanded did
        let expanded_did = "../ssi-manager/test/mock/expanded_did_doc.json";
        let expanded_did_str: Value =
            from_str(&fs::read_to_string(expanded_did).unwrap()).expect("Failed");

        // Read the expanded did proof
        let expanded_did_proof = "../ssi-manager/test/mock/expanded_did_proof.json";
        let expanded_did_proof_str: Value =
            from_str(&fs::read_to_string(expanded_did_proof).unwrap()).expect("Failed");

        let signature = "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq";

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

        // check if user own that nft or not
        // let qresp2: cw721::NftInfoResponse<Q> = app
        //     .wrap()
        //     .query_wasm_smart(
        //         "contract1".clone(),
        //         &cw721_metadata_onchain::QueryMsg::NftInfo {
        //             token_id: ("1".to_string()),
        //         },
        //     )
        //     .unwrap();

        // assert_eq!(
        //     qresp2,
        //     cw721::NftInfoResponse {
        //         token_uri: Some("abc".to_string()),
        //         extension: None
        //     }
        // );

        // TODO: assert taht token was transfered to the user
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
