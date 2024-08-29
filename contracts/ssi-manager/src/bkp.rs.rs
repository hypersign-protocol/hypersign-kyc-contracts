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
            "proofPurpose":"assertionMethod",
            "proofValue":"z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa"
            }

        "#;

        let canon_did_doc_proof_string = r#"_:c14n0 <http://purl.org/dc/terms/created> "2010-01-01T19:23:24Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
_:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> .
_:c14n0 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#assertionMethod> .
_:c14n0 <https://w3id.org/security#proofValue> "z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa"^^<https://w3id.org/security#multibase> .
_:c14n0 <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1> .
"#;

        
        use serde::{Deserialize, Serialize};
        use serde_json::{self, Value};
        use std::collections::HashMap;
        use std::fs::File;
        use std::io::Read;
        use std::fs;
        use std::error::Error;
        use std::path::PathBuf;
        use std::io;
        use json_ld::{JsonLdOptions, JsonLdProcessor, rdf::RdfDataset, RemoteDocument, Document};
        use json_ld::contexts::Loader as ContextLoader;
        use json_ld::remote_document::SimpleDocument;
        use json_ld::rdf::RdfSerializer;
        use rio_api::model::{Triple, NamedNode};
        use rio_turtle::TurtleFormatter;
        use std::collections::HashMap;
        use std::sync::Arc;
        use std::io::Cursor;
        
        #[derive(Debug, Serialize, Deserialize)]
        struct JsonLdDocument {
            #[serde(rename = "@context")]
            context: Value, // Use Value to handle both array and object contexts
            #[serde(rename = "type")]
            type_: String,
            created: String,
            verificationMethod: String,
            proofPurpose: String,
            proofValue: Option<String>,
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Triple {
            subject: String,
            predicate: String,
            object: String,
        }

        // Define the VerificationMethod struct
        #[derive(Debug, Serialize, Deserialize)]
        struct VerificationMethod {
            id: String,
            #[serde(rename = "type")]
            type_: String,
            controller: String,
            publicKeyMultibase: Option<String>, // This field may not be present
        }

        // Define the JsonLdDocument struct
        #[derive(Debug, Serialize, Deserialize)]
        struct JsonLdDocumentDID {
            #[serde(rename = "@context")]
            context: Value, // Handle both array and object contexts
            id: String,
            controller: Vec<String>,
            alsoKnownAs: Vec<String>,
            verificationMethod: Vec<VerificationMethod>,
            authentication: Vec<String>,
            assertionMethod: Vec<String>,
            keyAgreement: Vec<String>,
            capabilityInvocation: Vec<String>,
            capabilityDelegation: Vec<String>,
            service: Vec<String>,
        }
        struct LocalContextLoader {
            contexts: HashMap<String, RemoteDocument>,
        }

        impl LocalContextLoader {
            fn new() -> Self {
                let mut contexts = HashMap::new();
                
                // Preload the DID and Ed25519 contexts
                contexts.insert(
                    "https://www.w3.org/ns/did/v1".to_string(),
                    SimpleDocument::new_from_str(include_str!("did-context.jsonld")),
                );
                contexts.insert(
                    "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
                    SimpleDocument::new_from_str(include_str!("ed25519-context.jsonld")),
                );
                
                LocalContextLoader { contexts }
            }
        }

        impl ContextLoader for LocalContextLoader {
            fn load_context(
                &self,
                iri: &str,
            ) -> json_ld::Result<Option<RemoteDocument>> {
                Ok(self.contexts.get(iri).cloned())
            }
        }

        fn jsonld_to_rdf(jsonld_data: &str) -> Result<String, Box<dyn std::error::Error>> {
            let jsonld: json_ld::json::JsonValue = serde_json::from_str(jsonld_data)?;
        
            let context_loader = Arc::new(LocalContextLoader::new());
            let options = JsonLdOptions::default();
            let mut dataset = RdfDataset::new();
        
            jsonld.expand_with_context(None, &context_loader, &options)?
                .into_rdf(&mut dataset, &options)?;
        
            let mut output = Vec::new();
            let mut formatter = TurtleFormatter::new(Cursor::new(&mut output));
        
            for quad in dataset.iter_quads() {
                let triple = Triple {
                    subject: quad.subject.clone().unwrap(),
                    predicate: NamedNode { iri: quad.predicate.iri() },
                    object: quad.object.clone().unwrap(),
                };
                formatter.format(&triple)?;
            }
        
            let turtle_string = String::from_utf8(output)?;
            Ok(turtle_string)
        }

        
        fn load_test_context(context_paths: &[&str]) -> HashMap<String, String> {
            let mut context_map = HashMap::new();

            for path in context_paths {
                // Read the context file
                let mut file = File::open(path).expect("Failed to open context file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read context file");

                // Parse the context file as JSON
                let context_value: Value = serde_json::from_str(&contents).expect("Failed to parse context JSON");
                
                // Handle both object and array forms of @context
                if let Value::Array(context_array) = context_value {
                    for item in context_array {
                        if let Value::Object(context_object) = item {
                            for (key, value) in context_object {
                                if let Value::String(uri) = value {
                                    context_map.insert(key.clone(), uri.clone());
                                }
                            }
                        }
                    }
                } else if let Value::Object(context_object) = context_value {
                    for (key, value) in context_object {
                        if let Value::String(uri) = value {
                            context_map.insert(key.clone(), uri.clone());
                        }
                    }
                }
            }

            context_map
        }

        fn load_local_context(context_paths: &[&str]) -> HashMap<String, String> {
            let mut context_map = HashMap::new();

            for path in context_paths {
                // Read the context file
                let mut file = File::open(path).expect("Failed to open context file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read context file");

                // Parse the context file as JSON
                let context_value: Value = serde_json::from_str(&contents).expect("Failed to parse context JSON");
                
                // Handle both object and array forms of @context
                if let Value::Array(context_array) = context_value {
                    for item in context_array {
                        if let Value::Object(context_object) = item {
                            for (key, value) in context_object {
                                if let Value::String(uri) = value {
                                    context_map.insert(key.clone(), uri.clone());
                                }
                            }
                        }
                    }
                } else if let Value::Object(context_object) = context_value {
                    for (key, value) in context_object {
                        println!("{:?} {}", key, value);
                        if let Value::String(uri) = value {
                            context_map.insert(key.clone(), uri.clone());
                        }
                    }
                }
            }

            context_map
        }

        fn resolve_term(term: &str, context_map: &HashMap<String, String>) -> String {
            context_map.get(term).cloned().unwrap_or_else(|| term.to_string())
        }

        // fn json_ld_to_triples(doc: &JsonLdDocumentDID, context_map: &HashMap<String, String>) -> Vec<Triple> {
        //     let mut triples = Vec::new();

        //     // Blank node identifier
        //     let blank_node = "_:c14n0";

        //     // Translate terms using context
        //     let type_uri = resolve_term(&doc.type_, context_map);
        //     let created_uri = "http://purl.org/dc/terms/created";
        //     let created_value = format!("\"{}\"^^<http://www.w3.org/2001/XMLSchema#dateTime>", doc.created);
        //     let verification_method_uri = resolve_term(&doc.verificationMethod, context_map);
        //     let proof_purpose_uri = resolve_term(&doc.proofPurpose, context_map);

        //     // Output the context map
        //     for (term, uri) in context_map {
        //         println!("Term: {}, URI: {}", term, uri);
        //     }
            
        //     // Construct RDF triples
        //     triples.push(Triple {
        //         subject: blank_node.to_string(),
        //         predicate: "http://purl.org/dc/terms/created".to_string(),
        //         object: format!(
        //             "{}",
        //             created_value
        //         ),
        //     });
        
        //     triples.push(Triple {
        //         subject: blank_node.to_string(),
        //         predicate: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string(),
        //         object:  format!(
        //             "{}",
        //             type_uri.to_string()
        //         ),
        //     });
        
        //     triples.push(Triple {
        //         subject: blank_node.to_string(),
        //         predicate: "https://w3id.org/security#proofPurpose".to_string(),
        //         object: proof_purpose_uri.to_string(),
        //     });
        
        //     // triples.push(Triple {
        //     //         subject: blank_node.to_string(),
        //     //         predicate: "https://w3id.org/security#proofValue".to_string(),
        //     //         object: format!(
        //     //             "{}",
        //     //             proof_purpose_uri.to_string()
        //     //         ),
        //     //     });
        
        //     triples.push(Triple {
        //         subject: blank_node.to_string(),
        //         predicate: "https://w3id.org/security#verificationMethod".to_string(),
        //         object: format!("<{}>", doc.verificationMethod),
        //     });
            
        //     triples
        // }

        fn canonize_rdf(triples: &mut Vec<Triple>) -> String {
            triples.sort();
            triples.iter()
                .map(|t| {
                    let predicate =  &t.predicate;
                    format!("{} <{}> {} .", t.subject, predicate, t.object)
                })
                .collect::<Vec<_>>()
                .join("\n")
        }

        fn read_json_file(path: PathBuf) -> io::Result<String> {
            fs::read_to_string(path)
        }
       

        let dir_path = "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test";
        // Read all JSON files in the directory and handle errors
        let documents = match read_json_files_from_dir(dir_path, "test") {
            Ok(docs) => docs,
            Err(e) => {
                eprintln!("Error reading JSON files: {}", e);
                return; // Exit early on error
            }
        };

        
        // Print the deserialized documents
        // Define paths to local context files
        let context_paths = [
            "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context.json", 
            // "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context1.json", 
            // "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context2.json",
        ];

        // Load and parse local contexts
        let context_map = load_test_context(&context_paths);

        for (i, doc) in documents.iter().enumerate() {

            // let file_contents = read_json_file(doc.to_path_buf()).expect("Failed to read file");
            // let efile_path =  format!("{}/expected_{}", dir_path, doc.file_name().unwrap().to_string_lossy());
            // let expected_str = fs::read_to_string(efile_path).unwrap();

            // let document: JsonLdDocumentDID = serde_json::from_str(&file_contents).unwrap();
            // // let mut triples = json_ld_to_triples(&document, &context_map);
            // let canonical_form = canonize_rdf(&mut triples);

            // println!("Canonical RDF:\n{}", canonical_form);

            // assert_eq!(
            //     canonical_form, expected_str
            // );

            // Read and parse the JSON file
            match read_json_file(doc) {
                Ok(document) => {
                    println!("Successfully read and parsed JSON-LD document: {:?}", document);
                }
                Err(e) => {
                    eprintln!("Failed to read or parse JSON-LD document: {}", e);
                }
            }
        }



        fn read_json_files_from_dir(dir_path: &str, prefix: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
            let mut documents = Vec::new();
            
            // Read all entries in the directory
            let entries = fs::read_dir(dir_path)?;
        
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
        
                // Only process JSON files with the specified prefix
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                        if file_name.starts_with(prefix) {
                            // Add document to the vector
                            documents.push(path);
                        }
                    }
                }
            }
        
            Ok(documents)
        }
        // // Initialiing NFT contract
        // app.execute_contract(
        //     sender.clone(),
        //     contract_addr.clone(),
        //     &ExecMsg::RegisterDID {
        //         did: did.to_string(),
        //         did_doc: did_doc_string.to_owned(),
        //         did_doc_proof: canon_did_doc_proof_string.to_owned(),
        //     },
        //     &[],
        // )
        // .unwrap();

        // // // then test is counter has been incremented
        // let resp: ResolveDIDResp = app
        //     .wrap()
        //     .query_wasm_smart(
        //         contract_addr.clone(),
        //         &QueryMsg::ResolveDID {
        //             did: did.to_string(),
        //         },
        //     )
        //     .unwrap();

        // assert_eq!(
        //     resp,
        //     ResolveDIDResp {
        //         did_doc: did_doc_string.to_string()
        //     }
        // );

        // // check the did verification status
        // let resp3: GetDIDVerStatusResp = app
        //     .wrap()
        //     .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetDIDVerStatus {})
        //     .unwrap();

        // assert_eq!(resp3, GetDIDVerStatusResp { status: true });

        // // let m = "40ea48e7bfde895182f57845da0b6648de11a9f31203569d10936a3bba0b1b8f0df7abe82aef2eb7b86bb78897066dca754180a99edd692c66b6fc71d028d5f6";
        // // let signature_str = "z4S8Zxko4KLtHEKGkJVSPCrK4PcchJTYmcx3gsgxq3YG8uYQ3DJfaVufTDgjozNV174mZEmmUiib6J917jirmRfnY";
        // // let public_key_str = "z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B";

        // let m = "300ca1bc6cda0ef58ce58f638afc759be35c39fb41ae8879687d9180e581b7201e4c6152326424ee226927ce572264fb05958df55156f8241cf2db3bc113bfb7";
        // let signature_str = "z326jXtLJDnzL7LtmQbRXCKjWNUxbUZvrJdpGh1JztYgxec6LJ5Dt2RwzyNKJkiCEneDPkDTTee6wsx6usZ9zQWSa";
        // let public_key_str = "z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM";

        // // Convert the above did to hash to get the strings
        
        // let resp4: VerifyProofsResp = app
        //     .wrap()
        //     .query_wasm_smart(
        //         contract_addr.clone(),
        //         &QueryMsg::VerifySSIProof {
        //             public_key_str: public_key_str.to_string(),
        //             signature_str: signature_str.to_string(),
        //             message: m.to_string(),
        //         },
        //     )
        //     .unwrap();

        // assert_eq!(resp4, VerifyProofsResp { result: true });
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
