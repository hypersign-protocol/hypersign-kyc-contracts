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

```bash
TXHASH="$(nibid tx wasm store /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts/cw721_base.wasm --from validator --gas 100000000 -y  | jq -rcs '.[0].txhash')"
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

nibid tx wasm execute nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz '{"register_d_i_d": {"did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B", "did_doc": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\",\"controller\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"],\"alsoKnownAs\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"],\"verificationMethod\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"type\":\"Ed25519VerificationKey2020\",\"controller\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\",\"publicKeyMultibase\":\"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B\"}],\"authentication\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"assertionMethod\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"keyAgreement\":[],\"capabilityInvocation\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\"],\"capabilityDelegation\":[],\"service\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"type\":\"LinkedDomains\",\"serviceEndpoint\":\"https://www.linkeddomains.com\"}]}","did_doc_proof": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"type\":\"Ed25519Signature2020\",\"created\":\"2010-01-01T19:23:24Z\",\"verificationMethod\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B#key-1\",\"proofPurpose\":\"assertionMethod\"}"}}' --from validator  --keyring-backend test  --gas 100000000 
nibid q tx 8423B6A76C19A693F893B6772E6824133ABFB259FDD365A132963BEADA5D26E5  | jq
```

## 2.3. Admin Reolves a DID with SSI Manage just to check

```bash
nibid query wasm contract-state smart nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz '{"resolve_d_i_d":{"did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"}}'
```

## 3. Admin instantiate the hypersign factory contract

```bash
nibid tx wasm instantiate 7 '{"counter": 0, "hypersign_ssi_manager_contract_address": "nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz", "kyc_contract_code_id": 6, "hypersign_admin_did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxoa1B"  }' --label "Activity" --from validator --gas 100000000 --no-admin
nibid q tx 1D2F09F560EA8C312A5B3F11DD7D2EE0309028F3196F987EBB93B2AD45A089A1  | jq

issuer-kyc git:(main) ✗ nibid q wasm list-contract-by-code 7
```

```bash
{"contracts":["nibi17r3faruez552kdxy0lsjydmj0nu22mxax33azx326039hfe7pnhqp5u720"],"pagination":{"next_key":null,"total":"0"}}
```

## 3.1. Admin queries DID from the hypersign factory contract

```bash
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

## 4. Issuer onboard himself

```bash

## Issued registered himself a DID
nibid tx wasm execute nibi1hm4y6fzgxgu688jgf7ek66px6xkrtmn3gyk8fax3eawhp68c2d5q7tt2vz '{"register_d_i_d": {"did": "did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123", "did_doc": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\",\"controller\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\"],\"alsoKnownAs\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\"],\"verificationMethod\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\",\"type\":\"Ed25519VerificationKey2020\",\"controller\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\",\"publicKeyMultibase\":\"z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123\"}],\"authentication\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\"],\"assertionMethod\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\"],\"keyAgreement\":[],\"capabilityInvocation\":[\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\"],\"capabilityDelegation\":[],\"service\":[{\"id\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\",\"type\":\"LinkedDomains\",\"serviceEndpoint\":\"https://www.linkeddomains.com\"}]}","did_doc_proof": "{\"@context\":[\"https://www.w3.org/ns/did/v1\",\"https://w3id.org/security/suites/ed25519-2020/v1\"],\"type\":\"Ed25519Signature2020\",\"created\":\"2010-01-01T19:23:24Z\",\"verificationMethod\":\"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123#key-1\",\"proofPurpose\":\"assertionMethod\"}"}}' --from issuer  --keyring-backend test  --gas 100000000 
nibid q tx F4FD8B22F028C6F22D900B2DFA6975594457919FAE1313DF12DF9D1951E81EEA | jq

## Issued  on boards hims self on the factory contract , also deploys his issuer contract
nibid tx wasm execute nibi17r3faruez552kdxy0lsjydmj0nu22mxax33azx326039hfe7pnhqp5u720 '{"onboard_issuer": {"issuer_did":"did:hid:testnet:z6MkkyG63Rb68hBFhUg9n2a3teEzQdhqyCqAdVZYC5Dxo123123" }}' --from issuer  --keyring-backend test  --gas 100000000 
nibid q tx F43BA788D41B43F13464AB7AEC782C57E5073E25087DCFE4FC11FE5E6239A9CD | jq
```

## 5. Issuer Get the Issuer KYC contract address
```bash
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
nibid tx wasm execute nibi1466nf3zuxpya8q9emxukd7vftaf6h4psr0a07srl5zw74zh84yjqxpf0en '{"init": {"token_code_id": 8}}' --from issuer  --keyring-backend test --gas 100000000 
nibid q tx FFFCDC020E2E224C8BE744A5F6FCB43C39420D0BF3B4F4F37051F067B922D573 | jq
```

## 7. Issuer Get SBT contract address

```bash
nibid query wasm contract-state smart nibi1466nf3zuxpya8q9emxukd7vftaf6h4psr0a07srl5zw74zh84yjqxpf0en '{"s_b_t_contract_address":{}}'
```
```bash
{
  "data": {
    "sbt_contract_address": "nibi14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3lesu3yusz"
  }
}
```

# ----------- USERS -------------------------------- 

## 8. User mints NFT through ISsuer contract only..
```bash
nibid tx wasm execute nibi1466nf3zuxpya8q9emxukd7vftaf6h4psr0a07srl5zw74zh84yjqxpf0en '{"mint": {}}' --from user --gas 100000000 
nibid q tx EA40497C4F1FC8CB17FC66683199C234137104EE991739A4C4705824CAC209F9 | jq
```

## 9. Check status of NFT in the NFT contract

```bash
nibid query wasm contract-state smart nibi14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3lesu3yusz '{"num_tokens":{}}'
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
nibid query wasm contract-state smart nibi14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3lesu3yusz '{"tokens":{"owner": "nibi17wj5y04tsnpwgl4vswqpvvj0ef0k7cu7wpldp0"}}' | jq
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





