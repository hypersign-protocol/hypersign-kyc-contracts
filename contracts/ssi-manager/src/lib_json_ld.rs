use std::error::Error;
use sha2::{Sha256, Digest};
use serde_json::{self, Value, Map};
use rdf_types::LexicalQuad;
use ssi_rdf::urdna2015::normalize;
use locspan::Meta;
use nquads_syntax::Parse;
use oxigraph::io::{RdfFormat, RdfSerializer};
use oxigraph::io::{GraphParser, GraphFormat};
use oxigraph::model::{Quad, NamedNode, Triple, GraphName, BlankNode, Term, Graph, Literal, TripleRef, NamedOrBlankNode };
use oxigraph::model::Subject;
use cosmwasm_std::{Binary, Response, StdError, StdResult, MessageInfo, DepsMut, Env};
use crate::ed25519_signature_2020::{transfrom_signature, transform_public_key, decode_hex_message};

/// Hashes a string using SHA-256
pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    
    // Convert the hash result to a hexadecimal string
    hex::encode(result)
}

pub fn extract_after_last_delimiter(input: &str, delimiter: char) -> &str {
    // Split the string by the delimiter and collect the parts into a vector
    let parts: Vec<&str> = input.split(delimiter).collect();

    // Return the last part of the vector
    return parts.last().unwrap_or(&"")
}

// https://w3c.github.io/vc-di-eddsa/#transformation-ed25519signature2020
pub fn get_cannonized_str(expanded_did_json: Value) -> String {

                            let graph = convert_expanded_jsonld_to_graph(&expanded_did_json).expect("Failed");
                            let triples = collect_triples_as_vec(&graph);

                            for triple in triples.iter() {
                                println!(
                                    "{} {} {} .",
                                    triple.subject,
                                    triple.predicate,
                                    triple.object
                                );
                            }
                            
                            // Convert triples to quads (adding a default graph name)
                            let mut quads: Vec<Quad> = triples.into_iter().map(|Triple { subject, predicate, object }| {
                            let graph_name = if subject.to_string().starts_with("_:b0") {
                                    GraphName::DefaultGraph
                                } else if subject.to_string().starts_with("_:") {
                                    // Use the blank node identifier as the graph name
                                    let named_node = BlankNode::new_unchecked(extract_blank_node_id(&subject.to_string()).unwrap());
                                    GraphName::BlankNode(named_node)
                                } else {
                                    GraphName::DefaultGraph
                                };


                                Quad {
                                    subject,
                                    predicate,
                                    object,
                                    graph_name: graph_name
                                }
                            }).collect();

                        // Sort quads by subject, predicate, and object in lexicographic order
                        quads.sort_by(|a, b| {
                            let subject_a = a.subject.to_string();
                            let subject_b = b.subject.to_string();
                            let predicate_a = a.predicate.to_string();
                            let predicate_b = b.predicate.to_string();
                            let object_a = a.object.to_string();
                            let object_b = b.object.to_string();

                            // First, compare by subject
                            match subject_a.cmp(&subject_b) {
                                std::cmp::Ordering::Equal => {
                                    // If subjects are equal, compare by predicate
                                    match predicate_a.cmp(&predicate_b) {
                                        std::cmp::Ordering::Equal => {
                                            // If predicates are equal, compare by object
                                            object_a.cmp(&object_b)
                                        }
                                        ordering => ordering,
                                    }
                                }
                                ordering => ordering,
                            }
                        });

                        // Serialize the quads into N-Quads format
                        let mut nquads_output = Vec::new();
                        let mut serializer = RdfSerializer::from_format(RdfFormat::NQuads).serialize_to_write(&mut nquads_output);

                        for quad in quads {
                            serializer.write_quad(&quad).expect("Failed");
                        }

                        // // Finish serialization
                        // serializer.finish().expect("Failed");
                        // println!(
                        //     "===="
                        // );

                        // // Print the serialized N-Quads output
                        // match String::from_utf8(nquads_output.clone()) {
                        //     Ok(output_str) => println!("{}", output_str),
                        //     Err(e) => eprintln!("Error converting bytes to string: {}", e),
                        // }
                        // let nq_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_vc.nq";
                        // let nq_string = fs::read_to_string(nq_path).expect("Failed");
                        // assert_eq!(
                        //     true, false
                        // );

                        // Step 5
                        // let nq_path =  "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/test/expanded_vc.nq";
                        // let nq_string = fs::read_to_string(nq_path).expect("Failed");
                        let nq_string = String::from_utf8(nquads_output).unwrap();

                        // Parse the N-Quad string back to a dataset
                        let dataset =  nquads_syntax::Document::parse_str(&nq_string).unwrap();

                        // let dataset = nquads_syntax::Document::parse_str(&nq_string).unwrap();
                        let stripped_dataset: Vec<_> = dataset
                            .into_value()
                            .into_iter()
                            .map(Meta::into_value)
                            .map(nquads_syntax::strip_quad)
                            .collect();
                        let normalized = normalize(
                            stripped_dataset
                                .iter()
                                .map(LexicalQuad::as_lexical_quad_ref),
                        )
                        .into_nquads();

    return normalized
}


        // Function to generate a unique blank node identifier
        fn generate_blank_node(counter: &mut usize) -> BlankNode {
            let blank_node = BlankNode::new(&format!("b{}", counter)).expect("Failed to create BlankNode");
            *counter += 1;
            blank_node
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
            let mut counter: usize = 0;
            if let Some(nodes) = jsonld.as_array() {
                for node in nodes {
                    if let Some(subject_id) = node.get("@id").and_then(Value::as_str) {
                        let blank_node = generate_blank_node(&mut counter);
                        process_node(&mut graph, node, Some(&blank_node), &mut counter)?;
                    }  else {
                        process_node(&mut graph, node, None, &mut counter)?;
                    }
                }
            }
        
            Ok(graph)
        }
        
        fn process_value(value: &Value, graph: &mut Graph, blank_node: Option<&BlankNode>, counter: &mut usize) -> Result<Term, Box<dyn Error>> {
            match value {
                Value::String(s) => {
                    // Treat URIs as NamedNode and other strings as Literal
                    if s.starts_with('_') {
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
                                            let term = process_value(item, graph, blank_node, counter)?;
                                            triples.push((predicate.clone(), term));
                                        }
                                    }
                                    _ => {
                                        let term = process_value(value, graph, blank_node, counter)?;
                                        triples.push((predicate.clone(), term));
                                    }
                                }
                            } else if key == "@type" {
                                // Check if value exists 
                                if obj.get("@value").is_none() {
                                    // Perform your logic here when @value exists
                                    let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
                                    if let Some(array) = value.as_array() {
                                        for (index, item) in array.iter().enumerate() {
                                            let term = process_value(item, graph, blank_node, counter)?;
                                            triples.push((predicate.clone(), term));
                                        }
                                    }
                                    // let term = process_value(value, graph, blank_node.clone())?;
                                    // let term = NamedNode::new(value.get(0).cloned())?;
                                    // let term =  Term::NamedNode(NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?);
                                    // triples.push((predicate.clone(), term));
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
                            // Concatenate value and type without quotes
                            let literal_value = format!("{}^^<{}>", value, type_str);
                            let literal = Literal::new_typed_literal(value, NamedNode::new(type_str)?);
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
                            let term = process_value(type_value, graph, blank_node, counter)?;
                            terms.push(term);
                        }
                        return Ok(terms.first().cloned().ok_or("No types found")?);
                    }
        
                    // Handle @graph for nested graphs
                    if let Some(graph_array) = obj.get("@graph").and_then(Value::as_array) {
                       
                        // Start a new subgraph
                        let mut nested_graph = Graph::default();

                        let blank_node_new = generate_blank_node(counter);
                        for nested_node in graph_array {
                            process_node(&mut nested_graph, nested_node, Some(&blank_node_new), counter)?;
                        }
                        // Insert all nested triples into the main graph
                        for triple in nested_graph.iter() {
                            graph.insert(triple);
                        }
                        return  Ok(Term::BlankNode(blank_node.unwrap().clone()))
                    }
        
                    Ok(Term::NamedNode(NamedNode::new_unchecked(&format!("{}", blank_node.unwrap()))))
                }
                Value::Array(array) => {
                    // Process each item in the array
                    let mut terms = Vec::new();
                    for item in array {
                        let term = process_value(item, graph, blank_node, counter)?;
                        terms.push(term);
                    }
                    // Insert all terms into the graph with a placeholder subject and predicate
                    for (index, term) in terms.iter().enumerate() {
                        let subject = NamedNode::new("http://example.org/array1")?;
                        let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
                        let triple = Triple::new(subject.clone(), predicate.clone(), term.clone());
                        graph.insert(&triple);
                    }
                    // Return a term for the array as a whole if needed
                    Ok(Term::NamedNode(NamedNode::new("http://example.org/array2")?))
                }
                _ => Err("Unsupported JSON-LD value".into()),
            }
        }
        
        fn process_node(graph: &mut Graph, node: &Value, blank_node: Option<&BlankNode>, counter: &mut usize) -> Result<(), Box<dyn Error>> {

            // If no blank node
            // if let Some(id_value) = node.get("@id").and_then(|v| v.as_str()) {
            //     let mut blank_node = Some(blank_node.unwrap_or_else(|| {
            //             // Generate the blank node, wrap it in a Box, leak it, and convert to an immutable reference
            //             let leaked_node: &mut _ = Box::leak(Box::new(generate_blank_node(counter)));
            //         &*leaked_node // Convert from &mut BlankNode to &BlankNode
            //     }));
            // } else {
            // }

            // get the subject
            // Check if `@id` exists and create a `NamedNode`; otherwise, use the `blank_node` as the subject.
            let subject = if let Some(id_value) = node.get("@id").and_then(|v| v.as_str()) {
                NamedOrBlankNode::from(NamedNode::new_unchecked(id_value.to_string()))
            } else {
                let mut blank_node = blank_node.unwrap_or_else(|| {
                    // Generate the blank node, wrap it in a Box, leak it, and convert to an immutable reference
                    let leaked_node: &mut _ = Box::leak(Box::new(generate_blank_node(counter)));
                    &*leaked_node // Convert from &mut BlankNode to &BlankNode
                });

                NamedOrBlankNode::BlankNode(blank_node.clone())
            };

            if let Some(properties) = node.as_object() {
                for (predicate_str, objects) in properties {
                    // println!("Node Iteration {} {:?}", predicate_str, objects);
                    if predicate_str == "@type"  {
                        let predicate = NamedNode::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")?;
                        match objects {
                            Value::Array(values) => {
                                for value in values {
                                    let object = process_value(value, graph, blank_node, counter)?;
                                    let triple = Triple::new(
                                        subject.clone(), // Ensure subject is a NamedNode Term
                                        predicate.clone(),
                                        object
                                    );
                                    graph.insert(&triple);
                                }
                            }
                            value => {
                                let object = process_value(value, graph, blank_node, counter)?;
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
                                    let object = process_value(value, graph, blank_node, counter)?;
                                    let triple = Triple::new(
                                        subject.clone(), // Ensure subject is a NamedNode Term
                                        predicate.clone(),
                                        object
                                    );
                                    graph.insert(&triple);
                                }
                            }
                            value => {
                                let object = process_value(value, graph, blank_node, counter)?;
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
        
        fn extract_blank_node_id(subject: &str) -> Option<String> {
            if subject.starts_with("_:b") {
                // Extract the part after "_:b" and attempt to parse it as an integer
                let remaining = subject.trim_start_matches("_:b");
                if let Ok(id_number) = remaining.parse::<i32>() {
                    if id_number == 0 {
                        // Return "0" if the id_number is 0
                        Some("0".to_string())
                    } else {
                        // Subtract 1 from the number and format it as a new ID
                        let new_id_number = id_number - 1;
                        Some(format!("b{}", new_id_number))
                    }
                } else {
                    // If parsing fails, return None
                    None
                }
            } else if subject.starts_with("_:") {
                // Extract the blank node ID as usual if it doesn't start with "_:b"
                Some(subject.trim_start_matches("_:").to_string())
            } else {
                None
            }
        }