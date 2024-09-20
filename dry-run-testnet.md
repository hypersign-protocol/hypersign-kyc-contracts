
nibid config chain-id nibiru-localnet-0 && nibid config node http://localhost:26657

## Key-Materials

### Issuer IDentity
```
nibid keys add issuer --keyring-backend test

issuer: {
    address: "nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg",
    name: "issuer",
    mnemonic: "fossil away enjoy victory conduct position window torch middle grab maple head scheme kick idle shoe width monkey village spawn goddess ankle parrot knife"
}

nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg | jq
```

fossil away enjoy victory conduct position window torch middle grab maple head scheme kick idle shoe width monkey village spawn goddess ankle parrot knife




### User Identity
```
nibid keys add user2 --keyring-backend test

user: {
    address: "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5",
    name: "user",
    mnemonic: "mind rate breeze party huge brain solar upon budget find opinion sketch submit awkward evil throw phrase umbrella night person improve ribbon siren cute"
}

nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 1000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 | jq

balances:
- amount: "100000"
  denom: unibi
pagination:
  next_key: null
  total: "0"
```


nibid keys add user2 --keyring-backend test | jq
```
{
  "name": "user2",
  "type": "local",
  "address": "nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l",
  "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A6mXgB9pECPh6IPl7Rj/1mmU6XYvP7tZjZu91rSIAuZ7\"}",
  "mnemonic": "pony fantasy artist prefer level puzzle nerve powder rude front endless rib supreme fan off sand clarify age thumb chronic celery coffee below beef"
}
```

```
nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l 1000000unibi --keyring-backend test

nibid q bank balances nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l | jq
```
----------- ADMIN --------------------------------

## Developer

### Uploads SSI contract code
```bash
nibid tx wasm store ./artifacts/ssi_manager.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx F2EEB4B3FD4B9A008E8CB74E05E425B4A15D57B3B845515FE359F42326877260 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'

#1310
```


### Uploads kyc contract code

```bash
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 126C10344066750CA9C4C922728812B20F5BFA2D0DCB975CA5C8868F555E8B71 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#1311
```

### Uploads factory contract code

```bash
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx B49EB81C83FDE405B6A474B1ABA0C82CD7682D03FC3B252CBAAD8A5692DFC39E | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#1312
```


### Uploads SBT Contract code

```bash
## this artifact is present in /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts folder
nibid tx wasm store /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts/cw721_metadata_onchain.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 2A25E9FD9C32123C1ABF6CDA20F5C53C1B8416C0E88F16ED66347481927DEEEE | jq -rcs '.[0].logs[0].events[1].attributes[1].value' 
#1313
```


## Admin for SSI

### Instnatiate SSI Contract

```bash
nibid tx wasm instantiate 1310 '{"owner_did": "did:hid:123123123", "did_method": "did:hid:testnet" }' --label "SSI" --no-admin --from validator   --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx CCF027D11A45F5F4EA60693D31BD8EF2653A4B3F7FC223C2E561D7ED6EF5CE6C | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#nibi1ffq5s3ghw2aghgks98hqxexsdn736ec9jc96rzjhwpwyxz5wcjuqwslhmn
```

### Register DID

```bash
nibid tx wasm execute nibi15tu27v983k0my76k4e6r4z6y7sjvussctwe0aqr69tztzdgpauasv98ejt '{"register_d_i_d":{"did_doc":"[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]","did_doc_proof":"[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]","signature":"z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq"}}' --from nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl --gas 100000000  --keyring-backend test

nibid q tx 8B1FA1FD8BE52FF7DA4103F884F9A365B74D0397B397797EBEB8BCBA8364DD65 | jq
```

### Resolve DID

```bash
nibid query wasm contract-state smart nibi15tu27v983k0my76k4e6r4z6y7sjvussctwe0aqr69tztzdgpauasv98ejt '{"resolve_d_i_d":{"did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
```



## Admin for Hypersign FActory

### Admin instantiate the factory contract

```bash
nibid tx wasm instantiate 1312 '{"counter": 0, "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "hypersign_ssi_manager_contract_address": "nibi1ffq5s3ghw2aghgks98hqxexsdn736ec9jc96rzjhwpwyxz5wcjuqwslhmn", "kyc_contract_code_id": 1311 }' --label "Activity" --no-admin --from validator   --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 047A683EFD2CA7280AE3B04DCEAD86DB502F306ED08A32F7E3B53B33E0ACEA56 | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#nibi19c3tpkj9dr3wakehywh8mxkua56d0l5nrkzx35uyt6jxuy686yrqlr6q2u
```











==============





### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1qfdeqsc4m8jhz8zxch2vmsqr0fx7zspc54w8r7sj880n9l75qrlsfqynf5 '{"get_hypersign_admin_d_i_d":{}}'
```

## Issuer 

### Issuer onboard himself

```bash
nibid tx wasm instantiate 76 '{"did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "hypersign_ssi_manager_contract_address": "nibi15tu27v983k0my76k4e6r4z6y7sjvussctwe0aqr69tztzdgpauasv98ejt" }' --label "Activity" --from issuer --gas 100000000 --no-admin
nibid q tx 7C447FEB3200F5F4ABA33D9B4AB4B2ACC85C69685DAF72D691E0A0B2DCD16F4F | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
```

### Issuer onboard himself

```bash
# nibid tx wasm execute nibi1qfdeqsc4m8jhz8zxch2vmsqr0fx7zspc54w8r7sj880n9l75qrlsfqynf5 '{"onboard_issuer": { "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq" }}' --from issuer  --keyring-backend test  --gas 100000000 
# nibid q tx 3F271F63FD063C4F48A0887AA516DFA5992F3C3678614AA86B931E024DC15B08 | jq
```

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1qfdeqsc4m8jhz8zxch2vmsqr0fx7zspc54w8r7sj880n9l75qrlsfqynf5 '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
# {"data":{"issuer":{"id":"issuer-1","did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp","kyc_contract_address":"nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87","kyc_contract_code_id":71}}}
```

### Query Issuer Contract about its data 

```bash
nibid query wasm contract-state smart nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"owner_d_i_d":{}}'
# {"data":{"owner_did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}} 
```

### Issuer inistalizes his SBT contract 

```bash
nibid tx wasm execute nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"init": { "token_code_id": 75 }}' --from issuer  --keyring-backend test  --gas 100000000 
nibid q tx 6C450BB6BA03E503488AD91C4EB7E87BC30E5B7DE88FE0067C6DE91CA5A47E1D | jq
```


### Issuer query his SBY contact data
```bash
nibid query wasm contract-state smart nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"s_b_t_contract_address":{}}'
# {"data":{"sbt_contract_address":"nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx"}}
```

### Check if Issuer Contract has became the owner of the sBT contract or not


```bash 
nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"minter":{}}'
#{"data":{"minter":"nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87"}}

#nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"contract_info":{}}'
```

## User

### User mints his NFT

```bash
nibid tx wasm execute nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"mint": {"hypersign_proof": {"credential_id": "", "data": "","description": "Proves that user has finished his/her KYC", "proof_type_image": "https://cdn.mos.cms.futurecdn.net/mpGh6USjRkE3dPQnF8tXRC-1200-80.jpg", "sbt_code": "T1" , "proof_type" : "proof_of_personhood" }}}' --from nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l --gas 100000000 --keyring-backend test 
nibid q tx CCDBC03684655596B95E13E56F67E97A553981514F2F8D6E1B81B27E09C43AE1 | jq
```

### USer checks his balance in nft contract

```bash 
nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"num_tokens":{}}'
```

```bash
nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"tokens":{"owner": "nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l"}}'
```

```bash
nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"all_nft_info":{"token_id": "1"}}'
```

```bash
nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"nft_info":{"token_id": "1"}}'
```
