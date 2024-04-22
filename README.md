# Hypersign On-Chain KYC Contract Architecture

![img](./docs/hypersign-kyc-cosm-wasm-contracts.png)

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


## TODOs

- [x] Research
    - [x] Figoure out spec for SBT (cw-721-non-transferable)
    - [x] Figure out how can you make a smart contract mint SBT
    - [x] Figure out how to instantiate one contract from the other in factory design pattern
- [ ] Implement  basic `Hypersign_KYC_factory_Contract`
    - [ ] Implement `onboard_issuer()` to onboard an issuer in the system 
- [ ] Implement basic `Issuer_KYC_Contract`
    - [ ] Implement `mint()` to mint a NFT to the user
- [ ] Integrate entity dashboard to onboard_issuer from the UI
- [ ] Integrate widget UI to mint SBTs
- [ ] Refator code; use wrappers
- [ ] Implement multitest cases 
- [ ] Work on all edge cases of smart contract
- [ ] Contract optimization; see how we can impvoe gas fee, check for any security loopholes