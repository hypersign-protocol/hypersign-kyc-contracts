
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
}```


nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l 1000000unibi --keyring-backend test

nibid q bank balances nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l | jq
----------- ADMIN --------------------------------

## Developer

### Uploads SSI contract code
```bash
nibid tx wasm store ./artifacts/ssi_manager.wasm --from validator --gas  100000000 --yes | jq -rcs '.[0].txhash'

nibid q tx 07F91DCED56F010A5F8E69168190D88DE0AD84708EDA678CA1A73FABCACF1C9B | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
```

### Uploads kyc contract code

```bash
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas-prices 0.1unibi --gas auto --gas-adjustment 1.3  --yes | jq -rcs '.[0].txhash' 
nibid q tx 1623131C915AEB7C7A1660D1E680A2881BC0865262AD2CF8911AA95AA3D5FD1D | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#1358
```

### Uploads factory contract code

```bash
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas-prices 0.1unibi --gas auto --gas-adjustment 1.3  --yes | jq -rcs '.[0].txhash' 
nibid q tx 73F9FBA1E3241C83309384FAEA9695B07AB456DD052D3AD6D7E72128715636F2 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#1359
```


### Uploads SBT Contract code

```bash
## this artifact is present in /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts folder
nibid tx wasm store ./artifacts/cw721_metadata_onchain.wasm --from validator --gas 100000000
nibid q tx 32DCDE2AF62E8B3858C3DEDB85CDE9E2749CC75CADDC9AB0C555DF8D2D3E1438 | jq -rcs '.[0].logs[0].events[1].attributes[1].value' | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#75
```


## Admin for SSI

### Instnatiate SSI Contract

```bash
nibid tx wasm instantiate 67 '{"owner_did": "did:hid:123123123", "did_method": "did:hid:testnet" }' --label "SSI" --from nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl --gas 100000000 --no-admin --keyring-backend test

nibid q tx 916FB7EF77AD3EA960A20B2AB9D750342AB13F7695659C649560A14B7780375D | jq -rcs '.[0].logs[0].events[1].attributes[0].value'

nibid q wasm list-contract-by-code 67 | jq
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
nibid tx wasm instantiate 1359 '{"counter": 0, "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "kyc_contract_code_id": 1358 }' --label "Activity" --from validator --gas-prices 0.1unibi --gas auto --gas-adjustment 1.3  --yes  --no-admin | jq -rcs '.[0].txhash' 
nibid q tx FD10C955785EB21DD9B1F7A4518D1C94AF586ADDB8293E2934D54D7206FC54D9 | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#nibi1ghtmzya95j9tqpkww6mlvy7pvr9qfks5rp34p3pe27cahja9ft6sgz2eje
```

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1ghtmzya95j9tqpkww6mlvy7pvr9qfks5rp34p3pe27cahja9ft6sgz2eje '{"get_hypersign_admin_d_i_d":{}}'
```

## Issuer 

### Issuer onboard himself

```bash
nibid tx wasm instantiate 1358 '{"did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq" }' --label "Activity" --from validator --gas-prices 0.1unibi --gas auto --gas-adjustment 1.3  --yes  --no-admin | jq -rcs '.[0].txhash' 
nibid q tx 212E4DB61066F4E860153B91DAE025A017E9BDE720B1B5D448C96453483D35DE | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
```

### Issuer onboard himself

```bash
# nibid tx wasm execute nibi1ghtmzya95j9tqpkww6mlvy7pvr9qfks5rp34p3pe27cahja9ft6sgz2eje '{"onboard_issuer": { "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq" }}' --from issuer  --keyring-backend test  --gas 100000000 
# nibid q tx 3F271F63FD063C4F48A0887AA516DFA5992F3C3678614AA86B931E024DC15B08 | jq
```

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1u7gaw4alm2zjj7ad3n7lqgcxduwhrln894ekxk0t9fxz4qzxt0zq9l85pv '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
# {"data":{"issuer":{"id":"issuer-1","did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp","kyc_contract_address":"nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87","kyc_contract_code_id":71}}}
```

### Query Issuer Contract about its data 

```bash
nibid query wasm contract-state smart nibi1u7gaw4alm2zjj7ad3n7lqgcxduwhrln894ekxk0t9fxz4qzxt0zq9l85pv '{"owner_d_i_d":{}}'
# {"data":{"owner_did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}} 
```

### Issuer inistalizes his SBT contract 

```bash
nibid tx wasm execute nibi1u7gaw4alm2zjj7ad3n7lqgcxduwhrln894ekxk0t9fxz4qzxt0zq9l85pv '{"init": { "token_code_id": 1313 }}' --from validator --gas-prices 0.1unibi --gas auto --gas-adjustment 1.3  --yes  | jq -rcs '.[0].txhash' 
nibid q tx 85A4EFB7F835D74B2B6C23287EAE9DA2A8129032CAEEA2B72DC8E7F598A313AB | jq
```


### Issuer query his SBY contact data
```bash
nibid query wasm contract-state smart nibi1u7gaw4alm2zjj7ad3n7lqgcxduwhrln894ekxk0t9fxz4qzxt0zq9l85pv '{"s_b_t_contract_address":{}}'
# {"data":{"sbt_contract_address":"nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx"}}
```

### Check if Issuer Contract has became the owner of the sBT contract or not


```bash 
nibid query wasm contract-state smart nibi1wm73ngvprx6suu9t93zvwtszeqt7c67w9vl8emkj7ws5ryd332lsn66n2n '{"minter":{}}'
#{"data":{"minter":"nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87"}}

#nibid query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"contract_info":{}}'
```

## User

### User mints his NFT

```bash
nibid tx wasm execute nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"mint": {"hypersign_proof": {"credential_id": "", "data": "","description": "Proves that user has finished his/her KYC", "proof_type_image": "https://cdn.mos.cms.futurecdn.net/mpGh6USjRkE3dPQnF8tXRC-1200-80.jpg", "sbt_code": "T1" , "proof_type" : "proof_of_personhood" }}}' --from nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l --gas 100000000 --keyring-backend test 

nibid tx wasm execute nibi1ag6a26rvt9duue8wh8rsp94z9lx7c5pdjf8mlkals6f2uqr0aj0s4ltp6q '{"mint": {"hypersign_proof": {"credential_id": "", "data": "","description": "Proves that user has finished his/her KYC", "proof_type_image": "https://cdn.mos.cms.futurecdn.net/mpGh6USjRkE3dPQnF8tXRC-1200-80.jpg", "sbt_code": "T1" , "proof_type" : "proof_of_personhood" }}}' --from issuer --keyring-backend test --gas 100000000

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
