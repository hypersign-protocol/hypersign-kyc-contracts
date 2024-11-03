pub mod msg;
mod parser;
pub mod types;
use crate::msg::{HsZkProof, HypersignKYCProofTypes};
use bellman_ce::domain::Scalar;
use bellman_ce::groth16::{prepare_verifying_key, verify_proof};
use ff_ce::PrimeField as Frce;
use pairing_ce::bn256::Bn256;
use parser::parser_bn::{parse_bn_proof, parse_bn_vkey};
use std::result;

pub fn verify_zkp(
    proof_str: HsZkProof,
    inputs: Vec<String>,
    proof_type: HypersignKYCProofTypes,
) -> Result<bool, bellman_ce::SynthesisError> {
    let pof = parse_bn_proof::<Bn256>(proof_str);
    let vk = parse_bn_vkey::<Bn256>(proof_type);
    let pvkv = prepare_verifying_key(&vk);

    let public_input1: Vec<bellman_ce::bn256::Fr> =
        inputs.iter().map(|x| Frce::from_str(&x).unwrap()).collect();

    let public_input: &[bellman_ce::bn256::Fr] = public_input1.as_slice();

    let result = verify_proof(&pvkv, &pof, &public_input);

    return result;
}
