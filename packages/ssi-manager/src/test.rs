#[cfg(any(test, feature = "tests"))]
pub mod test {

    use crate::entry::{self, *};

    use crate::msg::{
        ExecMsg, GetDIDVerStatusResp, InstantiateMsg, QueryMsg, ResolveDIDResp, ValueResp,
        VerifyProofsResp,
    };

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Addr, Attribute, Empty, Response};
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

    // Test Proof verification
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

        use crate::ed25519_signature_2020;
        use crate::lib_json_ld::{get_cannonized_str, get_did_value, hash_string};

        use cosmwasm_std::{from_binary, Attribute, Binary, Response, StdResult};
        use cw721_base::ExecuteMsg;
        use serde_json::{from_slice, from_str, Value};
        use std::error::Error;
        use std::fs;
        use std::io;

        let did_doc_string_raw = r#"
        [
            {
              "https://www.w3.org/ns/activitystreams#alsoKnownAs": [
                {
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                }
              ],
              "https://w3id.org/security#assertionMethod": [
                {
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1"
                }
              ],
              "https://w3id.org/security#authenticationMethod": [
                {
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1"
                }
              ],
              "https://w3id.org/security#capabilityDelegationMethod": [
                {
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1"
                }
              ],
              "https://w3id.org/security#capabilityInvocationMethod": [
                {
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1"
                }
              ],
              "https://w3id.org/security#controller": [
                {
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                }
              ],
              "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp",
              "https://w3id.org/security#keyAgreementMethod": [],
              "https://www.w3.org/ns/did#service": [],
              "https://w3id.org/security#verificationMethod": [
                {
                  "https://w3id.org/security#controller": [
                    {
                      "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                    }
                  ],
                  "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1",
                  "https://w3id.org/security#publicKeyMultibase": [
                    {
                      "@type": "https://w3id.org/security#multibase",
                      "@value": "z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"
                    }
                  ],
                  "@type": [
                    "https://w3id.org/security#Ed25519VerificationKey2020"
                  ]
                }
              ]
            }
          ]
        "#;

        let did_doc_proof_string_raw = r#"
        [
            {
                "https://w3id.org/security#challenge": [
                {
                    "@value": "123123"
                }
                ],
                "http://purl.org/dc/terms/created": [
                {
                    "@type": "http://www.w3.org/2001/XMLSchema#dateTime",
                    "@value": "2024-09-01T17:44:11Z"
                }
                ],
                "https://w3id.org/security#domain": [
                {
                    "@value": "http:adsasd"
                }
                ],
                "https://w3id.org/security#proofPurpose": [
                {
                    "@id": "https://w3id.org/security#authenticationMethod"
                }
                ],
                "@type": [
                "https://w3id.org/security#Ed25519Signature2020"
                ],
                "https://w3id.org/security#verificationMethod": [
                    {
                        "@id": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1"
                    }
                ]
            }
        ]
        "#;
        let signature = "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq";

        let msg = &ExecMsg::RegisterDID {
            did_doc: String::from(did_doc_string_raw), //serde_json::to_string(&expanded_did_proof_str).unwrap(),
            did_doc_proof: String::from(did_doc_proof_string_raw), //serde_json::to_string(&expanded_did_str).unwrap(),
            signature: signature.to_string(),
        };
        let execute_resp = app
            .execute_contract(sender.clone(), contract_addr.clone(), msg, &[])
            .unwrap();

        // let t = execute_resp.events.join(' ');
        println!("t = {:?}", execute_resp.events.clone());

        // Resolve a DID
        let did = get_did_value(&serde_json::from_str(did_doc_string_raw).expect("reason.."));
        println!("did here = {:?}", did.clone());
        let qresp: ResolveDIDResp = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::ResolveDID {
                    did: did.to_string(),
                },
            )
            .unwrap();

        assert_eq!(
            qresp,
            ResolveDIDResp {
                did_doc: String::from(did_doc_string_raw)
            }
        );

        // let qresp2: ValueResp = app
        //     .wrap()
        //     .query_wasm_smart(contract_addr.clone(), &QueryMsg::OwnerDID {})
        //     .unwrap();

        // assert_eq!(
        //     qresp2,
        //     ValueResp {
        //         owner_did: "did:hid:123131231231".to_string()
        //     }
        // );
    }
}
