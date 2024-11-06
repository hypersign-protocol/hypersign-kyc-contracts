# Hypersign On-Chain KYC

The on-chain KYC feature enables you to verify users' identities in a privacy-preserving manner, granting access to your dApps without compromising sensitive information. For example, if you need to verify a user’s identity before allowing access to your smart contract, this can be done in three key steps:

### On-chain KYC Configuration
- dApps deploy the on-chain KYC contracts via the Entity Studio dashboard on their preferred (supported) blockchains.
- dApps activate the on-chain KYC settings and configure zero-knowledge (zk) proof options within the widget configuration.

### Token Minting
- Users generate the required zk proofs from their identity credentials.
- Users mint Soulbound Tokens (SBTs) tied to their zk proofs.
### Token Verification
- The dApp’s smart contract queries and verifies the user's SBT directly on-chain, confirming identity before granting access.
- This process provides decentralised, privacy-respecting identity verification for secure dApp access.

## The Architecture

The architecture of on-chain KYC contracts is straightforward, consisting primarily of three smart contracts and a set of libraries. Initially, the Hypersign Admin deploys the **Hypersign KYC Factory** contract on the blockchain. This KYC Factory contract manages a registry of issuers and maintains the contract addresses of their respective KYC Issuer contracts.

For any dApp that wishes to implement an on-chain KYC system, they simply call the **Hypersign KYC Factory** contract to initiate the deployment of their dedicated **Hypersign KYC Issuer** contract. The contract serves two primary purposes: (a) Deploying the **Hypersign KYC Token** contract, and (b) Verifying zero-knowledge (zk) proofs to mint the **Hypersign KYC Token** for users. Once a dApp deploys its own **Hypersign KYC Issuer** contract, it needs to call the **Hypersign KYC Token** contract to instantiate its respective KYC token contract. Users then mint their on-chain ID—a  Soulbound Token (SBT)—through the **Hypersign KYC Issuer** contract, establishing their verified identity on-chain.

![img](./docs/hypersign-kyc-cosm-wasm-contracts-Page-3.drawio.png)

Finally, The **Hypersign KYC Token** contract is responsible for managing all on-chain identities for users. dApps that need to confirm a user’s completion of the KYC process can call this contract to verify their status.

> As a dApp, you do not need to worry about **Hypersign KYC Factory** Contract. You only need to deploy the KYC Issuer and KYC Token contract through [Entity Studio Dashboard](https://docs.hypersign.id/hypersign-kyc/on-chain-kyc/contracts-deployment) in few button clicks. 

## Bit of History 

We started working on this project after we recieved some appreciation on our work in [Delphi Hackathon 2023](https://devpost.com/software/eclipse-fi?ref_content=my-projects-tab&ref_feature=my_projects). We even received token of appreciation of few thousands dollar (;p). The proposed idea was to make token transactions compliant, in the most privaccy preserving way. In the hackathon we not only used zero knowledge proofs but also sent and verfied proof across blockchains using IBC. But ofcourse we hacked some of thing related to KYC and data capturing during the hackathon due to lack to time and funds. I encourage you to checkout that [zk-kyc-using-ibc repository](https://github.com/hypersign-protocol/zk-kyc-using-ibc) as well. 

That hackathon motivated us to pursue the usecase and build it as a product. This particular repository contains contracts and packages necessary for deploying on-chain KYC on Cosmos-based blockchains.

## Folder St. 

Root is a workspace wherein we have two main folders:

- `contracts`
    - `hypersign-factory` contract
    - `hypersign-kyc-token` contract
    - `issuer-kyc` contract
- `packages`
    - `ssi-manager` package 
    - `hypersign-zk-verifier` package

## Installation

### Pre-requisite: 

```bash
rustup target add wasm32-unknown-unknown
cargo install cosmwasm-check
```

### Version 

- rustc 1.80.1 (3f5fd8dd4 2024-08-06)
- rustup 1.27.1 (54dd3d00f 2024-04-24)
- cargo 1.80.1 (376290515 2024-07-16)

## Pending Tasklist

[Todo list](/docs/todo.md)

## Supported Blockchain 

We maintain list of blockchains that we support in the [Chain metadata repository](https://github.com/hypersign-protocol/hypersign-kyc-chains-metadata).

## How to contribute?

