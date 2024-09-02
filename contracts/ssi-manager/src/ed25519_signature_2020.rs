use std::fmt::format;
use std::hash::Hash;

use crate::error::KycContractError;
use crate::lib_json_ld::{self, Urdna2015};
use cosmwasm_std::{Api, Deps, DepsMut};
use multibase::Base;

use sha2::{Digest, Sha256};
use serde_json::json;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const SIGNATURE_BYTE_SIZE: usize = 64;

use rdf::node::Node;
use rdf::reader::rdf_parser::RdfParser;
use rdf::reader::turtle_parser::TurtleParser;
use url::{Host, Position, Url};

use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use cosmwasm_std::{StdError, Response, StdResult, MessageInfo, Env};
use cosmwasm_crypto::ed25519_verify;

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
    let signature_array = vec_to_array::<SIGNATURE_BYTE_SIZE>(signature_bytes.to_owned()).unwrap();
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
 
    let message = decode_hex_message(&m);
    let signature_array = transfrom_signature(&signature_str1);
    let public_key = transform_public_key(&public_key_str);

    let result = deps_api
        .ed25519_verify(&message, &signature_array, &public_key)
        .unwrap();

    println!("verify_proof result {:?}", result);
    return result;
}

pub fn try_verify_signature(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    public_key: String,
    message: String,
    signature: String,
) -> StdResult<Response> {
    
    let message_arr = decode_hex_message(&message);
    let signature_array = transfrom_signature(&signature);
    let public_key_arr = transform_public_key(&public_key);

    match ed25519_verify(&message_arr, &signature_array, &public_key_arr) {
        Ok(is_valid) => {
            if is_valid {
                Ok(Response::new().add_attribute("verification", "success"))
            } else {
                Err(StdError::generic_err("Invalid signature"))
            }
        }
        Err(err) => Err(StdError::generic_err(format!(
            "Verification error: {:?}",
            err
        ))),
    }
}
