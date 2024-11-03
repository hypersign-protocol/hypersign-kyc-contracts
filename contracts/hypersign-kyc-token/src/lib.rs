pub mod error;
pub mod msg;
pub mod state;

use cw721::traits::{Cw721Execute, Cw721Query};

// Version info for migration
const CONTRACT_NAME: &str = "hypersign.id:cw721-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
use cw721::extension::Cw721OnchainExtensions;
pub type HypersignKYCToken<'a> = Cw721OnchainExtensions<'a>;

pub mod entry {
    use super::*;

    use crate::state::{Config, CONFIG};
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response};
    use cw721::msg::Cw721MigrateMsg;
    use cw721::msg::{Cw721ExecuteMsg, Cw721InstantiateMsg};
    use error::ContractError;
    use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let admin_addr: Option<Addr> = msg
            .minter
            .as_deref()
            .map(|s| deps.api.addr_validate(s))
            .transpose()?;

        let config = Config { admin: admin_addr };

        CONFIG.save(deps.storage, &config)?;

        let cw721_instantiate_msg = Cw721InstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            collection_info_extension: msg.collection_info_extension,
            minter: msg.minter,
            creator: msg.creator,
            withdraw_address: msg.withdraw_address,
        };

        HypersignKYCToken::default().instantiate_with_version(
            deps.branch(),
            &env,
            &info,
            cw721_instantiate_msg,
            CONTRACT_NAME,
            CONTRACT_VERSION,
        )
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let config = CONFIG.load(deps.storage)?;
        match config.admin {
            Some(admin) => {
                if admin == info.sender {
                    Cw721OnchainExtensions::default().execute(deps, &env, &info, msg)
                } else {
                    Err(ContractError::Ownership(
                        cw721::OwnershipError::TransferNotFound,
                    ))
                }
            }
            None => match msg {
                Cw721ExecuteMsg::Mint {
                    token_id,
                    owner,
                    token_uri,
                    extension,
                } => HypersignKYCToken::default()
                    .mint(deps, &env, &info, token_id, owner, token_uri, extension),
                _ => Err(ContractError::Ownership(cw721::OwnershipError::NotOwner)),
            },
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
        HypersignKYCToken::default().query(deps, &env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: Cw721MigrateMsg,
    ) -> Result<Response, ContractError> {
        let contract = HypersignKYCToken::default();
        contract.migrate(deps, env, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::{msg::NftExtensionMsg, state::Trait, NftExtension};
    use msg::{ExecuteMsg, InstantiateMsg};

    const CREATOR: &str = "creator";

    /// Make sure cw2 version info is properly initialized during instantiation,
    /// and NOT overwritten by the base contract.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        entry::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InstantiateMsg {
                name: "collection_name".into(),
                symbol: "collection_symbol".into(),
                collection_info_extension: None,
                minter: None,
                creator: None,
                withdraw_address: None,
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
    }

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = HypersignKYCToken::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            collection_info_extension: None,
            minter: None,
            creator: None,
            withdraw_address: None,
        };
        contract
            .instantiate(deps.as_mut(), &mock_env(), &info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
        let extension = Some(NftExtensionMsg {
            description: Some("description1".into()),
            name: Some("name1".to_string()),
            attributes: Some(vec![Trait {
                display_type: None,
                trait_type: "type1".to_string(),
                value: "value1".to_string(),
            }]),
            ..NftExtensionMsg::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), &mock_env(), &info, exec_msg)
            .unwrap();

        let res = contract
            .query_nft_info(deps.as_ref().storage, token_id.into())
            .unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(
            res.extension,
            Some(NftExtension {
                description: Some("description1".into()),
                name: Some("name1".to_string()),
                attributes: Some(vec![Trait {
                    display_type: None,
                    trait_type: "type1".to_string(),
                    value: "value1".to_string(),
                }]),
                ..NftExtension::default()
            })
        );
    }
}
