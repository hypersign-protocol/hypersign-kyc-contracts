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
        use serde_json::{self, Value, Map};
        use std::collections::HashMap;
        use std::fs::File;
        use std::io::Read;
        use std::fs;
        use std::error::Error;
        use std::path::PathBuf;
        use std::io;
        // use json_ld::{JsonLdProcessor, Options, RemoteDocument, RemoteDocumentReference};
        // use rio_api::model::{Triple, NamedNode};
        use rio_turtle::TurtleFormatter;
        use std::sync::Arc;
        use std::io::Cursor;
        use rdf_types::Quad;
        use static_iref::iri;
        use url::Url;
        use rdf::node::Node;
        use rdf::triple::Triple as RdfTriple;
        use rdf::graph::Graph;
        use rdf::reader::turtle_parser::TurtleParser;
        use std::str::FromStr;
        use rdf::reader::rdf_parser::RdfParser;
        // use nquads_syntax::Parse;
        use rdf_types::LexicalQuad;
        use ssi_rdf::urdna2015::normalize;
        use locspan::Meta;
        // use ssi_json_ld::{CompactJsonLd, ContextLoader, Expandable};
        use json_syntax::{ Print, json };
        // use json_ld::expand;
        // use json_ld::rdf::{DataSet, ToRdf};
        use rdf_types::{Term, Iri, Literal};
        use nquads_syntax::{NQuadWriter, NQuad};

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

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Triple {
            subject: String,
            predicate: String,
            object: String,
        }

        impl Triple {
            fn new(subject: String, predicate: String, object: String) -> Self {
                Triple {
                    subject,
                    predicate,
                    object,
                }
            }
        }
        
        type ContextMap = HashMap<String, Value>;
        
        // Implement a conversion from Source to Target
        impl From<&RdfTriple> for Triple {
            fn from(rtriple: &RdfTriple) -> Self {
                Triple {
                    subject: match &rtriple.subject() {
                        Node::BlankNode { id } => format!("{}", id),
                        Node::LiteralNode {
                            literal,
                            data_type,
                            language,
                        } => format!("{}", literal),
                        Node::UriNode { uri } => format!("{}", uri.to_string()),
                        _ =>  format!("Failed"),
                    },
                    predicate: match &rtriple.subject() {
                        Node::BlankNode { id } => format!("{}", id),
                        Node::LiteralNode {
                            literal,
                            data_type,
                            language,
                        } => format!("{}", literal),
                        Node::UriNode { uri } => format!("{}", uri.to_string()),
                        _ =>  format!("Failed"),
                    },
                    object: match &rtriple.subject() {
                        Node::BlankNode { id } => format!("{}", id),
                        Node::LiteralNode {
                            literal,
                            data_type,
                            language,
                        } => format!("{}", literal),
                        Node::UriNode { uri } => format!("{}", uri.to_string()),
                        _ =>  format!("Failed"),
                    },
                }
            }
        }

        // Function to read JSON context from a file
        fn read_context_from_file(file_path: &str) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
            let content = fs::read_to_string(file_path)?;
            let context: HashMap<String, Value> = serde_json::from_str(&content)?;
            Ok(context)
        }

        // Function to merge multiple JSON contexts
        // fn merge_contexts(context_files: &[&str]) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        //     let mut merged_context = HashMap::new();

        //     for file in context_files {
        //         let context = read_context_from_file(file)?;
        //         merged_context.extend(context);
        //     }

        //     Ok(merged_context)
        // }

        // fn json_ld_to_triples(json_ld: Value) -> Vec<Triple> {
        //     let mut triples = Vec::new();

        //     if let Some(objects) = json_ld.as_object() {
        //         for (subject, value) in objects {
        //             // Simplified extraction logic, assumes predicate and object are directly available
        //             if let Some(predicates) = value.as_object() {
        //                 for (predicate, obj) in predicates {
        //                     if let Some(object) = obj.as_str() {
        //                         triples.push(Triple {
        //                             subject: subject.clone(),
        //                             predicate: predicate.clone(),
        //                             object: object.to_string(),
        //                         });
        //                     }
        //                 }
        //             }
        //         }
        //     }

        //     triples
        // }

    
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
            match read_json_file(doc.to_path_buf()) {
                Ok(document) => {
                    // println!("Successfully read and parsed JSON-LD document: {:?}", document);
                    // List of context files
                    // let context_files = [
                    //         "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context1.json", 
                    //         "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context2.json"
                    // ];

                    // Use `FsLoader` to redirect any URL starting with `https://example.com/` to
                    // the local `example` directory. No HTTP query.
                    // let mut loader = json_ld::FsLoader::default();
                    // loader.mount(iri!("https://www.w3.org/ns/did/v1").to_owned(), "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/");
                    // loader.mount(iri!("https://w3id.org/security/suites/ed25519-2020/v1").to_owned(), "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/");

                    // Merge contexts from the files
                    // let context = merge_contexts(&context_files).unwrap();

                    // Create a JSON-LD processor and load the document
                    // let processor = JsonLdProcessor::default();
                    // let document = processor.create_document_from_json(&document)?;

                    // Process the JSON-LD document to RDF
                    // let rdf_graph = processor.to_rdf(&document, Some(&context))?;

                    // let local_file_path = Path::new("/path/to/your/local/file.jsonld");
                    // let iri_index = "file://".to_string() + local_file_path.to_str().unwrap();

                    // let input = RemoteDocumentReference::iri(iri_index);
                    // let mut generator = rdf_types::generator::Blank::new();
                    // let mut rdf = input.to_rdf(
                    //                 &mut generator,
                    //                     &loader
                    //                 )
                    //                 .expect("flattening failed");
                                                        
                    // for Quad(s, p, o, g) in rdf.quads() {
                    //     println!("subject: {}", s);
                    //     println!("predicate: {}", p);
                    //     println!("object: {}", o);
                    
                    //     if let Some(g) = g {
                    //     println!("graph: {}", g);send
                    //     }
                    // }

//                     let expected = r#"<did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519VerificationKey2020> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> <https://w3id.org/security#controller> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> <https://w3id.org/security#publicKeyMultibase> "z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL"^^<https://w3id.org/security#multibase> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://w3id.org/security#assertionMethod> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://w3id.org/security#authenticationMethod> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://w3id.org/security#capabilityDelegationMethod> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://w3id.org/security#capabilityInvocationMethod> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://w3id.org/security#controller> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> .
// <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> <https://www.w3.org/ns/activitystreams#alsoKnownAs> <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> ."#;

                    let turtle_data = r#"
                    @prefix did: <https://www.w3.org/ns/did/v1#> .
                    @prefix sec: <https://w3id.org/security/suites/ed25519-2020/v1#> .
                    
                    <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL>
                        a did:DidDocument ;
                        did:controller <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> ;
                        did:alsoKnownAs <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> ;
                        did:verificationMethod <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> ;
                        did:authentication <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> ;
                        did:assertionMethod <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> ;
                        did:keyAgreement [] ;
                        did:capabilityInvocation <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1>,
                                                 <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> ;
                        did:capabilityDelegation <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1> ;
                        did:service [] .
                    
                    <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL#key-1>
                        a sec:Ed25519VerificationKey2020 ;
                        did:controller <did:hid:testnet:z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL> ;
                        sec:publicKeyMultibase "z6MkkcVrXobq46BPgbjBozG2dfWm5d8KR8uYiJRP26cX2XUL" .
                    "#;

                    // Create a Turtle parser
                    let parser = TurtleParser::new();
                    let triples = parser.parse(&turtle_data).expect("Failed to parse Turtle data");

                    // Convert triples to N-Quads using the separate function
                    let nquads = convert_triples_to_nquads(triples);

                    // Output the N-Quads
                    println!("{}", nquads);
                
//                     // Parse the RDF data using the Turtle parser
//                     let mut parser = TurtleParser::from_string(turtle_data);
//                     let graph = parser.decode().unwrap();
//                     let blank_node = "_:c14n0";

//                     let mut triples: Vec<(Triple)> = graph.triples_iter()
//                                 .map(|rdf_triple| Triple {
//                                     subject: match &rdf_triple.subject() {
//                                         Node::BlankNode { id } => format!("{}", blank_node),
//                                         Node::LiteralNode {
//                                             literal,
//                                             data_type,
//                                             language,
//                                         } => format!("{}", literal),
//                                         Node::UriNode { uri } => format!("{}", uri.to_string()),
//                                         _ =>  format!("Failed"),
//                                     },
//                                     predicate: match &rdf_triple.predicate() {
//                                         Node::BlankNode { id } => format!("{}", blank_node), // is tthis possible
//                                         Node::LiteralNode {
//                                             literal,
//                                             data_type,
//                                             language,
//                                         } => format!("{}", literal),
//                                         Node::UriNode { uri } => format!("{}", uri.to_string()),
//                                         _ =>  format!("Failed"),
//                                     },
//                                     object: match &rdf_triple.object() {
//                                         Node::BlankNode { id } => format!("{}", blank_node), // is tthis possible
//                                         Node::LiteralNode {
//                                             literal,
//                                             data_type,
//                                             language,
//                                         } => format!("{}", literal),
//                                         Node::UriNode { uri } => format!("{}", uri.to_string()),
//                                         _ =>  format!("Failed"),
//                                     },
//                                 })  // Filter out any errors
//                                 .collect();
                                
//                     let canonical_form = canonize_rdf(&mut triples);
//                     assert_eq!(
//                         canonical_form, expected
//                     );

//                         let expected_str = r#"_:c14n0 <http://purl.org/dc/terms/created> "2010-01-01T19:23:24Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
// _:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> .
// _:c14n0 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#assertionMethod> .
// _:c14n0 <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1> .
// "#;
//                         let in_str = r#"_:b0 <http://purl.org/dc/terms/created> "2010-01-01T19:23:24Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
// _:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> .
// _:b0 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#assertionMethod> .
// _:b0 <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1> .
// "#;
//                         let in_str = String::from_utf8(buffer)?;
//                         let dataset = nquads_syntax::Document::parse_str(&in_str).unwrap();
//                         let stripped_dataset: Vec<_> = dataset
//                             .into_value()
//                             .into_iter()
//                             .map(Meta::into_value)
//                             .map(nquads_syntax::strip_quad)
//                             .collect();
//                         let normalized = normalize(
//                             stripped_dataset
//                                 .iter()
//                                 .map(LexicalQuad::as_lexical_quad_ref),
//                         )
//                         .into_nquads();
                        // assert_eq!(
                        //     &normalized, &expected_str
                        // );

                        // Safely expand jsonld with local processor
                        // let input = CompactJsonLd(json_syntax::json!({
                        //     "@context": [
                        //         "https://www.w3.org/ns/did/v1",
                        //         "https://w3id.org/security/suites/ed25519-2020/v1"
                        //     ],
                        //     "type": "Ed25519Signature2020",
                        //     "created": "2010-01-01T19:23:24Z",
                        //     "verificationMethod": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1",
                        //     "proofPurpose": "assertionMethod"
                        // }));
                        // match input.expand(&ContextLoader::default()).await.unwrap_err() {
                        //     JsonLdError::Expansion(json_ld::expansion::Error::InvalidTypeValue) => (),
                        //     e => panic!("{:?}", e),
                        // }

                        let document_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/test_basic.json";
                        let merged_context_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context.json";
                        let expanded_document_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_test_basic.json";

                        // // Paths to local context files
                        // let context_paths = vec![
                        //     "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context1.json", 
                        //     "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context2.json", 
                        // ];
                        
                        // // Load and merge contexts
                        // let context = resolve_contexts(context_paths).expect("Failed");

                        // // Print the resolved context map
                        // let context_json = context_map_to_json_string(&context);
                        // println!("Resolved ContextMap:\n{:?}", context_json);
                        
                        // Hard code the context map
                        
                        // Load JSON-LD document
                        // let document_content = fs::read_to_string(document_path).expect("Failed");
                        // let document: Value = serde_json::from_str(&document_content).expect("Failed");
                        
                        // let merged = fs::read_to_string(merged_context_path).expect("Failed");
                        // let context_map: ContextMap = serde_json::from_str(&merged).expect("Failed");
                        // let context_json = context_map_to_json_string(&context_map);

                        // Expand JSON-LD document using the merged context
                        // let mut expanded =  expand_value("@context", &document, &context_map).expect("Failed to expand JSON-LD document");

                        // Access the JSON object as a mutable map
                        // if let Some(map) = expanded.as_object_mut() {
                        //      //    Key to be removed
                                
                        //     let key_to_remove = "@context";

                        //     // Remove the key from the map
                        //     if map.remove(key_to_remove).is_some() {
                        //         println!("Removed key: {}", key_to_remove);
                        //     } else {
                        //         println!("Key not found: {}", key_to_remove);
                        //     }
                        // }

                        // Read the expanded doc
                        let expanded_str = fs::read_to_string(expanded_document_path).expect("Failed");
                        let expanded: Value = serde_json::from_str(&expanded_str).expect("Failed");

                        // Step 2: Convert the expanded JSON-LD to RDF quads
                        // let dataset = expanded.to_rdf()?;
                        // println!("Expanded JSON-LD: {:?}", expanded);
                        // let triples = json_ld_to_rdf_triples(&expanded);

                        // Convert JSON-LD to RDF triples
                        // let rdf_triples = match expanded.as_array() {
                        //     Some(arr) => arr.iter()
                        //         .flat_map(|item| {
                        //             let subject = item["@id"].as_str().unwrap_or("");
                        //             json_ld_to_rdf_triples(subject, item)
                        //         })
                        //         .collect(),
                        //     None => vec![],
                        // };


                        // Step 3: Serialize RDF quads as N-Quads
                        // let nquads = NQuadsSerializer::new_string().serialize_dataset(&dataset)?;   

                        println!("Final Result: {:?}", expanded);

                        // for (term, uri) in expanded {
                        //     println!("Term: {}, URI: {}", term, uri);
                        // }
                        assert_eq!(
                           true, false
                        );


                }
                Err(e) => {
                    eprintln!("Failed to read or parse JSON-LD document: {}", e);
                }
            }
        }

        // Function to convert RDF triples to N-Quads
        fn convert_triples_to_nquads(triples: Vec<Triple>) -> String {
            let mut nquad_writer = NQuadWriter::new();

            for triple in triples {
                let nquad = NQuad::new(
                    triple.subject().to_string(),
                    triple.predicate().to_string(),
                    triple.object().to_string(),
                    None // Graph name is optional
                );

                nquad_writer.write(&nquad).expect("Failed to write N-Quad");
            }

            nquad_writer.to_string()
        }


        // // Function to convert JSON-LD value to RDF triples
        // fn json_ld_to_rdf_triples(subject: &str, json_ld: &Value) -> Vec<(Term, Term, Term)> {
        //     let mut triples = Vec::new();
        
        //     if let Some(properties) = json_ld.as_object() {
        //         for (predicate_str, object_value) in properties {
        //             let predicate = Term::Iri(Iri::new(predicate_str.clone()).unwrap());
        
        //             if let Some(objects) = object_value.as_array() {
        //                 for obj in objects {
        //                     let object = match obj {
        //                         Value::String(s) => Term::Literal(Literal::new(s.clone())),
        //                         Value::Object(inner_obj) => {
        //                             // Handle nested objects
        //                             let inner_subject = inner_obj["@id"].as_str().unwrap_or("");
        //                             let inner_triples = json_ld_to_rdf_triples(inner_subject, obj);
        //                             triples.extend(inner_triples);
        //                             continue;
        //                         }
        //                         _ => continue, // Handle other types as needed
        //                     };
        
        //                     triples.push((
        //                         Term::Iri(Iri::new(subject.to_string()).unwrap()),
        //                         predicate.clone(),
        //                         object.clone(),
        //                     ));
        //                 }
        //             } else if let Some(value) = object_value.as_str() {
        //                 let object = Term::Literal(Literal::new(value.to_string()));
        //                 triples.push((
        //                     Term::Iri(Iri::new(subject.to_string()).unwrap()),
        //                     predicate.clone(),
        //                     object.clone(),
        //                 ));
        //             }
        //         }
        //     }
        
        //     triples
        // }

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

        // Load a context file
        fn load_context(path: &str) -> Result<Value, std::io::Error> {
            let content = fs::read_to_string(path)?;
            let context: Value = serde_json::from_str(&content)?;
            Ok(context)
        }

        // Recursive function to merge contexts, handling nested contexts
        fn merge_contexts(context: &Value, merged: &mut ContextMap) {
            if let Some(map) = context.as_object() {
                for (key, value) in map {
                    println!("{} {:?}", key, value);
                    if key == "@context" {
                        // Recursively merge nested contexts
                        if let Some(nested_context) = value.as_object() {
                            for (nested_key, nested_value) in nested_context {
                                let nested_value = Value::Object(nested_value.clone().as_object().unwrap_or(&Map::new()).clone());
                                merge_contexts(&nested_value, merged);
                            }
                        }
                    } else {
                        // Directly insert the value into the merged context
                        merged.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        // Function to convert merged context map to a JSON string
        fn context_map_to_json_string(context_map: &ContextMap) -> Result<String, serde_json::Error> {
            let json_value = Value::Object(
                context_map.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            );
            serde_json::to_string_pretty(&json_value)
        }
        

        // Function to resolve contexts from local files and return as JSON string
        fn resolve_contexts(paths: Vec<&str>) -> Result<ContextMap, io::Error> {
            let mut merged_context = ContextMap::new();
            for path in paths {
                let context = load_context(path)?;
                println!("{:?}", context_map_to_json_string(&merged_context));
                merge_contexts(&context, &mut merged_context);
            }
            Ok(merged_context)
        }
        

        // fn get_expanded(document: &Value, context: &Value) -> Result<Value, std::io::Error> {
        //     let expanded = expand(document, context)?;
        //     Ok(expanded)
        // }

        // Function to expand terms in a JSON-LD value
        fn expand_value(key: &str, value: &Value, context: &ContextMap) -> Result<Value, String> {
        
            match value {
                Value::Object(map) => {
                    let mut expanded_map: Map<String, Value> = map.iter()
                        .map(|(k, v)| {
                            let expanded_context = get_nested_context(k, context);

                            let mut ctx;
                            if expanded_context != *context {
                                ctx = expanded_context.clone();
                            }

                            let expanded_key = expand_term(k, &context);
                            let expanded_value = expand_value(k, v, &expanded_context).unwrap_or(Value::Null);
                            (expanded_key, expanded_value)
                        })
                        .collect();
                    Ok(Value::Object(expanded_map))
                }
                Value::Array(arr) => {
                    // Get the nested context for the array
                    let expanded_context = get_nested_context(key, context);
                    let mut expanded_arr = Vec::new();
                    for v in arr {
                        let expanded_value = match v {
                            Value::String(s) => {
                               if should_wrap_with_id(context, key) {
                                    Value::Array(vec![Value::Object(serde_json::json!({
                                        "@id": s
                                    }).as_object().unwrap().clone())])
                                } else {
                                    Value::String(s.clone())
                                }
                            }
                            _ => {
                                expand_value(key, v, &expanded_context)?
                            },
                        };
                        expanded_arr.push(expanded_value);
                    }
                   
                    Ok(Value::Array(expanded_arr))
                }
                Value::String(s) => {
                    if key == "id" {
                        Ok(Value::String(s.clone()))
                    } else if let Some(Value::Object(ctx)) = context.get(key) {
                        if let Some(Value::String(type_value)) = ctx.get("@type") {
                            if type_value != "@id" {
                                Ok(Value::Array(vec![Value::Object(serde_json::json!({
                                    "@type": type_value,
                                    "@value": s
                                }).as_object().unwrap().clone())]))
                            } else {
                                Ok(Value::Array(vec![Value::Object(serde_json::json!({
                                    "@id": s
                                }).as_object().unwrap().clone())]))
                            }
                        } else {
                            Ok(Value::Object(serde_json::json!({
                                "@id": s
                            }).as_object().unwrap().clone()))
                        }
                    } else {
                        Ok(Value::Array(vec![Value::String(s.clone())]))
                    }
                }
                _ => {
                    if should_wrap_with_id(context, key) && key != "@id" {
                        Ok(Value::Array(vec![Value::Object(serde_json::json!({
                            "@id": value.clone()
                        }).as_object().unwrap().clone())]))
                    } else if key == "@id" {
                        Ok(value.clone())
                    } else {
                        Ok(Value::Array(vec![value.clone()]))
                    }
                }
            }
}

        
        // Function to expand a term based on context
        fn expand_term(term: &str, context: &ContextMap) -> String {
            println!("Start {} {:?}", term, context);
            match context.get(term) {
                Some(Value::String(s)) => s.clone(),  // Handle direct String value
                Some(Value::Object(obj)) => {
                    if let Some(Value::String(s)) = obj.get("@id") {
                        s.clone()  // Replace term with @id value in nested contexts
                    } else {
                        term.to_string()  // Fallback if no specific mapping found
                    }
                }
                _ => term.to_string(),  // Default case
            }
        }
        
        // Function to determine if the value should be wrapped in an object with @id
        fn should_wrap_with_id(context: &ContextMap, term: &str) -> bool {
            if let Some(Value::Object(map)) = context.get(term) {
                map.contains_key("@id") && map.contains_key("@type")
            } else {
                false
            }
        }

        // Function to get the nested context if it exists
        fn get_nested_context(term: &str, context: &ContextMap) -> ContextMap {
            if let Some(Value::Object(map)) = context.get(term) {
                if let Some(Value::Object(nested_context)) = map.get("@context") {
                    return nested_context.clone().into_iter().collect();
                }
            }
            context.clone()
        }

    } 
}
