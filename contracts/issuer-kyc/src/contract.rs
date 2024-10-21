use crate::error::KycContractError;
use crate::state::{COUNTER, INSTANTIATE_TOKEN_REPLY_ID, OWNER, OWNERDID, SBT_CODE_ID};
use crate::{msg::InstantiateMsg, state::*};

use cosmwasm_std::{
    to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn, Response,
    StdError, StdResult, SubMsg, WasmMsg,
};
use serde_json::{from_slice, from_str, Value};
// use crate::ssi_manager::ed25519_signature_2020;

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
    use super::{
        COUNTER, INSTANTIATE_TOKEN_REPLY_ID, OWNER, SBT_CODE_ID, SBT_CONTRACT_ADDRESS, SBT_NAME,
        SBT_SYMBOL,
    };
    use cosmwasm_std::Empty;
    use strum_macros::ToString;
    pub type ExecuteMsg = cw721_metadata_onchain::ExecuteMsg;

    use crate::{
        error::KycContractError,
        msg::{
            CW721OnChainMetadataInstantiateMsg, ExecMsg, HypersignKYCProof, HypersignKYCProofTypes,
        },
        zkpverify,
    };
    use cosmwasm_std::{
        to_binary, to_json_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, ReplyOn,
        Response, StdError, StdResult, SubMsg, WasmMsg,
    };
    pub fn init(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        token_code_id: u64,
    ) -> Result<Response, KycContractError> {
        // let token_code_id = SBT_CODE_ID.load(deps.storage)?;
        // println!("token_code_id {:?}", token_code_id);

        SBT_CODE_ID.save(deps.storage, &token_code_id)?;

        // Instantiating a new SBT contract
        let sub_msg: Vec<SubMsg> = vec![SubMsg {
            msg: WasmMsg::Instantiate {
                code_id: token_code_id,
                msg: to_json_binary(&CW721OnChainMetadataInstantiateMsg {
                    name: SBT_NAME.to_owned(),
                    symbol: SBT_SYMBOL.to_owned(),
                    minter: env.contract.address.clone().into(), //
                })?,
                funds: vec![],
                admin: Some(env.contract.address.clone().into()),
                label: String::from("Instantiate fixed NFT contract"),
            }
            .into(),
            id: INSTANTIATE_TOKEN_REPLY_ID,
            gas_limit: None,
            reply_on: ReplyOn::Success,
        }];

        let resp = Response::new().add_submessages(sub_msg);
        Ok(resp)
    }

    pub fn mint(
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        hypersign_proof: HypersignKYCProof,
    ) -> Result<Response, KycContractError> {
        let mut resp = Response::new();

        let sbt_contract_address = SBT_CONTRACT_ADDRESS.load(deps.storage)?;

        // TODO: check if sbt_contract_address is not set
        //if(sbt_contract_address)

        // TODO verify zk proof sent by users..
        // tODO pass proof...
        let proof_string = r#"
            {"pi_a":[40,92,159,239,135,71,131,180,248,147,169,222,232,97,48,105,217,165,250,185,7,60,74,90,135,68,142,168,205,253,76,96,15,217,143,188,100,205,0,41,220,68,189,168,247,105,81,239,21,251,38,244,193,42,110,83,49,160,238,190,131,198,159,67],"pi_b":[24,247,33,247,208,58,206,103,45,36,80,164,234,255,191,187,147,112,19,133,188,230,6,38,69,69,64,139,233,90,118,8,27,225,72,30,105,245,158,141,143,237,117,50,31,254,51,110,158,224,8,185,60,212,8,113,168,227,149,144,77,216,105,105,24,210,243,58,123,237,21,248,101,190,236,130,230,29,162,115,116,24,162,247,140,111,129,87,114,50,97,221,35,162,146,90,31,252,83,232,106,217,108,29,137,233,11,150,187,45,90,212,232,8,251,86,187,112,123,29,64,182,237,107,169,28,129,145],"pi_c":[7,227,99,82,182,142,207,181,216,239,108,223,37,105,149,62,227,167,64,136,119,23,180,153,245,38,38,254,54,10,71,99,48,64,56,8,200,111,39,153,41,97,2,11,48,230,70,149,245,40,15,48,29,74,92,191,234,202,117,80,119,168,252,2],"protocol":"groth16","curve":"bn128"}
        "#;

        let public_signal = [
            "1",
            "18955587923911110975324593921788466916679894646588172021082202393332121293343",
            "11370393776179332609488947571879226318156480814724305073726489837302371244311",
            "3502129987681126598706754762542340737175834041097740797030651868926291943299",
            "16689638488897210389721526189894955938148630429690598708199340667708642425048",
            "18",
        ];

        let proof_type = "zkProofOfAge";
        match zkpverify::verify_zkp(
            proof_string.to_string(),
            &public_signal,
            proof_type.to_string(),
        ) {
            Ok(result) => {
                if result {
                    println!("proof is verified");
                } else {
                    return Err(KycContractError::ZkProofVerificationFailure {});
                }
            }
            Err(err) => {
                return Err(KycContractError::ZkProofFailure {});
            }
        }
        ////

        // fetch the counter
        let value: u64 = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;

        // {
        //     token_id: token_id.to_string(),
        //     owner: "john".to_string(),
        //     token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
        //     extension: Some(Metadata {
        //         description: Some("Spaceship with Warp Drive".into()),
        //         name: Some("Starship USS Enterprise".to_string()),
        //         ..Metadata::default()
        //     }),
        // }

        // https://docs.opensea.io/docs/metadata-standards
        // https://github.com/public-awesome/cw-nfts/tree/v0.9.3/contracts/cw721-metadata-onchain

        //// https://docs.opensea.io/docs/metadata-standards#metadata-structure

        //// https://docs.opensea.io/docs/metadata-standards#attributes
        // Here trait_type is the name of the trait, value is the value of the trait,
        // and display_type is a field indicating how you would like it to be displayed.
        // For string traits, you don't have to worry about display_type.

        /// Creating all traits
        let prooftype = hypersign_proof.proof_type; //HypersignKYCProofTypes::ProofOfAge;
        let proof_type_trait = cw721_metadata_onchain::Trait {
            display_type: None, // No display type
            trait_type: "proof-type".to_string(),
            value: prooftype.to_string(),
        };

        let sbt_code_trait = cw721_metadata_onchain::Trait {
            display_type: None,
            trait_type: "sbt-code".to_string(),
            value: hypersign_proof.sbt_code.to_string(),
        };

        let extension = Some(cw721_metadata_onchain::Metadata {
            description: Some(hypersign_proof.description.to_string()),
            name: Some(prooftype.to_string()),
            // image_data: "", // use this if you are not using image. you can store svg image
            image: Some(hypersign_proof.proof_type_image.expect("Error")),
            background_color: Some("#ffttwww".to_string()),
            attributes: Some(vec![proof_type_trait, sbt_code_trait]),
            ..cw721_metadata_onchain::Metadata::default()
        });

        let mint_msg = cw721_metadata_onchain::MintMsg {
            token_id: value.to_string(),
            owner: env.contract.address.to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: extension.clone(),
        };

        // {
        //     token_id: "1",
        //     owner: "contract0",
        //     token_uri: None,
        //     extension: Some(Metadata { image: Some(""), image_data: None, external_url: None, description: Some("Proves that user has finished his/her KYC"), name: Some("ProofOfKYC"), attributes: Some([Trait { display_type: None, trait_type: "proof-type", value: "ProofOfKYC" }, Trait { display_type: None, trait_type: "sbt-code", value: "T2" }]), background_color: Some("#ffttwww"), animation_url: None, youtube_url: None })
        // }

        println!(
            "Beofre cw721_metadata_onchain::MintMsg {:?}",
            mint_msg.clone()
        );

        let exec_msg = cw721_metadata_onchain::ExecuteMsg::Mint(mint_msg.clone());
        // Mint SBT to the issuer_kyc_contract
        let mint_nft_msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&exec_msg)?,
            funds: (&[]).to_vec(),
        };
        resp = resp.add_message(mint_nft_msg);

        // transfer SBT to the user
        let transfer_nft_msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&cw721_metadata_onchain::ExecuteMsg::TransferNft {
                recipient: info.sender.to_string(),
                token_id: value.clone().to_string(),
            })?,
            funds: (&[]).to_vec(),
        };

        println!("transfer_nft_msg = {:?}", transfer_nft_msg.clone());

        resp = resp.add_message(transfer_nft_msg);

        resp = resp
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", value.to_string());
        Ok(resp)
    }
}
