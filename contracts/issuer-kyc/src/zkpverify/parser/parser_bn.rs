use crate::zkpverify::types::{ProofStr, VkeyStr};
use bellman_ce::groth16::{Proof, VerifyingKey};
use pairing_ce::bn256::{G1Affine, G1Uncompressed, G2Affine, G2Uncompressed};
use pairing_ce::{CurveAffine, EncodedPoint, Engine};
use std::fs;
use std::path::PathBuf;

pub fn parse_bn_proof<E>(proof_str: String) -> Proof<E>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
    let pof: ProofStr = serde_json::from_str(&proof_str).unwrap();

    //serde_json::from_str("").unwrap();

    let pi_a = pof.pi_a;
    let pi_b = pof.pi_b;
    let pi_c = pof.pi_c;

    let mut a_arr: [u8; 64] = [0; 64];
    let mut b_arr: [u8; 128] = [0; 128];
    let mut c_arr: [u8; 64] = [0; 64];

    a_arr[..pi_a.len()].copy_from_slice(&pi_a[..]);

    b_arr[..pi_b.len()].copy_from_slice(&pi_b[..]);

    c_arr[..pi_c.len()].copy_from_slice(&pi_c[..]);

    let pia_affine: G1Affine = G1Uncompressed::from_fixed_bytes(a_arr)
        .into_affine()
        .unwrap();
    let pib_affine: G2Affine = G2Uncompressed::from_fixed_bytes(b_arr)
        .into_affine()
        .unwrap();
    let pic_affine: G1Affine = G1Uncompressed::from_fixed_bytes(c_arr)
        .into_affine()
        .unwrap();

    Proof {
        a: pia_affine,
        b: pib_affine,
        c: pic_affine,
    }
}

pub fn parse_bn_vkey<E>(proof_type: String) -> VerifyingKey<E>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.pop();
    // config_path.push("contracts");
    config_path.push("issuer-kyc");
    config_path.push("src");
    config_path.push("zkpverify");
    config_path.push("hypersign-circuits");
    config_path.push(proof_type.to_string()); // make it dynamic
    config_path.push("ver_key.json");

    // print!("Path = {:?}", config_path.as_path());
    let str = fs::read_to_string(config_path.as_path()).unwrap();
    let vk: VkeyStr = serde_json::from_str(&str).unwrap();

    let vk_alpha_1 = vk.alpha_1;
    let vk_beta_2 = vk.beta_2;
    let vk_gamma_2 = vk.gamma_2;
    let vk_delta_2 = vk.delta_2;
    let vk_ic = vk.ic;

    let mut alpha1: [u8; 64] = [0; 64];
    let mut beta2: [u8; 128] = [0; 128];
    let mut gamma2: [u8; 128] = [0; 128];
    let mut delta2: [u8; 128] = [0; 128];

    // Copying from vk values to the corresponding arrays
    alpha1[..vk_alpha_1.len()].copy_from_slice(&vk_alpha_1[..]);
    beta2[..vk_beta_2.len()].copy_from_slice(&vk_beta_2[..]);
    gamma2[..vk_gamma_2.len()].copy_from_slice(&vk_gamma_2[..]);
    delta2[..vk_delta_2.len()].copy_from_slice(&vk_delta_2[..]);

    // Affine conversions
    let alpha1_affine = G1Uncompressed::from_fixed_bytes(alpha1)
        .into_affine()
        .unwrap();
    let beta2_affine = G2Uncompressed::from_fixed_bytes(beta2)
        .into_affine()
        .unwrap();
    let gamma2_affine = G2Uncompressed::from_fixed_bytes(gamma2)
        .into_affine()
        .unwrap();
    let delta2_affine = G2Uncompressed::from_fixed_bytes(delta2)
        .into_affine()
        .unwrap();

    // Dynamic handling of vk_ic array
    let mut ic = Vec::new();
    for ic_bytes in vk_ic {
        let mut ic_buffer: [u8; 64] = [0; 64];
        ic_buffer[..ic_bytes.len()].copy_from_slice(&ic_bytes[..]);
        let ic_affine = G1Uncompressed::from_fixed_bytes(ic_buffer)
            .into_affine()
            .unwrap();
        ic.push(ic_affine);
    }

    VerifyingKey {
        alpha_g1: alpha1_affine,
        beta_g1: G1Affine::zero(),
        beta_g2: beta2_affine,
        gamma_g2: gamma2_affine,
        delta_g1: G1Affine::zero(),
        delta_g2: delta2_affine,
        ic,
    }
}
