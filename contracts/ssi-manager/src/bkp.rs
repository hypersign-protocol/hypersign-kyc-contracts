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
        use rio_turtle::TurtleFormatter;
        use std::io::Cursor;
        use static_iref::iri;
        use url::Url;
        use rdf::node::Node;
        use rdf::reader::turtle_parser::TurtleParser;
        use std::str::FromStr;
        use rdf::reader::rdf_parser::RdfParser;
        use rdf_types::LexicalQuad;
        use ssi_rdf::urdna2015::normalize;
        use locspan::Meta;
        use json_syntax::{ Print, json };
        use json_ld::{JsonLdProcessor, Options, Loader, NoLoader};
        use std::fmt;
        use rdf_types::Iri;
        use std::io::Write;
        use std::collections::HashSet;
        use nquads_syntax::Parse;
        use oxigraph::io::{RdfFormat, RdfSerializer};
        use oxigraph::io::{GraphParser, GraphFormat};
        use oxigraph::model::{Quad, NamedNode, Triple, GraphName, BlankNode, Term, Graph, Literal, TripleRef};
        use oxigraph::model::Subject;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        // Counter for generating blank node IDs
        static BLANK_NODE_COUNTER: AtomicUsize = AtomicUsize::new(0);
        
      
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
                    
                        let document_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/test_basic.json";
                        let merged_context_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context.json";
                        let expanded_document_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_vc.json";

                        // Read the expanded doc
                        let expanded_str = fs::read_to_string(expanded_document_path).expect("Failed");
                        let expanded: Value = serde_json::from_str(&expanded_str).expect("Failed");

                        let graph = convert_expanded_jsonld_to_graph(&expanded).expect("Failed");
                        let triples = collect_triples_as_vec(&graph);

                        // Print the triples
                        for triple in graph.iter() {
                            println!(
                                "{} {} {} .",
                                triple.subject,
                                triple.predicate,
                                triple.object
                            );
                        }

                        // Step 2: Convert the expanded JSON-LD to Triples
                        // // Create an in-memory graph to hold the parsed data
                        // let mut graph = oxigraph::model::Graph::default();

                        // // Parse the JSON-LD input into the graph
                        // GraphParser::from_format(GraphFormat::JsonLd)
                        //     .read_into(&mut graph, &mut reader)?;

                        // // Create a buffer to hold the serialized N-Quads string
                        // let mut buffer = Vec::new();

                        // // Serialize the graph to N-Quads format and write it to the buffer
                        // GraphSerializer::from_format(GraphFormat::NQuads)
                        //     .serialize_graph(&graph, None, &mut buffer)?;

                        // // Convert the buffer to a UTF-8 string and print it
                        // let nquads_string = String::from_utf8(buffer).expect("Buffer contains invalid UTF-8");
                        // println!("{}", nquads_string);
                        
                        // let turtle_output = convert_jsonld_to_turtle(&expanded_str).expect("Buffer contains invalid UTF-8");
                        // println!("{}", turtle_output);

                        // // Step 4 TTL to N-Quads
                        // let ttl_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded.ttl";
                        // let turtle_data = fs::read_to_string(ttl_path).expect("Failed");

                        // // Parse Turtle input into a collection of quads
                        // let mut parser = GraphParser::from_format(GraphFormat::Turtle);
                        // let triples: Vec<Triple> = parser.read_triples(turtle_data.as_bytes())
                        //             .collect::<Result<Vec<_>, _>>().expect("REASON");
                   
                                    
                        // Convert triples to quads (adding a default graph name)
                        let mut quads: Vec<Quad> = triples.into_iter().map(|Triple { subject, predicate, object }| {
                            Quad {
                                subject,
                                predicate,
                                object,
                                graph_name: GraphName::DefaultGraph,
                            }
                        }).collect();

                        // Sort quads by subject string representation
                        quads.sort_by(|a, b| {
                            let subject_a = a.subject.to_string();
                            let subject_b = b.subject.to_string();
                            subject_a.cmp(&subject_b)
                        });

                        // Serialize the quads into N-Quads format
                        let mut nquads_output = Vec::new();
                        let mut serializer = RdfSerializer::from_format(RdfFormat::NQuads).serialize_to_write(&mut nquads_output);

                        for quad in quads {
                            serializer.write_quad(&quad).expect("Failed");
                        }

                        // Finish serialization
                        serializer.finish().expect("Failed");
                        // println!(
                        //     "===="
                        // );

                        // // Print the serialized N-Quads output
                        // match String::from_utf8(nquads_output) {
                        //     Ok(output_str) => println!("{}", output_str),
                        //     Err(e) => eprintln!("Error converting bytes to string: {}", e),
                        // }
                        // let nq_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_vc.nq";
                        // let nq_string = fs::read_to_string(nq_path).expect("Failed");
                        assert_eq!(
                            true, false
                        );

                        // Step 5
                        // let nq_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_vc.nq";
                        // let nq_string = fs::read_to_string(nq_path).expect("Failed");
                        // let dataset = nquads_syntax::Document::parse_str(&nq_string).unwrap();
                        // let stripped_dataset: Vec<_> = dataset
                        //     .into_value()
                        //     .into_iter()
                        //     .map(Meta::into_value)
                        //     .map(nquads_syntax::strip_quad)
                        //     .collect();
                        // let normalized = normalize(
                        //     stripped_dataset
                        //         .iter()
                        //         .map(LexicalQuad::as_lexical_quad_ref),
                        // )
                        // .into_nquads();

                        // // Nquads to Urdna normalized format
                        // let canon_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/canon_vc.txt";
                        // let canon_string = fs::read_to_string(canon_path).expect("Failed");
                        // assert_eq!(
                        //     canon_string, normalized
                        // );
                }
                Err(e) => {
                    eprintln!("Failed to read or parse JSON-LD document: {}", e);
                }
            }
        }

         // Function to generate a unique blank node identifier
         fn generate_blank_node() -> BlankNode {
            let counter = BLANK_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
            BlankNode::new(&format!("b{}", counter)).expect("Failed to create BlankNode")
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

        fn collect_triples_as_vec(graph: &Graph) -> Vec<Triple> {
            // Initialize an empty vector to collect triples
            let mut triples_vec = Vec::new();
        
            // Iterate over each TripleRef in the graph
            for triple_ref in graph.iter() {
                // Clone each component separately
                let subject = triple_ref.subject.clone();
                let predicate = triple_ref.predicate.clone();
                let object = triple_ref.object.clone();
        
                // Create a new Triple with the cloned components
                let triple = Triple::new(subject, predicate, object);
                
                // Push the Triple into the vector
                triples_vec.push(triple);
            }
        
            triples_vec
        }

        fn convert_expanded_jsonld_to_graph(jsonld: &Value) -> Result<Graph, Box<dyn Error>> {
            let mut graph = Graph::default();
        
            if let Some(nodes) = jsonld.as_array() {
                for node in nodes {
                    if let Some(subject_id) = node.get("@id").and_then(Value::as_str) {
                        let subject = NamedNode::new(subject_id)?;
                        process_node(&mut graph, &subject, node, None)?;
                    }
                }
            }
        
            Ok(graph)
        }
        
        fn process_value(value: &Value, graph: &mut Graph, blank_node: BlankNode) -> Result<Term, Box<dyn Error>> {
            match value {
                Value::String(s) => {
                    // Treat URIs as NamedNode and other strings as Literal
                    if s.starts_with('_') {
                        println!("{}", s);
                        let named_node = BlankNode::new(s)?; 
                        Ok(Term::BlankNode(named_node))
                    } else if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("did:") {
                        let named_node = NamedNode::new(s)?; 
                        Ok(Term::NamedNode(named_node))
                    } else {
                        let literal = Literal::new_simple_literal(s);
                        Ok(Term::Literal(literal))
                    }
                }
                Value::Object(obj) => {
                    // Check for @id and create NamedNode if present
                    if let Some(id) = obj.get("@id").and_then(Value::as_str) {
                        let named_node = NamedNode::new(id)?;

                        // Handle other object properties
                        let mut triples = Vec::new();
                        for (key, value) in obj {
                            if key != "@id" && key != "@value" && key != "@type" && key != "@graph" {
                                let predicate = NamedNode::new(key)?;
                                match value {
                                    Value::Array(array) => {
                                        for item in array {
                                            let term = process_value(item, graph, blank_node.clone())?;
                                            triples.push((predicate.clone(), term));
                                        }
                                    }
                                    _ => {
                                        let term = process_value(value, graph, blank_node.clone())?;
                                        triples.push((predicate.clone(), term));
                                    }
                                }
                            } else if key == "@type" {
                                // Check if value exists 
                                if obj.get("@value").is_none() {
                                    // Perform your logic here when @value exists
                                    let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
                                    let term = Term::NamedNode(NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?);
                                    triples.push((predicate.clone(), term));
                                }
                            }
                        }
                        // Insert triples into the graph
                        for (predicate, term) in triples {
                            let placeholder_subject = NamedNode::new("http://example.org/subject2")?;
                            let triple = Triple::new(named_node.clone(), predicate, term);
                            graph.insert(&triple);
                        }
                   
                        return Ok(Term::NamedNode(named_node));
                    }
                    
                    // Handle @value for literals
                    if let Some(value) = obj.get("@value").and_then(Value::as_str) {
                        if let Some(type_str) = obj.get("@type").and_then(Value::as_str) {
                            // Concatenate value and type
                            let literal = Literal::new_simple_literal(&format!("{}^^<{}>", value, type_str));
                            return Ok(Term::Literal(literal));
                        } else {
                            // Handle case where @type is missing
                            let literal = Literal::new_simple_literal(value);
                            return Ok(Term::Literal(literal));
                        }
                    }
        
                    // Handle @type if present
                    if let Some(types) = obj.get("@type").and_then(Value::as_array) {
                        let mut terms = Vec::new();
                        for type_value in types {
                            let term = process_value(type_value, graph, blank_node.clone())?;
                            terms.push(term);
                        }
                        return Ok(terms.first().cloned().ok_or("No types found")?);
                    }
        
                    // Handle @graph for nested graphs
                    if let Some(graph_array) = obj.get("@graph").and_then(Value::as_array) {
                        let blank_node_new = generate_blank_node();

                        let mut nested_graph = Graph::default();
                        for nested_node in graph_array {
                            process_node(&mut nested_graph, &NamedNode::new_unchecked(&format!("{}", blank_node_new)), nested_node, Some(blank_node.clone()))?;
                        }
                        // Insert all nested triples into the main graph
                        for triple in nested_graph.iter() {
                            graph.insert(triple);
                        }
                        return  Ok(Term::NamedNode(NamedNode::new_unchecked(&format!("{}", blank_node))))
                    }
        
                    Ok(Term::NamedNode(NamedNode::new_unchecked(&format!("{}", blank_node_new))))
                }
                Value::Array(array) => {
                    // Process each item in the array
                    let mut terms = Vec::new();
                    for item in array {
                        let term = process_value(item, graph, blank_node.clone())?;
                        terms.push(term);
                    }
                    // Insert all terms into the graph with a placeholder subject and predicate
                    for (index, term) in terms.iter().enumerate() {
                        let placeholder_subject = NamedNode::new("http://example.org/array")?;
                        let placeholder_predicate = NamedNode::new("http://example.org/hasValue")?;
                        let triple = Triple::new(placeholder_subject.clone(), placeholder_predicate.clone(), term.clone());
                        graph.insert(&triple);
                    }
                    // Return a term for the array as a whole if needed
                    Ok(Term::NamedNode(NamedNode::new("http://example.org/array")?))
                }
                _ => Err("Unsupported JSON-LD value".into()),
            }
        }
        
        fn process_node(graph: &mut Graph, subject: &NamedNode, node: &Value, blank_node: Option<BlankNode>) -> Result<(), Box<dyn Error>> {
            let blank_node = generate_blank_node(); // Generate blank node on first processing
            if let Some(properties) = node.as_object() {
                for (predicate_str, objects) in properties {
                    if predicate_str == "@type"  {
                        let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
                        match objects {
                            Value::Array(values) => {
                                for value in values {
                                    let object = process_value(value, graph, blank_node.clone())?;
                                    let triple = Triple::new(
                                        subject.clone(), // Ensure subject is a NamedNode Term
                                        predicate.clone(),
                                        object
                                    );
                                    graph.insert(&triple);
                                }
                            }
                            value => {
                                let object = process_value(value, graph, blank_node.clone())?;
                                let triple = Triple::new(
                                    subject.clone(), // Ensure subject is a NamedNode Term
                                    predicate.clone(),
                                    object
                                );
                                graph.insert(&triple);
                            }
                        }
                    } else if predicate_str != "@id" {
                        let predicate = NamedNode::new(predicate_str)?;
                        match objects {
                            Value::Array(values) => {
                                for value in values {
                                    let object = process_value(value, graph, blank_node.clone())?;
                                    // println!("Triple {} {} Obj {:?} {:?}", subject, predicate, object, blank_node);
                                    let triple = Triple::new(
                                        subject.clone(), // Ensure subject is a NamedNode Term
                                        predicate.clone(),
                                        object
                                    );
                                    graph.insert(&triple);
                                }
                            }
                            value => {
                                let object = process_value(value, graph, blank_node.clone())?;
                                let triple = Triple::new(
                                    subject.clone(), // Ensure subject is a NamedNode Term
                                    predicate.clone(),
                                    object
                                );
                                graph.insert(&triple);
                            }
                        }
                    } 
                }
            }
            Ok(())
        }
        
    } 
}
