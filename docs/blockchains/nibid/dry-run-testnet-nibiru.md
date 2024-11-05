
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

### Uploads kyc contract code

```bash
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
# nibid q tx 8E64E1648D1DD6D9B6584690E6B362F74E48B68B4AD368F21109FFAE186F0C0E | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
nibid q tx AD736B12C98D027DD0CDC4AC6E3739EB033B15D7AD73684923782395BAAF81FD | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#1393
```

### Uploads factory contract code

```bash
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx C80570EB81391A38F253314C76822F83ACEF9A315787EAE60DAB994704EA574F | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#1394
```


### Uploads SBT Contract code

```bash
## this artifact is present in /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts folder
nibid tx wasm store ./artifacts/hypersign_kyc_token.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 61391EA772797030777E7DC59A063424DDE3F5FD6EDC7CB9453106DF5A0A9781 | jq -rcs '.[0].logs[0].events[1].attributes[1].value' 
#1395
```

## Admin for Hypersign FActory

### Admin instantiate the factory contract

```bash
nibid tx wasm instantiate 1394 '{"counter": 0, "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "kyc_contract_code_id": 1393 }' --label "Activity" --from validator --gas-prices 0.025unibi --gas auto --gas-adjustment 1.3  --yes --admin validator | jq -rcs '.[0].txhash' 
nibid q tx 2A4081152CE8F057E7C8BD18AF16570FDAB1A324E0401DE585E117EA88ACA8F6 | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#nibi1zayxxsz5l9m0l058t9ckud6jdxenu09qstaplf896wy4ux9e3xnsxxu9ps
```
==============

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi16xz4vg9xh9arxfky5zwl8g6mtk9dxkjt7h608f80dplkfzgr3mesgq0658 '{"get_hypersign_admin_d_i_d":{}}'
```

## Issuer 

### Issuer onboard himself

```bash
nibid tx wasm instantiate 76 '{"did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "hypersign_ssi_manager_contract_address": "nibi15tu27v983k0my76k4e6r4z6y7sjvussctwe0aqr69tztzdgpauasv98ejt" }' --label "Activity" --from issuer --gas 100000000 --no-admin
nibid q tx 7C447FEB3200F5F4ABA33D9B4AB4B2ACC85C69685DAF72D691E0A0B2DCD16F4F | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
```

### Issuer onboard himself

```bash
nibid tx wasm execute nibi1w7prt0th3x9r2npjdyhfcej6wtk8kj0j98p9xqahlstq2jtah79scqgr0k '{"onboard_issuer": { "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq" }}' --from validator --gas-prices 0.025unibi --gas auto --gas-adjustment 1.3  --yes  --keyring-backend test 
# nibid q tx FC3C90F5850BFE6D0671328DC75BF17A93DB6B6D37337E2804E570A30B8A0CE3 | jq
```

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1yv3yzs5n90a7zxnqlndrve57s0ntz5kd6k39dvswg8egmnryyf3sdagrnr '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
# {"data":{"issuer":{"id":"issuer-1","did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp","kyc_contract_address":"nibi16xz4vg9xh9arxfky5zwl8g6mtk9dxkjt7h608f80dplkfzgr3mesgq0658","kyc_contract_code_id":71}}}
```

### Query Issuer Contract about its data 

```bash
nibid query wasm contract-state smart nibi1g44zacllp4j0k4yjjsp66rtlvwedfzymaejl3p5lfq8htn2xrrsqau8f8p '{"owner_d_i_d":{}}'
# {"data":{"owner_did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}} 
```

### Issuer inistalizes his SBT contract 

```bash
nibid tx wasm execute nibi1yv3yzs5n90a7zxnqlndrve57s0ntz5kd6k39dvswg8egmnryyf3sdagrnr '{"onboard_issuer": { "token_code_id": 1313 }}' --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes  --keyring-backend test  
nibid q tx 85B1E53D3A498C5D50BBCF44D95FE1D1C82C659E0A4EB1BC8B56C2716AD28816 | jq
```


### Issuer query his SBY contact data
```bash
nibid query wasm contract-state smart nibi1u7gaw4alm2zjj7ad3n7lqgcxduwhrln894ekxk0t9fxz4qzxt0zq9l85pv '{"s_b_t_contract_address":{}}'
# {"data":{"sbt_contract_address":"nibi105w0chel9d69cdzj62m4h2vqtj6vfuh7yzty2en825t2nyxw74est98gz2"}}
```

### Check if Issuer Contract has became the owner of the sBT contract or not


```bash 
nibid query wasm contract-state smart nibi1u7gaw4alm2zjj7ad3n7lqgcxduwhrln894ekxk0t9fxz4qzxt0zq9l85pv '{"minter":{}}'
#{"data":{"minter":"nibi16yhg0alwt30g7e5pe3jwnn48kywrdry0xknylufv4l27c9s9w0xsvqprt4"}}

#nibid query wasm contract-state smart nibi105w0chel9d69cdzj62m4h2vqtj6vfuh7yzty2en825t2nyxw74est98gz2 '{"contract_info":{}}'
```

## User

### User mints his NFT
```bash
nibid tx wasm execute nibi1jpq47nrtv37clh894dpd2r8s6kys3cwmjrcxv67lpaj9x5gqcqks93jalu '{"mint": {"hypersign_proof": {"credential_id": "12132312", "zk_proof": { "proof_type": "zk_proof_of_age", "public_signales": ["1","18955587923911110975324593921788466916679894646588172021082202393332121293343","11370393776179332609488947571879226318156480814724305073726489837302371244311","3502129987681126598706754762542340737175834041097740797030651868926291943299","16689638488897210389721526189894955938148630429690598708199340667708642425048","18"], "proof": {"pi_a":[40,92,159,239,135,71,131,180,248,147,169,222,232,97,48,105,217,165,250,185,7,60,74,90,135,68,142,168,205,253,76,96,15,217,143,188,100,205,0,41,220,68,189,168,247,105,81,239,21,251,38,244,193,42,110,83,49,160,238,190,131,198,159,67],"pi_b":[24,247,33,247,208,58,206,103,45,36,80,164,234,255,191,187,147,112,19,133,188,230,6,38,69,69,64,139,233,90,118,8,27,225,72,30,105,245,158,141,143,237,117,50,31,254,51,110,158,224,8,185,60,212,8,113,168,227,149,144,77,216,105,105,24,210,243,58,123,237,21,248,101,190,236,130,230,29,162,115,116,24,162,247,140,111,129,87,114,50,97,221,35,162,146,90,31,252,83,232,106,217,108,29,137,233,11,150,187,45,90,212,232,8,251,86,187,112,123,29,64,182,237,107,169,28,129,145],"pi_c":[7,227,99,82,182,142,207,181,216,239,108,223,37,105,149,62,227,167,64,136,119,23,180,153,245,38,38,254,54,10,71,99,48,64,56,8,200,111,39,153,41,97,2,11,48,230,70,149,245,40,15,48,29,74,92,191,234,202,117,80,119,168,252,2],"protocol":"groth16","curve":"bn128"}}}}}' --from validator --gas-prices 0.1unibi --gas auto --gas-adjustment 1.3  --yes  --keyring-backend test   
nibid q tx 3B54014C58B5E20BDF6165F2ABEC6976DC36DF5C843311BFE9A0560A5CB93CE6 | jq
```

### USer checks his balance in nft contract

```bash 
nibid query wasm contract-state smart nibi1wm73ngvprx6suu9t93zvwtszeqt7c67w9vl8emkj7ws5ryd332lsn66n2n '{"num_tokens":{}}'
```

```bash
nibid query wasm contract-state smart nibi105w0chel9d69cdzj62m4h2vqtj6vfuh7yzty2en825t2nyxw74est98gz2 '{"tokens":{"owner": "nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l"}}'
```

```bash
nibid query wasm contract-state smart nibi1wm73ngvprx6suu9t93zvwtszeqt7c67w9vl8emkj7ws5ryd332lsn66n2n '{"all_nft_info":{"token_id": "1"}}'
```

```bash
nibid query wasm contract-state smart nibi105w0chel9d69cdzj62m4h2vqtj6vfuh7yzty2en825t2nyxw74est98gz2 '{"nft_info":{"token_id": "1"}}'
```
