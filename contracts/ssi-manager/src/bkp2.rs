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
        use json_ld::{JsonLdProcessor, Options, RemoteDocument, syntax::{Value, Parse}};
        use json_ld::contexts::Loader as ContextLoader;
        use json_ld::remote_document::SimpleDocument;
        use json_ld::rdf::RdfSerializer;
        use rio_api::model::{Triple, NamedNode};
        use rio_turtle::TurtleFormatter;
        use std::collections::HashMap;
        use std::sync::Arc;
        use std::io::Cursor;
        use rio_api::{Graph, Serializer};
        use rio_turtle::TurtleSerializer;

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

                // Define paths to your local context files
                let did_context_path = Path::new("/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context1.json");
                let ed25519_context_path = Path::new("/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context2.json");
                
                // Read the context files into strings
                let did_context_str = fs::read_to_string(did_context_path)
                    .expect("Failed to read DID context file");
                let ed25519_context_str = fs::read_to_string(ed25519_context_path)
                    .expect("Failed to read Ed25519 context file");
                
                // Load contexts from the strings
                contexts.insert(
                    "https://www.w3.org/ns/did/v1".to_string(),
                    SimpleDocument::new_from_str(&did_context_str),
                );
                contexts.insert(
                    "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
                    SimpleDocument::new_from_str(&ed25519_context_str),
                );
        
                LocalContextLoader { contexts }
            }
        }

        impl ContextLoader for LocalContextLoader {
            fn load_context(
                &self,
                iri: &str,
            ) -> Result<Option<RemoteDocument>> {
                Ok(self.contexts.get(iri).cloned())
            }
        }


        fn json_ld_to_triples(json_ld: Value) -> Vec<Triple> {
            let mut triples = Vec::new();

            if let Some(objects) = json_ld.as_object() {
                for (subject, value) in objects {
                    // Simplified extraction logic, assumes predicate and object are directly available
                    if let Some(predicates) = value.as_object() {
                        for (predicate, obj) in predicates {
                            if let Some(object) = obj.as_str() {
                                triples.push(Triple {
                                    subject: subject.clone(),
                                    predicate: predicate.clone(),
                                    object: object.to_string(),
                                });
                            }
                        }
                    }
                }
            }

            triples
        }

        fn convert_to_rdf(json_ld: Value) -> Result<String, Box<dyn std::error::Error>> {
            // Assuming you have a method to convert JSON-LD to triples
            let triples = json_ld_to_triples(json_ld);
        
            let mut output = Vec::new();
            let mut serializer = TurtleSerializer::new(&mut output);
        
            for triple in triples {
                serializer.serialize_triple(&triple)?;
            }
        
            Ok(String::from_utf8(output)?)
        }

        // fn jsonld_to_rdf(jsonld_data: &str) -> Result<String, Box<dyn std::error::Error>> {
        //     let jsonld: Value = serde_json::from_str(jsonld_data)?;
        
        //     let context_loader = Arc::new(LocalContextLoader::new());
        //     let options = Options::default();
        //     let mut dataset = RdfDataset::new();
        
        //     jsonld.expand_with_context(None, &context_loader, &options)?
        //         .into_rdf(&mut dataset, &options)?;
        
        //     let mut output = Vec::new();
        //     let mut formatter = TurtleFormatter::new(Cursor::new(&mut output));
        
        //     for quad in dataset.iter_quads() {
        //         let triple = Triple {
        //             subject: quad.subject.clone().unwrap(),
        //             predicate: NamedNode { iri: quad.predicate.iri() },
        //             object: quad.object.clone().unwrap(),
        //         };
        //         formatter.format(&triple)?;
        //     }
        
        //     let turtle_string = String::from_utf8(output)?;
        //     Ok(turtle_string)
        // }

       
        // Define paths to local context files
        let context_paths = [
            "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context.json", 
        ];

        // Read all JSON files in the directory and handle errors
        let dir_path = "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test";
        let documents = match read_json_files_from_dir(dir_path, "test") {
            Ok(docs) => docs,
            Err(e) => {
                eprintln!("Error reading JSON files: {}", e);
                return; // Exit early on error
            }
        };
        
        // Load and parse local contexts
        // let context_map = load_test_context(&context_paths);
        for (i, doc) in documents.iter().enumerate() {
            // Read and parse the JSON file
            match read_json_file(doc) {
                Ok(document) => {
                    println!("Successfully read and parsed JSON-LD document: {:?}", document);

                    assert_eq!(
                        true, false
                    );

                }
                Err(e) => {
                    eprintln!("Failed to read or parse JSON-LD document: {}", e);
                }
            }
        }

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
    } 
}
