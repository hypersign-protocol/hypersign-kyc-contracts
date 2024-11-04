use crate::error::KycContractError;
use crate::state::{COUNTER, INSTANTIATE_TOKEN_REPLY_ID, NULLIFIER, OWNERDID, SBT_CODE_ID};
use crate::{msg::InstantiateMsg, state::*};
use cosmwasm_std::{
    to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response,
    StdError, StdResult, SubMsg, WasmMsg,
};
use hypersign_kyc_token::msg::ExecuteMsg;
use serde_json::{from_slice, from_str, Value};

pub fn instantiate(
    deps: DepsMut,
    msg: InstantiateMsg,
    info: MessageInfo,
    env: Env,
) -> Result<Response, KycContractError> {
    // TODO: implemnt check so that token_code_id is passed...
    // if msg.token_code_id.is_empty() {
    //     return Err(KycContractError::CodeIdRequired {});
    // }

    /// TODO: take DID signature as parameter...
    /// TODO: [optionally] We can reject issuer if their DID is not registerd.
    ///
    let cannonized_did_proof =
        ssi_manager::lib_json_ld::get_cannonized_str(msg.did_doc_proof.to_string());
    let cannonized_did_proof_hash = ssi_manager::lib_json_ld::hash_string(&cannonized_did_proof);
    let nullifier = NULLIFIER
        .load(deps.storage, &cannonized_did_proof_hash)
        .unwrap_or(0);
    if nullifier == 1 {
        return Err(KycContractError::ChallengeInvalid {});
    }

    /// TODO: verify DID signature
    match ssi_manager::ed25519_signature_2020::verify(
        msg.did_doc.to_owned(),
        msg.did_doc_proof.to_owned(),
        msg.signature.to_owned(),
        &deps,
    ) {
        Ok(is_valid) => {
            if is_valid {
                // let mut resp = Response::new();
                let did_json: Value = serde_json::from_str(&msg.did_doc).expect("Invalid JSON");
                let owner_did: String = ssi_manager::lib_json_ld::get_did_value(&did_json);

                if owner_did.is_empty() {
                    return Err(KycContractError::OnwerDIDRequired {});
                }
                // TODO check if owner_did is not empty

                // save the owner did
                OWNERDID.save(deps.storage, &owner_did)?;

                // save the owner
                //OWNER.save(deps.storage, &info.sender)?;

                // initiate the counter = 0
                COUNTER.save(deps.storage, &0)?;

                //    SBT_CODE_ID.save(deps.storage, &msg.token_code_id)?;
                // let sub_msg: Vec<SubMsg> = vec![SubMsg {
                //     msg: WasmMsg::Instantiate {
                //         code_id: msg.token_code_id,
                //         msg: to_json_binary(&Cw721InstantiateMsg {
                //             name: "SBT".to_owned(),
                //             symbol: "SBT".to_owned(),
                //             minter: env.contract.address.clone().to_string(), //
                //         })?,
                //         funds: vec![],
                //         admin: Some(info.sender.to_string()),
                //         label: String::from("Instantiate fixed price NFT contract"),
                //     }
                //     .into(),
                //     id: INSTANTIATE_TOKEN_REPLY_ID,
                //     gas_limit: None,
                //     reply_on: ReplyOn::Success,
                // }];

                // let resp = Response::new().add_submessages(sub_msg);
                // Ok(resp);
                // resp = resp.add_attribute("owner_did", owner_did.to_string());

                //// Nullifying the signature
                NULLIFIER.save(deps.storage, &cannonized_did_proof_hash, &1);

                Ok(Response::new())
            } else {
                // If invalid, return a response with a failure attribute
                // Ok(Response::new().add_attribute("verification", is_valid.to_string()))
                Err(KycContractError::SignatureMissmatch {})
            }
        }
        Err(err) => {
            // If there's an error, propagate it as a StdError
            Err(KycContractError::UnexpectedFailure {})
        }
    }
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{
        msg::{SBTcontractAddressResp, ValueResp},
        state::COUNTER,
        state::OWNERDID,
        state::SBT_CONTRACT_ADDRESS,
    };

    pub fn getOwnerDID(deps: Deps) -> StdResult<ValueResp> {
        Ok(ValueResp {
            owner_did: OWNERDID.load(deps.storage)?,
        })
    }

    pub fn getSbtContractAddress(deps: Deps) -> StdResult<SBTcontractAddressResp> {
        Ok(SBTcontractAddressResp {
            sbt_contract_address: SBT_CONTRACT_ADDRESS.load(deps.storage)?,
        })
    }
}

pub mod exec {
    use super::*;
    use super::{
        COUNTER, INSTANTIATE_TOKEN_REPLY_ID, SBT_CODE_ID, SBT_CONTRACT_ADDRESS, SBT_NAME,
        SBT_SYMBOL,
    };
    use cosmwasm_std::Empty;
    use hypersign_zk_verifier::msg::HypersignKYCProof;
    use strum_macros::ToString;

    pub type ExecuteMsg = hypersign_kyc_token::msg::ExecuteMsg;

    use crate::{
        error::KycContractError,
        msg::{CW721OnChainMetadataInstantiateMsg, ExecMsg},
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn,
        Response, StdError, StdResult, SubMsg, WasmMsg,
    };
    use hypersign_zk_verifier::msg::HypersignKYCProofTypes;

    use cw721::{msg::NftExtensionMsg, state::Trait, NftExtension};

    /**
     * Init
     */
    pub fn init(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_code_id: u64,
        label: Option<String>,
    ) -> Result<Response, KycContractError> {
        SBT_CODE_ID.save(deps.storage, &token_code_id)?;

        // Instantiating a new SBT contract
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: token_code_id,
                msg: to_json_binary(&CW721OnChainMetadataInstantiateMsg {
                    name: SBT_NAME.to_owned(),
                    symbol: SBT_SYMBOL.to_owned(),
                    minter: Some(env.contract.address.clone().to_string()),
                    creator: Some(env.contract.address.clone().to_string()),
                    withdraw_address: None,
                    collection_info_extension: None,
                })?,
                funds: vec![],
                admin: Some(env.contract.address.clone().to_string()),
                label: label.unwrap_or("Hypersign KYC Token".to_string()),
            }
            .into(),
            id: INSTANTIATE_TOKEN_REPLY_ID,
            gas_limit: None,
            reply_on: ReplyOn::Success,
        }];

        let resp = Response::new().add_submessages(sub_msg);
        Ok(resp)
    }

    /**
     * Mint
     * To mint SBT
     */
    pub fn mint(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        hypersign_proof: HypersignKYCProof,
    ) -> Result<Response, KycContractError> {
        let mut resp = Response::new();

        let prooftype = hypersign_proof.zk_proof.proof_type;

        /// TODO: verify nullifier to avoid replay attack.
        /// TODO: if the exposed did of issuer is same (issuer) as expected by this contract
        /// TODO: For Age criteria check if we get true in the public signal
        /// Verify the proof
        /// public_signales last 3: nullifier,issuer, holder, type
        match hypersign_zk_verifier::verify_zkp(
            hypersign_proof.zk_proof.proof,
            hypersign_proof.zk_proof.public_signales,
            prooftype.clone(),
        ) {
            Ok(result) => {
                if result {
                    println!("proof is verified");
                } else {
                    return Err(KycContractError::ZkProofVerificationFailure {});
                }
            }
            Err(_err) => {
                return Err(KycContractError::ZkProofFailure {
                    err: _err.to_string(),
                });
            }
        }

        ///

        /// Traits types
        /// Here trait_type is the name of the trait, value is the value of the trait,
        /// and display_type is a field indicating how you would like it to be displayed.
        /// For string traits, you don't have to worry about display_type.
        /// Creating all traits
        /// Trait - 1: proof-type
        let proof_type_trait = Trait {
            display_type: Some("Hypersign ZKProof Type".to_string()), // No display type
            trait_type: "proof-type".to_string(),
            value: prooftype.to_string(),
        };

        /// Trait - 2 [optional]: sbt-code - we probably dont need this
        let sbt_code_trait = Trait {
            display_type: Some("Hypersign SBTCode Type".to_string()),
            trait_type: "sbt-code".to_string(),
            value: prooftype.get_sbt_code().to_string(),
        };

        /// Trait - 3: Associated credential-id
        let credential_id_trait = Trait {
            display_type: Some("Hypersign Credential Id".to_string()),
            trait_type: "credential-id".to_string(),
            value: hypersign_proof.credential_id.unwrap_or("".to_string()),
        };

        /// Extensions
        let extension = Some(NftExtensionMsg {
            description: Some(prooftype.get_decription().to_string()),
            name: Some(prooftype.to_string()),
            image: Some(prooftype.get_logo().to_string()),
            background_color: Some(prooftype.get_color().to_string()),
            attributes: Some(vec![proof_type_trait, sbt_code_trait, credential_id_trait]),
            ..NftExtensionMsg::default()
        });

        //// NFT
        let value: u64 = COUNTER.load(deps.storage)? + 1;
        // let mint_msg = cw721_metadata_onchain::MintMsg {
        //     token_id: value.to_string(),
        //     owner: env.contract.address.to_string(),
        //     token_uri: None,
        //     extension: extension.clone(),
        // };

        let exec_msg = hypersign_kyc_token::msg::ExecuteMsg::Mint {
            token_id: value.to_string(),
            owner: env.contract.address.to_string(),
            token_uri: None,
            extension: extension.clone(),
        };

        //// Mint SBT to the issuer_kyc_contract
        let sbt_contract_address = SBT_CONTRACT_ADDRESS.load(deps.storage)?;
        let mint_nft_msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&exec_msg)?,
            funds: (&[]).to_vec(),
        };
        resp = resp.add_message(mint_nft_msg);

        //// transfer SBT to the user
        /// // fetch the counter

        /// TODO this should up updated in the call back - like may be in the reply.
        COUNTER.save(deps.storage, &value)?;

        let transfer_nft_msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&hypersign_kyc_token::msg::ExecuteMsg::TransferNft {
                recipient: info.sender.to_string(),
                token_id: value.clone().to_string(),
            })?,
            funds: (&[]).to_vec(),
        };

        //// TODO: need to find out how can use should not be able to re-transfer to other users.
        resp = resp.add_message(transfer_nft_msg);
        resp = resp
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", value.to_string());
        Ok(resp)
    }
}
