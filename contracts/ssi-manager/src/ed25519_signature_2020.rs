use std::fmt::format;
use std::hash::Hash;

use crate::error::KycContractError;
use crate::lib_json_ld::{self, Urdna2015};
use cosmwasm_std::{Api, Deps, DepsMut};
use multibase::Base;

// use serde::de::value::Error;
use sha2::{Digest, Sha256};
// use std::{io::Read, result};
use serde_json::json;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const SIGNATURE_BYTE_SIZE: usize = 64;

use rdf::node::Node;
use rdf::reader::rdf_parser::RdfParser;
use rdf::reader::turtle_parser::TurtleParser;
// use rdf::writer::turtle_writer;
use url::{Host, Position, Url};

use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use sophia_api::serializer::TripleSerializer;

// use crate::msg::DIDDocumentProof;

fn decode_multibase_public_key(multibase_str: &str) -> Result<Vec<u8>, String> {
    let decoded = multibase::decode(multibase_str).unwrap();
    let (base, data) = decoded;
    match base {
        Base::Base58Btc => {
            // println!("Decoded data (Base58btc): {:?}", data);
            //println!("Decoded data (Base58btc) vec: {:?}", data.to_vec());
        }
        _ => {
            println!("Unsupported base: {:?}", base);
        }
    }

    Ok(data)
}

fn vec_to_array<const N: usize>(input: Vec<u8>) -> Result<[u8; N], &'static str> {
    // Ensure the input Vec has the correct length for the array
    if input.len() != N {
        return Err("Input Vec length does not match the desired array length");
    }

    // Try to convert the Vec into a fixed-size array
    let mut array = [0u8; N];
    array.copy_from_slice(&input);

    Ok(array)
}

pub fn transform_public_key(public_key_str: &str) -> [u8; PUBLIC_KEY_LENGTH] {
    const ARRAY_LENGTH: usize = 34;
    let public_key_bytes = decode_multibase_public_key(public_key_str).unwrap();
    let t_public_key_array = vec_to_array::<ARRAY_LENGTH>(public_key_bytes.to_owned()).unwrap();
    println!("t_public_key_array.len {:?}", t_public_key_array.len());

    // extract secret key from index 2 to 32
    let public_key_start_index = 2;
    let public_key_end_index = t_public_key_array.len();
    let public_key_array = t_public_key_array[public_key_start_index..public_key_end_index]
        .try_into()
        .expect("Failed to create new array");

    return public_key_array;
}

pub fn transfrom_signature(signature_str1: &str) -> [u8; SIGNATURE_BYTE_SIZE] {
    let signature_bytes = decode_multibase_public_key(signature_str1).unwrap();
    println!("signature_bytes {:?}", signature_bytes.len());
    let signature_array = vec_to_array::<SIGNATURE_BYTE_SIZE>(signature_bytes.to_owned()).unwrap();
    println!("signature_str1_len {:?}", signature_array.len());
    return signature_array;
}

pub fn decode_hex_message(message: &str) -> Vec<u8> {
    let hex_decode_message: Vec<u8> = hex::decode(message).unwrap();
    return hex_decode_message;
}

fn parse_uri_get_fragment(uri: &str) -> Result<String, KycContractError> {
    if uri.contains("did:hid") {
        Ok(uri.to_string())
    } else {
        // Parse the URI
        let parsed_uri = Url::parse(uri)?;

        // Get the fragment and convert Option to Result
        // let fragment = parsed_uri.fragment();
        match parsed_uri.fragment() {
            Some(f) => match Some(f) {
                Some("type") => Ok("@type".to_string()),
                Some(other) => Ok(other.to_string()),
                None => Err(KycContractError::FragmentNotFound),
            },
            None => match parsed_uri.path_segments() {
                Some(segment) => match segment.last() {
                    Some(seg) => Ok(seg.to_string()),
                    None => Err(KycContractError::FragmentNotFound),
                },
                None => Err(KycContractError::FragmentNotFound),
            },
        }
    }
}

pub fn transform_rdf_to_json_ld(
    deps_api: &dyn Api,
    //rdf_str: &str,
) -> Result<String, KycContractError> {
    let input = r#"_:c14n0 <http://purl.org/dc/terms/created> "2024-05-09T08:01:46Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
_:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> .
_:c14n0 <https://w3id.org/security#challenge> "1231231231" .
_:c14n0 <https://w3id.org/security#domain> "www.adbv.com" .
_:c14n0 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#authenticationMethod> .
_:c14n0 <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> .
"#;

    let mut reader = TurtleParser::from_string(input.to_string());
    let graph = reader.decode().unwrap();

    // assert_eq!(graph.count(), 4);
    // TripleSerializer::new()
    // let writer = NTriplesWriter::new();
    // let t = writer.write_to_string(&graph).unwrap();
    // print!("{:?}", t);
    /**
    let subject = Node::BlankNode { id: "a".to_string() };
    let predicate = Node::UriNode { uri: Uri::new("http://example.org/show/localName".to_string()) } ;
    let object = Node::BlankNode { id: "b".to_string() };
    Triple::new(&subject, &predicate, &object);
    */
    let mut jsonld = serde_json::json!({});
    for triple in graph.triples_iter() {
        deps_api.debug("------------ granf node  ----");

        let subject: &Node = triple.subject();
        let predicate: &Node = triple.predicate();
        let object: &Node = triple.object();

        let subject_str = match subject {
            Node::BlankNode { id } => format!("{}", id),
            Node::LiteralNode {
                literal,
                data_type,
                language,
            } => format!("{}", literal),
            Node::UriNode { uri } => format!("{}", uri.to_string()),
            _ => continue,
        };

        deps_api.debug(&subject_str.to_string());

        let predicate_str = match predicate {
            Node::BlankNode { id } => {
                deps_api.debug("predicate is blank node");
                format!("{}", id)
            }
            Node::LiteralNode {
                literal,
                data_type,
                language,
            } => {
                deps_api.debug("predicate is literal node");
                format!("{}", literal)
            }
            Node::UriNode { uri } => {
                deps_api.debug("predicate is uri node");
                parse_uri_get_fragment(&uri.to_string())?
            } // format!("{}", uri.to_string()),
            _ => continue,
        };
        deps_api.debug(&predicate_str.to_string());

        match object {
            Node::BlankNode { id } => {
                deps_api.debug("Object is blank node");
                jsonld[predicate_str] = serde_json::json!({ "@id-blank": id.to_string() });
            }
            Node::LiteralNode {
                literal,
                data_type,
                language,
            } => {
                deps_api.debug("Object is litrel node");
                jsonld[predicate_str] = serde_json::json!(literal.to_string());
            }
            Node::UriNode { uri } => {
                // let url_parsed = Url::parse(uri.to_string())?;
                deps_api.debug("Object is uri node");

                if predicate_str.to_owned() == "@type" {
                    jsonld[predicate_str] =
                        serde_json::json!(parse_uri_get_fragment(uri.to_string())?)
                } else {
                    jsonld[predicate_str] = serde_json::json!(uri.to_string())
                }

                // jsonld[predicate_str] = serde_json::json!(uri.to_string());
            } //   _ => continue,
        }
    }

    let m = serde_json::to_string_pretty(&jsonld).unwrap();
    deps_api.debug(&m);

    // let proof: DIDDocumentProof = serde_json::from_str(&m).unwrap();
    // // Access properties
    // deps_api.debug(&proof.challenge);

    return Ok(m);
}

pub fn verify_proof(
    public_key_str: &str,
    m: &str,
    signature_str1: &str,
    deps_api: &dyn Api,
) -> bool {
    /// Redundant code for generating hash...
    let hash = Sha256::digest(m);
    let message1: &[u8] = hash.as_ref();
    let hash_hex = hex::encode(message1);
    deps_api.debug("Message HASH ===========");
    deps_api.debug(&hash_hex);
    deps_api.debug("Message HASH ===========");
    /// Redundant code for generating hash...
    ///
    transform_rdf_to_json_ld(deps_api);
    ///
    // deps_api.debug("Message ===========");
    // deps_api.debug(&m);
    // deps_api.debug("Message ===========");
    let message = decode_hex_message(&m);
    let signature_array = transfrom_signature(&signature_str1);
    let public_key = transform_public_key(&public_key_str);

    let result = deps_api
        .ed25519_verify(&message, &signature_array, &public_key)
        .unwrap();

    // deps_api.debug("Verification result ===========");
    // deps_api.debug(&result.to_string());
    // deps_api.debug("Verification result ===========");

    println!("verify_proof result {:?}", result);
    return result;
}

// Algorithm: // https://w3c.github.io/vc-di-eddsa/#hashing-eddsa-rdfc-2022
pub fn transform_proof_message(did_doc: &str, did_doc_proof: &str) -> String {
    // let did_string = r#did_doc.to_string();

    let did_string = r#"
    {"@context":["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","controller":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"alsoKnownAs":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"verificationMethod":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"Ed25519VerificationKey2020","controller":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","publicKeyMultibase":"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}],"authentication":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"assertionMethod":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"keyAgreement":[],"capabilityInvocation":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"capabilityDelegation":[]}]}
    "#;

    let t: Urdna2015 = lib_json_ld::get_urdna2015_normalized_str(&did_string);
    let did_doc_normalized_hash = t.value; //digest(&t.value);
    println!("did_doc_normalized_hash {:?}", did_doc_normalized_hash);

    let proof_string = r#"
        {
        "@context": [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/suites/ed25519-2020/v1"
        ],
        "type":"Ed25519Signature2020",
        "created":"2010-01-01T19:23:24Z",
        "verificationMethod":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1",
        "proofPurpose":"assertionMethod"
        }
    "#;

    println!("================================================");
    let u: Urdna2015 = lib_json_ld::get_urdna2015_normalized_str(&proof_string);

    let did_doc_proof_normalized_hash = u.value; // digest(&u.value);
    let message = format!(
        "{}{}",
        did_doc_proof_normalized_hash, did_doc_normalized_hash
    );

    return message;
}
