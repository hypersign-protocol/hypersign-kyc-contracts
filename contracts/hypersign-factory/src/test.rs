#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::*;
    use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp, ValueRespProxy};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        // App simulates blockhain
        let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");
        // More sophisticated way of simulating blockhain
        // need to put fund some tokens to this sender
        let initialBalance = 10000;
        let tokenDenom = "uHID";
        // let mut app = AppBuilder::new().build(|router, _api, storage| {
        //     router  // from router
        //     .bank // extract bank module
        //     .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account
        //     .unwrap()
        // });

        // storing contract code on blockhain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(), // simulating a blockchain address
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: Some(coin(10, tokenDenom)),
                },
                &[],
                "Funding contract",
                None,
            )
            .unwrap();

        // // lets first increment the counter = 1
        let proxy_contract_addr = "abcd".to_owned();
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Poke {
                proxy_contract_addr: proxy_contract_addr.clone(),
            },
            &[],
        )
        .unwrap();

        // // lets first increment the counter one more time = 2
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Poke {
                proxy_contract_addr: proxy_contract_addr.clone(),
            },
            &[],
        )
        .unwrap();

        // lets send some fund; which will also increase the coounter = 3
        // let amount_to_be_sent_to_contract = 10;
        // app.execute_contract(
        //     sender.clone(),
        //     contract_addr.clone(),
        //     &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
        //     .unwrap();

        // then test is counter has been incremented
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 2 });

        // test if proxy contract address is set
        let resp1: ValueRespProxy = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetProxyMessage {})
            .unwrap();
        assert_eq!(
            resp1,
            ValueRespProxy {
                proxyContractAddress: proxy_contract_addr.clone()
            }
        );

        // // lets check the balane of the cotnract as well....
        // assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
        // // check if amount was deducted from sernder account or not
        // assert_eq!(app.wrap().query_all_balances(sender).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom))
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
