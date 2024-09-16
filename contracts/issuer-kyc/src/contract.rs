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
    use cw721_base::Extension;
    use strum_macros::ToString;
    pub type ExecuteMsg = cw721_metadata_onchain::ExecuteMsg;

    use crate::{
        error::KycContractError,
        msg::{
            CW721OnChainMetadataInstantiateMsg, Cw721InstantiateMsg, ExecMsg, ExecuteNFTMsg,
            HypersignKYCProof, HypersignKYCProofTypes,
        },
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
                admin: Some(info.sender.to_string()),
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
            token_uri: None,
            extension: extension.clone(),
        };

        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        // Mint SBT to the issuer_kyc_contract
        let msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&exec_msg)?,
            funds: (&[]).to_vec(),
        };
        resp = resp.add_message(msg);

        // transfer SBT to the user
        let transfer_msg = WasmMsg::Execute {
            contract_addr: sbt_contract_address.clone(),
            msg: to_json_binary(&ExecuteNFTMsg::TransferNft {
                recipient: info.sender.to_string(),
                token_id: value.clone().to_string(),
            })?,
            funds: (&[]).to_vec(),
        };

        resp = resp.add_message(transfer_msg);

        resp = resp
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str());
        // .add_attribute("counter", counter.to_string());
        Ok(resp)
    }
}
