use std::fmt::format;
use std::hash::Hash;

use crate::error::KycContractError;
use crate::lib_json_ld::{extract_after_last_delimiter, hash_string, get_cannonized_str, get_public_key, get_verification_id};
use cosmwasm_std::{Api, Deps, DepsMut};
use multibase::Base;

use sha2::{Digest, Sha256};
use serde_json::{json, Value};
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const SIGNATURE_BYTE_SIZE: usize = 64;

use cosmwasm_std::{StdError, Response, StdResult, MessageInfo, Env};

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

pub fn transfrom_signature(signature_str: &str) -> [u8; SIGNATURE_BYTE_SIZE] {
    let signature_bytes = decode_multibase_public_key(signature_str).unwrap();
    let signature_array = vec_to_array::<SIGNATURE_BYTE_SIZE>(signature_bytes.to_owned()).unwrap();
    return signature_array;
}

pub fn decode_hex_message(message: &str) -> Vec<u8> {
    let hex_decode_message: Vec<u8> = hex::decode(message).unwrap();
    return hex_decode_message;
}

// fn parse_uri_get_fragment(uri: &str) -> Result<String, KycContractError> {
//     if uri.contains("did:hid") {
//         Ok(uri.to_string())
//     } else {
//         // Parse the URI
//         let parsed_uri = Url::parse(uri)?;

//         // Get the fragment and convert Option to Result
//         match parsed_uri.fragment() {
//             Some(f) => match Some(f) {
//                 Some("type") => Ok("@type".to_string()),
//                 Some(other) => Ok(other.to_string()),
//                 None => Err(KycContractError::FragmentNotFound),
//             },
//             None => match parsed_uri.path_segments() {
//                 Some(segment) => match segment.last() {
//                     Some(seg) => Ok(seg.to_string()),
//                     None => Err(KycContractError::FragmentNotFound),
//                 },
//                 None => Err(KycContractError::FragmentNotFound),
//             },
//         }
//     }
// }

pub fn verify_proof(
    public_key_str: &str,
    m: &str,
    signature_str: &str,
    deps_api: &dyn Api,
) -> bool {
    /// Redundant code for generating hash...
    let hash = Sha256::digest(m);
    let message1: &[u8] = hash.as_ref();
    let hash_hex = hex::encode(message1);
 
    let message = decode_hex_message(&m);
    let signature_array = transfrom_signature(&signature_str);
    let public_key = transform_public_key(&public_key_str);

    let result = deps_api
        .ed25519_verify(&message, &signature_array, &public_key)
        .unwrap();

    println!("verify_proof result {:?}", result);
    return result;
}

pub fn verify(
    did_doc: String,
    did_doc_proof: String,
    signature: String,
    deps: &DepsMut,
) -> StdResult<bool> {

    // Get pubkey
    let verification_id = get_verification_id(did_doc_proof.clone());
    let public_key = get_public_key(verification_id, did_doc.clone());

    let cannonized_did  = get_cannonized_str(did_doc.to_string());
    let cannonized_did_proof  = get_cannonized_str(did_doc_proof.to_string());

    let m1 = hash_string(&cannonized_did);
    let m2 = hash_string(&cannonized_did_proof);

    // Get the signature from the did proof
    let message = [m2.clone(), m1.clone()].concat();

    
    let result = try_verify_signature(
                    public_key.to_string(), 
                    message.to_string(), 
                    signature.to_string(), 
                    &deps
                );
    
    result
}

pub fn verify_signature(
    public_key: String,
    message: String,
    signature: String,
    deps: &DepsMut,
) -> StdResult<Response> {
    // Call the try_verify_signature function, which returns a bool
    match try_verify_signature(public_key, message, signature, &deps) {
        Ok(is_valid) => {
            if is_valid {
                // If valid, return a response with a success attribute
                Ok(Response::new().add_attribute("verification", "true"))
            } else {
                // If invalid, return a response with a failure attribute
                Ok(Response::new().add_attribute("verification", "false"))
            }
        }
        Err(err) => {
            // If there's an error, propagate it as a StdError
            Err(StdError::generic_err(format!(
                "Verification failed: {:?}", err
            )))
        }
    }
}


pub fn try_verify_signature(
    public_key: String,
    message: String,
    signature: String, 
    _deps: &DepsMut
) -> StdResult<bool> {
    
    // Decode and transform inputs
    let message_arr = decode_hex_message(&message);
    let signature_array = transfrom_signature(&signature);
    let public_key_arr = transform_public_key(&public_key);

    // Perform signature verification
    match _deps.api.ed25519_verify(&message_arr, &signature_array, &public_key_arr) {
        Ok(is_valid) => {
            if is_valid {
                Ok(true)  // Signature is valid
            } else {
                Ok(false)  // Signature is invalid
            }
        }
        Err(err) => Err(StdError::generic_err(format!(
            "Verification error: {:?}", err
        ))),  // Verification error
    }
}