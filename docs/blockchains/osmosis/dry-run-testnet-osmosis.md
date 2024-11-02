## set config of client
osmosisd config set client chain-id osmo-test-5 && osmosisd config set client node https://rpc.testnet.osmosis.zone:443 && osmosisd config set client output json




osmosisd config set client node https://rpc.testnet.osmosis.zone:443
osmosisd config set client chain-id osmo-test-5

osmosis config node https://rpc.osmosis.zone:443
## Key-Materials

### Issuer IDentity
```
osmosisd keys add validator --keyring-backend test

issuer: {
    {
      "name":"validator",
      "type":"local",
      "address":"osmo18n35r9hhj0q7h23uje6lylun2z3mx5cgms9tl8",
      "pubkey":"{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AuUpUVeWBanPDPqpWPi0pv7mBX6fkX+E40Kdz/fjaDBT\"}",
      "mnemonic":"cluster patrol truth alter immune cotton focus tragic caution bargain glory stay number staff link stomach dismiss pluck uphold labor thunder crop student holiday"}
}

osmosisd tx bank send osmo18n35r9hhj0q7h23uje6lylun2z3mx5cgms9tl8  osmo1rgzrfecrp63g2yjyrcyc2pn6mmjdmxhzap9nze 1osmo --node https://rpc.testnet.osmosis.zone:443 --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3

osmosisd q tx D3E30B090B0E671EE97151DE14D311F9FC04A0B00BD3B3679E5FB60B5B550817 --node https://rpc.testnet.osmosis.zone:443 

osmosisd query bank balances osmo18n35r9hhj0q7h23uje6lylun2z3mx5cgms9tl8  --node https://rpc.testnet.osmosis.zone:443

osmosisd tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0

osmosisd q bank balances osmo1rgzrfecrp63g2yjyrcyc2pn6mmjdmxhzap9nze --chain-id  osmo-test-5 | jq
```

fossil away enjoy victory conduct position window torch middle grab maple head scheme kick idle shoe width monkey village spawn goddess ankle parrot knife




### User Identity
```
osmosisd keys add user2 --keyring-backend test

user: {
    address: "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5",
    name: "user",
    mnemonic: "mind rate breeze party huge brain solar upon budget find opinion sketch submit awkward evil throw phrase umbrella night person improve ribbon siren cute"
}

osmosisd tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 1000000unibi --keyring-backend test --chain-id nibiru-localnet-0

osmosisd q bank balances nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 | jq

balances:
- amount: "100000"
  denom: unibi
pagination:
  next_key: null
  total: "0"
```


osmosisd keys add user2 --keyring-backend test | jq
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
osmosisd tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l 1000000unibi --keyring-backend test

osmosisd q bank balances nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l | jq
```
----------- ADMIN --------------------------------

## Developer

### Uploads SSI contract code
```bash
osmosisd tx wasm store ./artifacts/ssi_manager.wasm --from validator --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes | jq -rcs '.[0].txhash'
osmosisd q tx 4B6C1C6B41DC9B1808E7013DBCF1E7668AC28EB4D17B38C584D7CDCB5342AA0C --node https://rpc.testnet.osmosis.zone:443 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'

#11157
```


### Uploads kyc contract code

```bash
osmosisd tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes | jq -rcs '.[0].txhash'
osmosisd q tx D4E57377FE55CF4CDBA10BBC55118E4FC03951BD41D00D55856C4D53D23277AE | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#11345
```

### Uploads factory contract code

```bash
osmosisd tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes | jq -rcs '.[0].txhash'
osmosisd q tx E37DB557E63ED4B0B197A5143C5912ADF82B0D45006370D3EECB774CEAE8005A | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
#11343
```


### Uploads SBT Contract code

```bash
## this artifact is present in /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts folder
osmosisd tx wasm store ./artifacts/hypersign_kyc_token.wasm --from validator --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes | jq -rcs '.[0].txhash'
osmosisd q tx 464556AAC46E9AF0D18183D8875322EA445CF2EC1EE7B8FAE27078F62FF6084F | jq -rcs '.[0].logs[0].events[1].attributes[1].value' 
#11344
```


## Admin for SSI

### Instnatiate SSI Contract

```bash
osmosisd tx wasm instantiate 11157 '{"owner_did": "did:hid:123123123", "did_method": "did:hid:testnet" }' --label "SSI" --no-admin --from validator  --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes | jq -rcs '.[0].txhash'
osmosisd q tx 9CAA31A081397EAEED21FCE45ADCD7843B48AD44163FCFAC776DD75EC4CDD5E4 | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
#osmo1mm5gfhtpu7xc27dawf5hdykazrdf36nurh6hzgtc6mrveajnzauqxuyqc6
```

### Register DID

```bash
osmosisd tx wasm execute osmo1mm5gfhtpu7xc27dawf5hdykazrdf36nurh6hzgtc6mrveajnzauqxuyqc6 '{"register_d_i_d":{"did_doc":"[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]","did_doc_proof":"[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]","signature":"z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq"}}' --from validator  --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes
osmosisd q tx 8B1FA1FD8BE52FF7DA4103F884F9A365B74D0397B397797EBEB8BCBA8364DD65 | jq
```

### Resolve DID

```bash
osmosisd query wasm contract-state smart osmo1mm5gfhtpu7xc27dawf5hdykazrdf36nurh6hzgtc6mrveajnzauqxuyqc6 '{"resolve_d_i_d":{"did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
```


## Admin for Hypersign FActory

### Admin instantiate the factory contract

```bash
osmosisd tx wasm instantiate 11343 '{"counter": 0, "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "kyc_contract_code_id": 11345 }' --label "Hypersign KYC Factory" --no-admin --from validator --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes | jq -rcs '.[0].txhash'
## https://celatone.osmosis.zone/osmo-test-5/contracts/osmo1mw0w823wrpj4fh7e8jg84gsxd2dvktxwkvghe9d4qmsaq98289jq3hvrvp
## https://celatone.osmosis.zone/osmo-test-5/contracts/osmo1try0nrtnevevyx0t8vn5ukrqynhmjfzq64rrz6ytcdhfwjtjjygqftgl6t
#osmo1mw0w823wrpj4fh7e8jg84gsxd2dvktxwkvghe9d4qmsaq98289jq3hvrvp
```






### User mints his NFT
```bash
osmosisd tx wasm execute osmo1lwkzmnn8xv4dhq8cy32tnmsam8hv69cpzgcts09seqzu0x7p4jrqmzy9xf '{"mint": {"hypersign_proof": {"credential_id": "12132312", "zk_proof": { "proof_type": "zk_proof_of_age", "public_signales": ["1","12040884699199694350430421040574883160903448743611754661868587601688521091572","17402076351219241481156702044498624695626933249600419891544118840897244553492","10407708521482612808325680154257593139024275871119660778918312440159182133587","5198474275750008277983235026401485600136802015956123361274085143332394226688","18"], "proof": {"pi_a": [13, 217, 14, 229, 255, 80, 6, 7, 88, 98, 9, 220, 185, 162, 141, 90, 135, 140, 101,191, 29, 89, 82, 117, 68, 245, 117, 102, 144, 77, 171, 104, 22, 107, 200, 36, 138,164, 0, 251, 109, 167, 129, 143, 154, 34, 120, 23, 20, 118, 12, 12, 182, 201, 137,168, 202, 199, 159, 75, 54, 253, 30, 225],"pi_b": [23, 175, 152, 25, 244, 84, 161, 42, 208, 177, 72, 224, 76, 175, 243, 168, 173, 76,69, 248, 62, 126, 144, 139, 82, 2, 153, 70, 109, 41, 201, 204, 6, 243, 136, 40,148, 84, 203, 195, 106, 7, 137, 71, 241, 120, 40, 146, 199, 143, 93, 13, 200, 229,37, 225, 29, 163, 140, 227, 178, 7, 220, 154, 42, 109, 234, 103, 35, 233, 166, 127,143, 131, 100, 160, 109, 33, 74, 154, 138, 200, 210, 131, 56, 206, 18, 120, 56,123, 51, 30, 136, 200, 225, 80, 23, 11, 84, 7, 107, 86, 4, 60, 128, 15, 229, 137,22, 206, 69, 99, 54, 63, 160, 235, 176, 67, 0, 195, 33, 202, 243, 132, 248, 47,251, 222],"pi_c": [1, 196, 237, 169, 186, 214, 135, 209, 184, 3, 43, 101, 139, 78, 230, 249, 220, 53,232, 194, 195, 12, 69, 137, 242, 185, 228, 202, 225, 176, 126, 245, 44, 60, 205,29, 193, 59, 43, 34, 163, 215, 50, 217, 217, 9, 47, 108, 25, 201, 73, 217, 54, 0,100, 90, 179, 220, 20, 61, 14, 166, 44, 45],"protocol":"groth16","curve":"bn128"}}}}}' --from validator --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --yes 
osmosisd q tx FA2F1653B827236B93198CF1FFFE4825E2691F7DD02F56A612E2F57D4723BA84 | jq
```








==============





### Query hypersign admin did

```bash
osmosisd query wasm contract-state smart nibi1qfdeqsc4m8jhz8zxch2vmsqr0fx7zspc54w8r7sj880n9l75qrlsfqynf5 '{"get_hypersign_admin_d_i_d":{}}'
```

## Issuer 

### Issuer onboard himself

```bash
osmosisd tx wasm instantiate 11321 '{"did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq", "hypersign_ssi_manager_contract_address": "osmo1mm5gfhtpu7xc27dawf5hdykazrdf36nurh6hzgtc6mrveajnzauqxuyqc6" }' --label "Activity" --from validator  --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 --no-admin
osmosisd q tx 7C447FEB3200F5F4ABA33D9B4AB4B2ACC85C69685DAF72D691E0A0B2DCD16F4F | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
```

### Issuer onboard himself

```bash
osmosisd tx wasm execute osmo18fetvfnmx2dlda6rneqvp2rrwewvluajw0xe626zslgy08d48v6syycn9e '{"onboard_issuer": { "did_doc": "[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]", "did_doc_proof": "[{\"https://w3id.org/security#challenge\":[{\"@value\":\"123123\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-09-01T17:44:11Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"http:adsasd\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1\"}]}]", "signature": "z3aY71DPQAqiiV5Q4UYZ6EYeWYa3MjeEHeEZMxcNfYxTqyn6r14yy1K3eYpuNuPQDX2mjh2BJ8VaPj5UKKMcAjtSq" }}' --from validator --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --node https://rpc.testnet.osmosis.zone:443 
# osmosisd q tx 3F271F63FD063C4F48A0887AA516DFA5992F3C3678614AA86B931E024DC15B08 | jq
```

### Query hypersign admin did

```bash
osmosisd query wasm contract-state smart nibi1qfdeqsc4m8jhz8zxch2vmsqr0fx7zspc54w8r7sj880n9l75qrlsfqynf5 '{"get_registered_issuer":{"issuer_did": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}}'
# {"data":{"issuer":{"id":"issuer-1","did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp","kyc_contract_address":"nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87","kyc_contract_code_id":71}}}
```

### Query Issuer Contract about its data 

```bash
osmosisd query wasm contract-state smart nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"owner_d_i_d":{}}'
# {"data":{"owner_did":"did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp"}} 
```

### Issuer inistalizes his SBT contract 

```bash
osmosisd tx wasm execute nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"init": { "token_code_id": 75 }}' --from issuer  --keyring-backend test  --gas 100000000 
osmosisd q tx 6C450BB6BA03E503488AD91C4EB7E87BC30E5B7DE88FE0067C6DE91CA5A47E1D | jq
```


### Issuer query his SBY contact data
```bash
osmosisd query wasm contract-state smart nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"s_b_t_contract_address":{}}'
# {"data":{"sbt_contract_address":"nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx"}}
```

### Check if Issuer Contract has became the owner of the sBT contract or not


```bash 
osmosisd query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"minter":{}}'
#{"data":{"minter":"nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87"}}

#osmosisd query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"contract_info":{}}'
```

## User

### User mints his NFT

```bash
osmosisd tx wasm execute nibi14gnnxyp9fms2hxyuxs8ajd4e7uvw30fge0zdfe67yxs7n3rwq6gquqdt87 '{"mint": {"hypersign_proof": {"credential_id": "", "data": "","description": "Proves that user has finished his/her KYC", "proof_type_image": "https://cdn.mos.cms.futurecdn.net/mpGh6USjRkE3dPQnF8tXRC-1200-80.jpg", "sbt_code": "T1" , "proof_type" : "proof_of_personhood" }}}' --from nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l --gas 100000000 --keyring-backend test 
osmosisd q tx CCDBC03684655596B95E13E56F67E97A553981514F2F8D6E1B81B27E09C43AE1 | jq
```

### USer checks his balance in nft contract

```bash 
osmosisd query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"num_tokens":{}}'
```

```bash
osmosisd query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"tokens":{"owner": "nibi1j8099d7vm6w6t8vmzh6t3vanxt2hnf99vp7z7l"}}'
```

```bash
osmosisd query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"all_nft_info":{"token_id": "1"}}'
```

```bash
osmosisd query wasm contract-state smart nibi1afk9dd5rdqsjasz3v73jurufujfwm57jc7eznl8udwx9je0d34fqvhkkjx '{"nft_info":{"token_id": "1"}}'
```
