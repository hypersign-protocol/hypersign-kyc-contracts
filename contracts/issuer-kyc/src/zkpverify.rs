mod parser;
mod types;

use std::result;

use bellman_ce::domain::Scalar;
use parser::parser_bn::{parse_bn_proof, parse_bn_vkey};

use bellman_ce::groth16::{prepare_verifying_key, verify_proof};
use ff_ce::PrimeField as Frce;
use pairing_ce::bn256::Bn256;
pub fn verify_zkp(
    proof_str: String,
    inputs: [&str; 6],
    proof_type: String,
) -> Result<bool, bellman_ce::SynthesisError> {
    let pof = parse_bn_proof::<Bn256>(proof_str.to_string());
    let vk = parse_bn_vkey::<Bn256>(proof_type.to_string());
    let pvkv = prepare_verifying_key(&vk);

    // Convert the string values to Frce using from_str or similar method
    let public_input = [
        Frce::from_str(inputs[0]).unwrap(),
        Frce::from_str(inputs[1]).unwrap(),
        Frce::from_str(inputs[2]).unwrap(),
        Frce::from_str(inputs[3]).unwrap(),
        Frce::from_str(inputs[4]).unwrap(),
        Frce::from_str(inputs[5]).unwrap(),
    ];

    // let public_input = [Frce::from_str("30").unwrap()];
    print!("{:?}\n", public_input);

    // let public_input = [Frce::from_str("").unwrap()];
    // print!("{:?}\n", public_input);

    // let result =
    let result = verify_proof(&pvkv, &pof, &public_input);
    print!("result is {:?}\n", result);
    return result;
}
