use crate::lib_json_ld::{self, Urdna2015};
use cosmwasm_std::{Api, Deps, DepsMut};
use multibase::Base;
// use serde::de::value::Error;
use sha2::{Digest, Sha256};
// use std::{io::Read, result};

pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const SIGNATURE_BYTE_SIZE: usize = 64;

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
    let message = decode_hex_message(&m);
    let signature_array = transfrom_signature(&signature_str1);
    let public_key = transform_public_key(&public_key_str);

    let result = deps_api
        .ed25519_verify(&message, &signature_array, &public_key)
        .unwrap();

    deps_api.debug("Verification result ===========");
    deps_api.debug(&result.to_string());
    deps_api.debug("Verification result ===========");

    println!("verify_proof result {:?}", result);
    return result;
}

// Algorithm: // https://w3c.github.io/vc-di-eddsa/#hashing-eddsa-rdfc-2022
pub fn transform_proof_message(did_doc: &str, did_doc_proof: &str) -> String {
    // let did_string = r#did_doc.to_string();

    let did_string = r#"
    {"@context":["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","controller":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"alsoKnownAs":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"verificationMethod":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"Ed25519VerificationKey2020","controller":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","publicKeyMultibase":"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}],"authentication":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"assertionMethod":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"keyAgreement":[],"capabilityInvocation":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"capabilityDelegation":[],"service":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"LinkedDomains","serviceEndpoint":"https://www.linkeddomains.com"}]}
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
