#[cfg(any(test, feature = "tests"))]
pub mod test {
    use super::*;
    use crate::entry::{self, *};
    use crate::msg::{
        ExecMsg, InstantiateMsg, Issuer, QueryMsg, RegistredIssuerResp, ValueResp, ValueRespProxy,
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

    #[test]
    fn onboard_issuer() {
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

        let contract_addr = app
            .instantiate_contract(
                hypersign_kyc_factory_contract_code_id,
                sender.clone(),
                &InstantiateMsg { counter: 0 },
                &[],
                "Hypersign kyc factory contract",
                None,
            )
            .unwrap();

        // Onboarding a user by deploying a contaract for him
        let mut issuer_did = "did:hid:1234";
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::OnboardIssuer {
                issuer_did: issuer_did.into(),
                issuer_kyc_code_id: kyc_contract_code_id,
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
                    kyc_contract_address: Some("contract1".to_string())
                }
            }
        );

        // re registert the same issuer should not work
        issuer_did = "did:hid:12344";
        let resp_fail = app
            .execute_contract(
                sender.clone(),
                contract_addr.clone(),
                &ExecMsg::OnboardIssuer {
                    issuer_did: issuer_did.into(),
                    issuer_kyc_code_id: kyc_contract_code_id,
                },
                &[],
            )
            .unwrap();
    }
}
