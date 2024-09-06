# ----------- Accounts --------------------------------

## Issuer IDentity
```bash
nibid keys add issuer --keyring-backend test  | jq

## o/p
{
  "name": "issuer",
  "type": "local",
  "address": "nibi1m53rg8qnpnkcsu7ttth4lqqm9h9fau3ttp28s6",
  "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A7rmFRVgIrTQyYvKw8qwC8/asQNNtnFqccF5+4Y6dnnS\"}",
  "mnemonic": "fancy sweet lock flower depth stamp method leaf whisper memory gym ranch manual affair electric confirm deposit rebel soap nice poet please nuclear point"
}

nibid tx bank send validator nibi1m53rg8qnpnkcsu7ttth4lqqm9h9fau3ttp28s6 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi1m53rg8qnpnkcsu7ttth4lqqm9h9fau3ttp28s6  | jq
```

## User Identity
```bash
nibid keys add user --keyring-backend test

{"name":"user","type":"local","address":"nibi17wj5y04tsnpwgl4vswqpvvj0ef0k7cu7wpldp0","pubkey":"{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A87NmnkrvxquxtQ+uiPQe8Z2vn7zMXqVwcElN1sh2/+K\"}","mnemonic":"detail arrest sunny throw thrive hub runway off stove sadness mirror master sustain gesture cram sort recall clog flight injury ramp spy tongue patrol"}

nibid tx bank send validator nibi17wj5y04tsnpwgl4vswqpvvj0ef0k7cu7wpldp0 1000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi17wj5y04tsnpwgl4vswqpvvj0ef0k7cu7wpldp0  | jq

balances:
- amount: "100000"
  denom: unibi
pagination:
  next_key: null
  total: "0"
```

# ----------- ADMIN --------------------------------

## -1. Admin uploads cw721_base contract code

1284
```bash
TXHASH="$(nibid tx wasm store /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts/cw721_base.wasm --from validator --gas auto  --gas-adjustment 1.5 --gas-prices 0.025unibi -y  | jq -rcs '.[0].txhash')"
CW_721_CONTRACT_CODE_ID=$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')
```

```bash
{
  "key": "code_id",
  "value": "8",
  "index": true
}
```


## 0. Admin uploads ssi-manager contract code

```bash
TXHASH="$(nibid tx wasm store ./artifacts/ssi_manager.wasm --from nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl --gas 100000000  --gas-adjustment 1.5 --gas-prices 0.025unibi  -y  | jq -rcs '.[0].txhash')"
SSI_MANAGER_CONTRACT_CODE_ID=$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')
1281


nibid tx wasm store ./artifacts/ssi_manager.wasm --from validator --gas 100000000
nibid q tx 9538F4162CC46534CD73B554ABBFAC11EE40C6159BDB0ED502FEBB13F22B8C9F  | jq
```

```bash
{
  "key": "code_id",
  "value": "5",
  "index": true
}
```

## 1. Admin uploads  Issuer kyc contract code

```bash

TXHASH="$(nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas 100000000 --gas-adjustment 1.5  --gas-prices 0.025unibi -y  | jq -rcs '.[0].txhash')"
ISSUER_KYC_CONTRACT_CODE_ID=$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')

nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas 100000000
nibid q tx 5B46B826ADCCC41970DFF89E38FBEB829E08F3EF5BDA59442D847FA72F1627D5  | jq
#nibid q wasm list-code 
```

```bash
{
  "key": "code_id",
  "value": "6",
  "index": true
}
```

## 2. Admin uploads factory contract code

```bash

TXHASH="$(nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas 100000000 --gas-adjustment 1.5  --gas-prices 0.025unibi -y  | jq -rcs '.[0].txhash')"
HYPERSIGN_FACTORY_CONTRACT_CODE_ID=$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')

nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas 100000000
nibid q tx E5AC5E7E198BD00B87F585A8C06956D2B88E1E35D1DFD74E01DDB6CAC8B26AD2  | jq
```


```bash
{
  "key": "code_id",
  "value": "7",
  "index": true
}
```

## 2.1. Admin instantiate the SSI MAnager Contract


```bash
SSI_MANAGER_CONTRACT_CODE_ID=1281
TXHASH="$(nibid tx wasm instantiate $SSI_MANAGER_CONTRACT_CODE_ID '{"owner_did": "did:hid:12313123123", "did_method": "did:hid:testnet" }' --label "Activity" --from validator --gas 100000000 --gas-adjustment 1.5  --gas-prices 0.025unibi --no-admin -y  | jq -rcs '.[0].txhash')"



SSI_MANAGER_CONTRACT_ADDRESSS=$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[0].value')



nibid tx wasm instantiate 5 '{"owner_did": "did:hid:12313123123", "did_method": "did:hid:testnet" }' --label "Activity" --from validator --gas 100000000 --no-admin
nibid q tx 842B501373EA9E932D1F885D9211A2B1CF1237A1DD41287F991EAECA427A98DE  | jq
nibid q wasm list-contract-by-code 5
```

```bash
{"contracts":["nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz"],"pagination":{"next_key":null,"total":"0"}}
```

## 2.2. Admin Registers a DID with SSI Manage

```bash
#did='did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B'
#did_doc_string='{"@context":["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","controller":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"alsoKnownAs":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"],"verificationMethod":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"Ed25519VerificationKey2020","controller":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B","publicKeyMultibase":"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}],"authentication":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"assertionMethod":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"keyAgreement":[],"capabilityInvocation":["did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1"],"capabilityDelegation":[],"service":[{"id":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","type":"LinkedDomains","serviceEndpoint":"https://www.linkeddomains.com"}]}'
#did_doc_proof_string='{"@context": ["https://www.w3.org/ns/did/v1","https://w3id.org/security/suites/ed25519-2020/v1"],"type":"Ed25519Signature2020","created":"2010-01-01T19:23:24Z","verificationMethod":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1","proofPurpose":"assertionMethod"}'


SSI_MANAGER_CONTRACT_ADDRESSS=nibi1tkcvyx4vnfaauwx9k06s80m5s4acdsefzpl4g07yvxt8ugas256qwlwp7p
nibid tx wasm execute $SSI_MANAGER_CONTRACT_ADDRESSS '{"register_d_i_d": { "did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B", "did_doc": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\",\"controller\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"],\"alsoKnownAs\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"],\"verificationMethod\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"type\":\"Ed25519VerificationKey2020\",\"controller\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\",\"publicKeyMultibase\":\"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"}],\"authentication\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"assertionMethod\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"keyAgreement\":[],\"capabilityInvocation\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"capabilityDelegation\":[],\"service\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"type\":\"LinkedDomains\",\"serviceEndpoint\":\"https://www.linkeddomains.com\"}]}","did_doc_proof": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"type\":\"Ed25519Signature2020\",\"created\":\"2010-01-01T19:23:24Z\",\"verificationMethod\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"proofPurpose\":\"assertionMethod\"}"}}' --from validator  --keyring-backend test  --gas 100000000 --gas-adjustment 1.5  --gas-prices 0.025unibi -y

nibid tx wasm execute nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz '{"register_d_i_d": { "did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B", "did_doc": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\",\"controller\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"],\"alsoKnownAs\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"],\"verificationMethod\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"type\":\"Ed25519VerificationKey2020\",\"controller\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\",\"publicKeyMultibase\":\"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"}],\"authentication\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"assertionMethod\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"keyAgreement\":[],\"capabilityInvocation\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"capabilityDelegation\":[],\"service\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"type\":\"LinkedDomains\",\"serviceEndpoint\":\"https://www.linkeddomains.com\"}]}","did_doc_proof": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"type\":\"Ed25519Signature2020\",\"created\":\"2010-01-01T19:23:24Z\",\"verificationMethod\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"proofPurpose\":\"assertionMethod\"}"}}' --from validator  --keyring-backend test  --gas 100000000 
nibid q tx 8423B6A76C19A693F893B6772E6824133ABFB259FDD365A132963BEADA5D26E5  | jq
```

## 2.3. Admin Reolves a DID with SSI Manage just to check

```bash
nibid query wasm contract-state smart $SSI_MANAGER_CONTRACT_ADDRESSS '{"resolve_d_i_d":{"did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}}'
nibid query wasm contract-state smart nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz '{"resolve_d_i_d":{"did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}}'
```

## 3. Admin instantiate the hypersign factory contract

```bash

HYPERSIGN_FACTORY_CONTRACT_CODE_ID=1282
TXHASH="$(nibid tx wasm instantiate $HYPERSIGN_FACTORY_CONTRACT_CODE_ID '{"counter": 0, "hypersign_ssi_manager_contract_address": "nibi1tkcvyx4vnfaauwx9k06s80m5s4acdsefzpl4g07yvxt8ugas256qwlwp7p", "kyc_contract_code_id": 1283, "hypersign_admin_did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"  }' --label "Activity" --from validator --gas 100000000 --gas-adjustment 1.5  --gas-prices 0.025unibi --no-admin  -y | jq -rcs '.[0].txhash')"
HYPERSIGN_FACTORY_CONTRACT_ADDRESS=$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[0].value')


nibid tx wasm instantiate 7 '{"counter": 0, "hypersign_ssi_manager_contract_address": "nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz", "kyc_contract_code_id": 6, "hypersign_admin_did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"  }' --label "Activity" --from validator --gas 100000000 --no-admin
nibid q tx 1D2F09F560EA8C312A5B3F11DD7D2EE0309028F3196F987EBB93B2AD45A089A1  | jq

issuer-kyc git:(main) ✗ nibid q wasm list-contract-by-code 7
```

```bash
{"contracts":["nibi17r3faruez552kdxy0lsjydmj0nu22mxax33azx326039hfe7pnhqp5u720"],"pagination":{"next_key":null,"total":"0"}}
```

## 3.1. Admin queries DID from the hypersign factory contract

```bash

HYPERSIGN_FACTORY_CONTRACT_ADDRESS=nibi13xud4swt67s4x8xmrcuns3dqyn5dgp9s3v3vuwku5shvfvkpq45s84gy0v
nibid query wasm contract-state smart $HYPERSIGN_FACTORY_CONTRACT_ADDRESS '{"get_hypersign_admin_d_i_d":{}}' | jq



nibid query wasm contract-state smart nibi17r3faruez552kdxy0lsjydmj0nu22mxax33azx326039hfe7pnhqp5u720 '{"get_hypersign_admin_d_i_d":{}}'
```
```bash
{
  "data": {
    "did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"
  }
}
```
---

# ----------- ISSUER --------------------------------

## Issued registered himself a DID on SSI manager contract
```bash
nibid tx wasm execute $SSI_MANAGER_CONTRACT_ADDRESSS '{"register_d_i_d": {"did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo110", "did_doc": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\",\"controller\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\"],\"alsoKnownAs\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\"],\"verificationMethod\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\",\"type\":\"Ed25519VerificationKey2020\",\"controller\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\",\"publicKeyMultibase\":\"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\"}],\"authentication\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\"],\"assertionMethod\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\"],\"keyAgreement\":[],\"capabilityInvocation\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\"],\"capabilityDelegation\":[],\"service\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\",\"type\":\"LinkedDomains\",\"serviceEndpoint\":\"https://www.linkeddomains.com\"}]}","did_doc_proof": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"type\":\"Ed25519Signature2020\",\"created\":\"2010-01-01T19:23:24Z\",\"verificationMethod\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\",\"proofPurpose\":\"assertionMethod\"}"}}' --from issuer  --keyring-backend test  --gas 100000000  --gas-adjustment 1.5  --gas-prices 0.025unibi -y | jq
```

## 4. Issuer onboard himself through factory

```bash
## Issued  on boards hims self on the factory contract , also deploys his issuer contract
nibid tx wasm execute $HYPERSIGN_FACTORY_CONTRACT_ADDRESS '{"onboard_issuer": {"issuer_did":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B" }}' --from validator  --keyring-backend test --gas 100000000  --gas-adjustment 1.5  --gas-prices 0.025unibi  -y | jq
nibid q tx EFA9E68E38E54B6402341E54658048B2987A6C48B9D4C8B22E277E3C24EF26FB | jq
```

## 5. Issuer Get the Issuer KYC contract address
```bash
nibid query wasm contract-state smart $HYPERSIGN_FACTORY_CONTRACT_ADDRESS '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}}' | jq

nibid query wasm contract-state smart nibi17r3faruez552kdxy0lsjydmj0nu22mxax33azx326039hfe7pnhqp5u720 '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123"}}'
```

```bash
{
  "data": {
    "issuer": {
      "id": "issuer-1",
      "did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123",
      "kyc_contract_address": "nibi1466nf3zuxpya8q9emxukd7vftaf6h4psr0a07srl5zw74zh84yjqxpf0en",
      "kyc_contract_code_id": 6
    }
  }
}
```

## 6. Issuer initialize SBT contract
```bash
nibid tx wasm execute nibi155j73hrxs5zcul3jzh45k7yxzw6kyq6wr7fmdmntwsfmg87xcl4s3jkdql '{"init": {"token_code_id": 1284}}' --from validator  --keyring-backend test --gas auto  --gas-adjustment 1.5  --gas-prices 0.025unibi 
nibid q tx 69EE8B2256F310A8F2AAE86F0936B5B35616C18EA3C6CFB8A06723469F7DAF62 | jq
```

## 7. Issuer Get SBT contract address

```bash
nibid query wasm contract-state smart nibi155j73hrxs5zcul3jzh45k7yxzw6kyq6wr7fmdmntwsfmg87xcl4s3jkdql '{"s_b_t_contract_address":{}}'
```
```bash
{
  "data": {
    "sbt_contract_address": "nibi1tthpad273qzcq2dl8vrsu6j3hh8ceya9q5hht673mac35s6npt9qsgmlw8"
  }
}
```

# ----------- USERS -------------------------------- 

## 8. User mints NFT through ISsuer contract only..
```bash
nibid tx wasm execute nibi155j73hrxs5zcul3jzh45k7yxzw6kyq6wr7fmdmntwsfmg87xcl4s3jkdql '{"mint": {}}' --from validator --gas auto   --gas-adjustment 1.5  --gas-prices 0.025unibi --keyring-backend test 
nibid q tx D84FE5091B6C7E7E5A5D2C395E2CE987717692E39392A3A27ABC0E70C3B5C025 | jq
```

## 9. Check status of NFT in the NFT contract

```bash
nibid query wasm contract-state smart nibi1tthpad273qzcq2dl8vrsu6j3hh8ceya9q5hht673mac35s6npt9qsgmlw8 '{"num_tokens":{}}'
```
```bash
{
  "data": {
    "count": 1
  }
}
```

## 10. User own nft now

```bash
nibid query wasm contract-state smart nibi1tthpad273qzcq2dl8vrsu6j3hh8ceya9q5hht673mac35s6npt9qsgmlw8 '{"tokens":{"owner": "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl"}}' | jq
```
```bash
{
  "data": {
    "tokens": [
      "1"
    ]
  }
}
```

--

```
nibid query wasm contract-state smart nibi14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3lesu3yusz '{"all_nft_info":{"token_id": "1"}}'
```

---





