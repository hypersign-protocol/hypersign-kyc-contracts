
nibid config chain-id nibiru-localnet-0 && nibid config node http://localhost:26657


nibid config chain-id nibiru-testnet-1 && nibid config node https://rpc.testnet-1.nibiru.fi:443

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
nibid q tx C83C855B257284604E13DE24D28F8D819C289A17BF88E33FFF33E58D366B8B2D | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#15
```


### Uploads kyc contract code

```bash
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
# nibid q tx 126C10344066750CA9C4C922728812B20F5BFA2D0DCB975CA5C8868F555E8B71 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'

nibid q tx 034BC2F730BC599E93BCB0BD9AA2EB64CE624CE50B49D969E9518FFE2F840160 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#20
```

### Uploads factory contract code

```bash
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 09FB198651D1F127EFB50BFCF6EC121037287FF8BECC9035E002276A5C887B9B | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#17
```


### Uploads SBT Contract code

```bash
## this artifact is present in /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts folder
nibid tx wasm store /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts/cw721_metadata_onchain.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 420132F6BC55A31B0764E0F960B05609E2345BBE74C19055EB30F8BE17244152 | jq -rcs '.[0].logs[0].events[1].attributes[1].value' 
#18
```


## Admin for SSI

### Instnatiate SSI Contract

```bash
nibid tx wasm instantiate 15 '{"owner_did": "did:hid:123123123", "did_method": "did:hid:testnet" }' --label "SSI" --no-admin --from validator   --gas auto --gas-adjustment 1.5 --gas-prices 0.25unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 8B128B3638A37960BA6147CFEA882C441ED85F96BADAA487FFD86C33E0AF90DC | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#nibi1w3kngdapu0keaame9596wnfztxvv6zm29knnqeznax4z7nwcuk6syrc45w
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
nibid tx wasm instantiate 17 '{"counter": 0, "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "hypersign_ssi_manager_contract_address": "nibi1w3kngdapu0keaame9596wnfztxvv6zm29knnqeznax4z7nwcuk6syrc45w", "kyc_contract_code_id": 20 }' --label "Activity" --no-admin --from validator   --gas auto --gas-adjustment 1.5 --gas-prices 0.25unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 9B8AF660AAE2F1B8BCE4FAF30B3C176D8A04D708E95B4A7046B3461DC22327E9 | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#nibi1r22k3my03clkws8phjhfcc3ny45p939hwkh0f05wjyl27a3y8cwsnrsj2v
```
==============

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1r22k3my03clkws8phjhfcc3ny45p939hwkh0f05wjyl27a3y8cwsnrsj2v '{"get_hypersign_admin_d_i_d":{}}'
```

## Issuer 

### Issuer onboard himself

```bash
nibid tx wasm instantiate 76 '{"did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "hypersign_ssi_manager_contract_address": "nibi15tu27v983k0my76k4e6r4z6y7sjvussctwe0aqr69tztzdgpauasv98ejt" }' --label "Activity" --from issuer --gas 100000000 --no-admin
nibid q tx 7C447FEB3200F5F4ABA33D9B4AB4B2ACC85C69685DAF72D691E0A0B2DCD16F4F | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
```

### Issuer onboard himself

```bash
nibid tx wasm execute nibi1r22k3my03clkws8phjhfcc3ny45p939hwkh0f05wjyl27a3y8cwsnrsj2v '{"onboard_issuer": { "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq" }}' --from validator  --keyring-backend test  --gas 100000000 
# nibid q tx 4BE3478C54EC84725CB19C95A59373C4A6C46F0E51A471E3BE924366AE356BAD | jq
```

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1r22k3my03clkws8phjhfcc3ny45p939hwkh0f05wjyl27a3y8cwsnrsj2v '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
# {"data":{"issuer":{"id":"issuer-1","did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp","kyc_contract_address":"nibi1r22k3my03clkws8phjhfcc3ny45p939hwkh0f05wjyl27a3y8cwsnrsj2v","kyc_contract_code_id":71}}}
```

### Query Issuer Contract about its data 

```bash
nibid query wasm contract-state smart nibi1n78c8ckw5xwktg7laae7utrm7jtyrsshnfrl9n72xzlxz9gpq0zszfpzf3 '{"owner_d_i_d":{}}'
# {"data":{"owner_did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}} 
```

### Issuer inistalizes his SBT contract 

```bash
nibid tx wasm execute nibi1n78c8ckw5xwktg7laae7utrm7jtyrsshnfrl9n72xzlxz9gpq0zszfpzf3 '{"init": { "token_code_id": 18 }}' --from validator  --keyring-backend test  --gas 100000000 
nibid q tx 618ED82F55839D71A86B3D3BF1AA58CE734A50AA58D72A2B12A732934B72FEA0 | jq
```


### Issuer query his SBY contact data
```bash
nibid query wasm contract-state smart nibi1n78c8ckw5xwktg7laae7utrm7jtyrsshnfrl9n72xzlxz9gpq0zszfpzf3 '{"s_b_t_contract_address":{}}'
# {"data":{"sbt_contract_address":"nibi1az885vpd2azjmepzhs3t9fftv4td44cyk526jykgpzseghtj44qqtwtemd"}}
```

### Check if Issuer Contract has became the owner of the sBT contract or not


```bash 
nibid query wasm contract-state smart nibi1az885vpd2azjmepzhs3t9fftv4td44cyk526jykgpzseghtj44qqtwtemd '{"minter":{}}'
#{"data":{"minter":"nibi16yhg0alwt30g7e5pe3jwnn48kywrdry0xknylufv4l27c9s9w0xsvqprt4"}}

#nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"contract_info":{}}'
```

## User

### User mints his NFT

```bash
nibid tx wasm execute nibi1n78c8ckw5xwktg7laae7utrm7jtyrsshnfrl9n72xzlxz9gpq0zszfpzf3 '{"mint": {"hypersign_proof": {"credential_id": "12132312", "zk_proof": { "proof_type": "zk_proof_of_age", "public_signales": ["1","18955587923911110975324593921788466916679894646588172021082202393332121293343","11370393776179332609488947571879226318156480814724305073726489837302371244311","3502129987681126598706754762542340737175834041097740797030651868926291943299","16689638488897210389721526189894955938148630429690598708199340667708642425048","18"], "proof": {"pi_a":[40,92,159,239,135,71,131,180,248,147,169,222,232,97,48,105,217,165,250,185,7,60,74,90,135,68,142,168,205,253,76,96,15,217,143,188,100,205,0,41,220,68,189,168,247,105,81,239,21,251,38,244,193,42,110,83,49,160,238,190,131,198,159,67],"pi_b":[24,247,33,247,208,58,206,103,45,36,80,164,234,255,191,187,147,112,19,133,188,230,6,38,69,69,64,139,233,90,118,8,27,225,72,30,105,245,158,141,143,237,117,50,31,254,51,110,158,224,8,185,60,212,8,113,168,227,149,144,77,216,105,105,24,210,243,58,123,237,21,248,101,190,236,130,230,29,162,115,116,24,162,247,140,111,129,87,114,50,97,221,35,162,146,90,31,252,83,232,106,217,108,29,137,233,11,150,187,45,90,212,232,8,251,86,187,112,123,29,64,182,237,107,169,28,129,145],"pi_c":[7,227,99,82,182,142,207,181,216,239,108,223,37,105,149,62,227,167,64,136,119,23,180,153,245,38,38,254,54,10,71,99,48,64,56,8,200,111,39,153,41,97,2,11,48,230,70,149,245,40,15,48,29,74,92,191,234,202,117,80,119,168,252,2],"protocol":"groth16","curve":"bn128"}}}}}' --from validator --gas 100000000 --keyring-backend test  -y
nibid q tx 1C7A2F62AFCB4604266605A683959032383D923A49FDE22E185895C7E743601C | jq
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
