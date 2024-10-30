# Hypersign On-Chain KYC Contract Architecture

![img](./docs/hypersign-kyc-cosm-wasm-contracts-Page-3.drawio.png)



## How to install 

```
https://www.youtube.com/watch?v=z6Skl_a8IYk
```

Pre-requisite: 

```bash
rustup target add wasm32-unknown-unknown
cargo install cosmwasm-check
```

## Version 

- rustc 1.80.1 (3f5fd8dd4 2024-08-06)
- rustup 1.27.1 (54dd3d00f 2024-04-24)
- cargo 1.80.1 (376290515 2024-07-16)

## Sequence diagram 

![img](./docs/hypersign-multi-tenant-ssi-infra.png)

- Hypersign Admin deploys `Hypersign_KYC_factory_Contract`
- **Issuer Onboarding:**
    - Issuer invokes `deploy_your_kyc()` of `Hypersign_KYC_factory_Contract` to deploy its KYC System:
        - `Issuer_KYC_Contract` is instantiated.
        - `Issuer_NFT_Contract` (cw-721) is instantiated 
        - `Issuer_KYC_Contract` is set as admin of `Issuer_NFT_Contract`.
    - `Issuer_KYC_Contract`_address  gets registered as issuer in `Hypersign_KYC_factory_contract`
- **User on-chain KYC Process:**
    - User calls `mintNFT()` of the `Issuer_KYC_Contract` (user pays the gas - fee for the entire transaction)
        - which then will call `mint()` of `Issuer_NFT_Contract` (`Issuer_KYC_Contract` becomes the owner of NFT)
        - Which then will call `transfer()` of `Issuer_NFT_Contract` (transfering NFT from `Issuer_KYC_Contract` to user)


# TODOs

- :white_check_mark: Research
    - :white_check_mark: Figoure out spec for SBT (cw-721-non-transferable)
    - :white_check_mark: Figure out how can you make a smart contract mint SBT
    - :white_check_mark: Figure out how to instantiate one contract from the other in factory design pattern
- :white_check_mark: Implement  basic `Hypersign_KYC_factory_Contract`
    - :white_check_mark: Implement  `deploy_your_Kyc()` to onboard an issuer in the system 
- :white_check_mark: Implement basic `Issuer_KYC_Contract`
    - :white_check_mark: Implment `init()` to initialize the NFT contract
    - :white_check_mark: Implement `mint()` to mint a NFT to the user
- :white_check_mark: Integrate entity dashboard to `deploy_your_Kyc()` from the UI
- :white_check_mark: Integrate widget UI to mint SBTs
- :white_check_mark: Implement mint-sbt module in hypersig kyc service 
- :white_check_mark: Implement feature in caach server to capture user's miniing step
- :white_check_mark: Implement widget configuration page in Entity KYC dashboard
----------------------------------------------------------------
- :white_check_mark: Figure out how can we verify issuer_did (over IBC / ORacle etc.)
- :white_check_mark: Figure out how to canonize a json ld
- :white_check_mark: Figure out how can we verify DID of issuer in the factory contract while onboarding/deboarding an 

Issuer
----------------------------------------------------------------
- :white_check_mark: Implement `SSI_manager_contract` with constructor params `{ did_method: "did:testnet:hid" }`
    - :white_check_mark: Implement helper method - `verify_proof() -> true`
    - :white_check_mark: Implement `register_did({did, signed_did_doc})`
    - :white_check_mark: Implement `resolve_did({did}) -> did_doc`
    - :white_check_mark: Implement helper method - `canonize_ld_doc({ld_doc, context}) -> string`
- :white_check_mark: Improve `instantiation({SSI_manager_contract, hs_admin_did, hs_admin_did_doc, hs_admin_did_doc_proof})` of `Hypersign_KYC_factory_Contract` to whitelist `SSI_manager_contract` address and whitelist hypersign_did
- [ ] Improve `deploy_your_Kyc({issuer_did_proof, hs_authorization_presentation, issuer_did})` in `Hypersign_KYC_factory_Contract` 
    - :white_check_mark: to verify issuer_did before onboarding.
    - [ ] to verify presentation signed by hypersign_did (only allow those issuers who have authorization)
- [ ] Improve `mint({user_did, user_did_proof, issuer_authorization_presentation})` of `Issuer_KYC_Contract` 
    - [ ] to verify user_did
    - [ ] to verify signed presentation by issuer_did (only allow those users who have authorization)
- :white_check_mark: Implement request did-auth for issuer did in onchain-kyc page in dashboard during onchain kyc deployment step

Hypersign Admin Authorization
------ 

- [ ] Implement request `hs_authorization_credential` in onchain-kyc page in dashboard during onchain kyc deployment step
- [ ] Implement generate `hs_authorization_presentation` in onchain-kyc page in dashboard during onchain kyc deployment step
- [ ] Implement request did-auth for user_did in mint NFT page in widget during performing KYC
- [ ] Implement request `issuer_authorization_credential` in mint NFT page in widget during performing KYC
- [ ] Update new contract address in widget and dashboard and test the entire flow.
- [ ] Implement a authorization (using verifiable credential verifications) from hypersign admin 

General
-------
- :white_check_mark: Onchain data models (what exact data will go in the metadata of NFT)
- [ ] Backward compatibility with the old version (when upgrading the contracts states should not change)
- [ ] Verify if the user has actually done the minitng by calling the contract in the `verify()` of `SbtMintService` service
- :white_check_mark: Integrate facefi API backend
- [ ] The widget URL should be signed and then sent given to the user for security purposess
- [ ] Refator code; use wrappers
- [ ] Implement multitest cases 
- :white_check_mark: Contract optimization; see how we can improve gas fee, 
- :white_check_mark: Update ver-keys for all other proofs
- :white_check_mark: Implement test cases for all other kind of proofs
- [ ] Implement exectue function to update Hypersign Admin DID

Security Issues
-------

- [ ] zk proofs can be prone to replay attacks (Implement nullifiers)
- [ ] All DID signatures are also prone to replay attacks
- [ ] Check for any security loopholes
- [ ] Implement how to stop users to re-transfer the token to other users? 
- [ ] Work on all edge cases of smart contract

## Resources

- [Hypersign On-chain kyc documentation](https://docs.google.com/document/d/1Gso6w9mbkRlv6bvyQDnrhqZhmoD9WOhleY3p2LVIJOQ/edit#heading=h.1krz9xs6n001)
- [CosmWasm Documentation](https://docs.cosmwasm.com/docs/smart-contracts/state/cw-plus)
- [non-fungible-tokens](https://docs.aura.network/developer/tutorials/non-fungible-tokens/instantiate)
- [cosmwasm](https://book.cosmwasm.com)
- [transmitting-context-between-contracts](https://docs.burnt.com/xion/develop/cosmwasm-resources/contract-semantics/message/submessages#transmitting-context-between-contracts)
- [map-storage](https://book.cosmwasm.com/cross-contract/map-storage.html)
- Metadata
    - [metadata-standards](https://docs.opensea.io/docs/metadata-standards)
    - [cw721-metadata-onchain](https://github.com/public-awesome/cw-nfts/tree/v0.9.3/contracts/cw721-metadata-onchain)
    - [metadata-structure](https://docs.opensea.io/docs/metadata-standards#metadata-structure)
    - [metadata-attributes](https://docs.opensea.io/docs/metadata-standards#attributes)

