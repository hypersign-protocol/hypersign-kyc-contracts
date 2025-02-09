use crate::ed25519_signature_2020::{
    decode_hex_message, transform_public_key, transfrom_signature,
};
use crate::urdna::normalize;
use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use lazy_static::lazy_static;
use locspan::Meta;
use nquads_syntax::Parse;
use rdf_types::LexicalQuad;
use serde_json::{self, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::error::Error;

lazy_static! {
    static ref CONTEXT_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "verificationMethod",
            "https://w3id.org/security#verificationMethod",
        );
        m
    };
}

// Define a struct representing a triple
#[derive(Default, Clone)]
struct Triple {
    subject: String,
    predicate: String,
    object: String,
}

#[derive(Default, Clone)]
struct Quad {
    subject: String,
    predicate: String,
    object: String,
    graph_name: String,
}

impl Quad {
    fn to_string(&self) -> String {
        if self.graph_name.is_empty() {
            format!("{} {} {} .", self.subject, self.predicate, self.object)
        } else {
            format!(
                "{} {} {} {} .",
                self.subject, self.predicate, self.object, self.graph_name
            )
        }
    }
}

type Graph = Vec<Triple>;

/// Hashes a string using SHA-256
pub fn get_did_value(doc: &Value) -> String {
    let did = doc[0]
        .get("@id")
        .and_then(Value::as_str)
        .unwrap_or("Unknown");
    did.to_string()
}

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
    return parts.last().unwrap_or(&"");
}

// Helper function to find an object in a JSON array based on a key-value pair
fn find_object_by_key_value<'a>(
    array: &'a Vec<Value>,
    key: &'a str,
    value: Value,
) -> Option<&'a Value> {
    array.iter().find(|obj| obj.get(key) == Some(&value))
}

// Handle only @id
pub fn get_verification_id(doc: String) -> String {
    let value_to_find = "https://w3id.org/security#verificationMethod";
    let json_value_proof: Value = serde_json::from_str(&doc).expect("Failed");
    let verification_id_value = json_value_proof[0].get(value_to_find).unwrap();
    let verification_id = verification_id_value[0].get("@id");

    verification_id.expect("REASON").to_string()
}

// Handle only @id
pub fn get_public_key(id: String, did_doc: String) -> String {
    let mut public_key = None;
    let json_value: Value = serde_json::from_str(&did_doc).expect("Failed");

    // Get Verification Method from the proof
    let value_to_find = "https://w3id.org/security#verificationMethod";
    let key = CONTEXT_MAP
        .iter()
        .find(|&(_, &v)| v == value_to_find)
        .map(|(k, _)| k);

    // Get the public key from the did doc
    let verification_method = json_value[0].get(value_to_find).unwrap();

    // Ensure the value is an array
    if let Some(array) = verification_method.as_array() {
        // Define the key and value we are searching for
        let key_to_find = "@id";
        let value_to_find = id.trim_matches('"');

        // Use the helper function to find the matching object
        if let Some(matching_object) = find_object_by_key_value(
            array,
            key_to_find,
            serde_json::Value::String(value_to_find.to_string()),
        ) {
            // Output the matching object
            let obj = matching_object
                .get("https://w3id.org/security#publicKeyMultibase")
                .unwrap();
            public_key = Some(obj[0].get("@value").unwrap());
        } else {
            println!("No matching object found");
        }
    } else {
        println!("Expected a JSON array");
    }

    let binding = Value::String("".to_owned());
    let mut value = public_key.unwrap_or(&binding);
    value
        .as_str()
        .expect("REASON")
        .trim_matches('"')
        .to_string()
}

// pub fn get_public_key(id: String, did_doc: String) -> String {

//     let mut public_key = None;
//     let json_value: Value = serde_json::from_str(&did_doc).expect("Failed");
//     let json_value_proof: Value = serde_json::from_str(&did_doc_proof).expect("Failed");

//     // Get Verification Method from the proof
//     let value_to_find = "https://w3id.org/security#verificationMethod";
//     let key = CONTEXT_MAP.iter()
//                         .find(|&(_, &v)| v == value_to_find)
//                         .map(|(k, _)| k);
//     let verification_id_value =  json_value_proof[0].get(value_to_find).unwrap();
//     let verification_id = verification_id_value[0].get("@id");

//     // Get the public key from the did doc
//     let verification_method = json_value.get(value_to_find).unwrap();

//     // Ensure the value is an array
//      if let Some(array) = verification_method.as_array() {
//         // Define the key and value we are searching for
//         let key_to_find = "@id";
//         let value_to_find = verification_id;

//         // Use the helper function to find the matching object
//         if let Some(matching_object) = find_object_by_key_value(array, key_to_find, value_to_find.unwrap().clone()) {
//             // Output the matching object
//             let obj = matching_object.get("https://w3id.org/security#publicKeyMultibase").unwrap();
//             public_key = Some(obj[0].get("@value").unwrap());
//         } else {
//             println!("No matching object found");
//         }
//     } else {
//         println!("Expected a JSON array");
//     }

//     let binding = Value::String("".to_owned());
//     let mut value = public_key.unwrap_or(&binding);
//     value.as_str().expect("REASON").trim_matches('"').to_string()
// }

// https://w3c.github.io/vc-di-eddsa/#transformation-ed25519signature2020
pub fn get_cannonized_str(json_str: String) -> String {
    let json_value: Value = serde_json::from_str(&json_str).expect("Failed");
    let graph = convert_expanded_jsonld_to_graph(&json_value).expect("Failed");

    // Convert triples to quads (adding a default graph name)
    let mut quads: &mut Vec<Quad> = &mut graph
        .into_iter()
        .map(
            |Triple {
                 subject,
                 predicate,
                 object,
             }| {
                let graph_name = if subject.to_string().starts_with("_:b0") {
                    Some("".to_string())
                } else if subject.to_string().starts_with("_:") {
                    // Use the blank node identifier as the graph name
                    extract_blank_node_id(&subject.to_string())
                } else {
                    Some("".to_string())
                };

                Quad {
                    subject,
                    predicate,
                    object,
                    graph_name: graph_name.expect("Graph Name").to_string(),
                }
            },
        )
        .collect();

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
    let mut nquads_output = collect_quads(quads);
    let nq_string = String::from_utf8(nquads_output.into()).unwrap();

    // Parse the N-Quad string back to a dataset
    let dataset = nquads_syntax::Document::parse_str(&nq_string).unwrap();
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

    return normalized;
}

// Function to generate a unique blank node identifier
fn generate_blank_node(counter: &mut usize) -> String {
    let blank_node = format!("_:b{}", counter);
    *counter += 1;
    blank_node
}

fn convert_expanded_jsonld_to_graph(jsonld: &Value) -> Result<Graph, Box<dyn Error>> {
    let mut graph: Graph = Vec::new();
    let mut counter: usize = 0;
    if let Some(nodes) = jsonld.as_array() {
        for node in nodes {
            if let Some(subject_id) = node.get("@id").and_then(Value::as_str) {
                let blank_node = generate_blank_node(&mut counter);
                process_node(&mut graph, node, Some(&blank_node), &mut counter)?;
            } else {
                process_node(&mut graph, node, None, &mut counter)?;
            }
        }
    }

    Ok(graph)
}

fn process_value(
    value: &Value,
    graph: &mut Graph,
    blank_node: Option<&String>,
    counter: &mut usize,
) -> Result<String, Box<dyn Error>> {
    match value {
        Value::String(s) => {
            // Treat URIs as NamedNode and other strings as Literal
            if s.starts_with('_') {
                // let named_node = BlankNode::new(s)?;
                Ok(s.to_string())
            } else if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("did:")
            {
                // let named_node = NamedNode::new(s)?;
                Ok(s.to_string())
            } else {
                // let literal = Literal::new_simple_literal(s);
                Ok(s.to_string())
            }
        }
        Value::Object(obj) => {
            // Check for @id and create NamedNode if present
            if let Some(id) = obj.get("@id").and_then(Value::as_str) {
                let named_node = id;

                // Handle other object properties
                let mut triples = Vec::new();
                for (key, value) in obj {
                    if key != "@id" && key != "@value" && key != "@type" && key != "@graph" {
                        let predicate = key;
                        match value {
                            Value::Array(array) => {
                                for item in array {
                                    let term = process_value(item, graph, blank_node, counter)?;
                                    triples.push((predicate.to_string(), term));
                                }
                            }
                            _ => {
                                let term = process_value(value, graph, blank_node, counter)?;
                                triples.push((predicate.to_string(), term));
                            }
                        }
                    } else if key == "@type" {
                        // Check if value exists
                        if obj.get("@value").is_none() {
                            // Perform your logic here when @value exists
                            let predicate = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
                            if let Some(array) = value.as_array() {
                                for (index, item) in array.iter().enumerate() {
                                    let term = process_value(item, graph, blank_node, counter)?;
                                    triples.push((format!("{}", predicate.to_string()), term));
                                }
                            }
                        }
                    }
                }

                // Insert triples into the graph
                for (predicate, term) in triples {
                    let placeholder_subject = "http://example.org/subject2";
                    let triple = Triple {
                        subject: wrap_if_needed(&named_node),
                        predicate: wrap_if_needed(&predicate),
                        object: wrap_if_needed(&term),
                    };
                    graph.push(triple);
                }

                return Ok(named_node.to_string());
            }

            // Handle @value for literals
            if let Some(value) = obj.get("@value").and_then(Value::as_str) {
                if let Some(type_str) = obj.get("@type").and_then(Value::as_str) {
                    // Concatenate value and type without quotes
                    let literal_value = format!("\"{}\"^^<{}>", value, type_str);
                    return Ok(literal_value.to_string());
                } else {
                    // Handle case where @type is missing
                    return Ok(format!("\"{}\"", value));
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
                let mut nested_graph: Graph = Vec::new();

                let blank_node_new = generate_blank_node(counter);
                for nested_node in graph_array {
                    process_node(
                        &mut nested_graph,
                        nested_node,
                        Some(&blank_node_new),
                        counter,
                    )?;
                }
                // Insert all nested triples into the main graph
                for triple in nested_graph.iter() {
                    graph.push(triple.clone());
                }
                return Ok(blank_node.expect("REASON").to_string());
            }

            Ok(format!("{}", blank_node.unwrap()))
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
                let subject = "http://example.org/array1";
                let predicate = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
                let triple = Triple {
                    subject: subject.to_string(),
                    predicate: predicate.to_string(),
                    object: term.to_string(),
                };
                graph.push(triple);
            }
            // Return a term for the array as a whole if needed
            Ok("http://example.org/array2".to_string())
        }
        _ => Err("Unsupported JSON-LD value".into()),
    }
}

fn process_node(
    graph: &mut Graph,
    node: &Value,
    blank_node: Option<&String>,
    counter: &mut usize,
) -> Result<(), Box<dyn Error>> {
    // get the subject
    // Check if `@id` exists and create a `NamedNode`; otherwise, use the `blank_node` as the subject.
    let subject = if let Some(id_value) = node.get("@id").and_then(|v| v.as_str()) {
        id_value.to_string()
    } else {
        // Determine which blank node to use
        let blank_node = blank_node.map_or_else(
            || {
                // Generate a new blank node if blank_node is None
                let new_blank_node = generate_blank_node(counter);
                // Add the new node to the graph or handle as needed
                new_blank_node
            },
            |b| b.clone(),
        ); // Use the existing blank node otherwise

        blank_node
    };

    if let Some(properties) = node.as_object() {
        for (predicate_str, objects) in properties {
            if predicate_str == "@type" {
                let predicate = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
                match objects {
                    Value::Array(values) => {
                        for value in values {
                            let object = process_value(value, graph, blank_node, counter)?;
                            let triple = Triple {
                                subject: wrap_if_needed(&subject), // Ensure subject is a NamedNode Term
                                predicate: wrap_if_needed(&predicate),
                                object: wrap_if_needed(&object),
                            };
                            graph.push(triple);
                        }
                    }
                    value => {
                        let object = process_value(value, graph, blank_node, counter)?;
                        let triple = Triple {
                            subject: wrap_if_needed(&subject), // Ensure subject is a NamedNode Term
                            predicate: wrap_if_needed(&predicate),
                            object: wrap_if_needed(&object),
                        };
                        graph.push(triple);
                    }
                }
            } else if predicate_str != "@id" {
                let predicate = predicate_str;
                match objects {
                    Value::Array(values) => {
                        for value in values {
                            let object = process_value(value, graph, blank_node, counter)?;
                            let triple = Triple {
                                subject: wrap_if_needed(&subject), // Ensure subject is a NamedNode Term
                                predicate: wrap_if_needed(&predicate),
                                object: wrap_if_needed(&object),
                            };
                            graph.push(triple);
                        }
                    }
                    value => {
                        let object = process_value(value, graph, blank_node, counter)?;
                        let triple = Triple {
                            subject: wrap_if_needed(&subject), // Ensure subject is a NamedNode Term
                            predicate: predicate.to_string(),
                            object: object,
                        };
                        graph.push(triple);
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

fn wrap_if_needed(value: &str) -> String {
    if value.starts_with('_') || value.contains("^^") || value.starts_with("\"") {
        value.to_string()
    } else {
        format!("<{}>", value)
    }
}

fn collect_quads(quads: &[Quad]) -> String {
    quads
        .iter()
        .map(|quad| quad.to_string())
        .collect::<Vec<String>>()
        .join("\n") // Join all triple strings with a newline
}
