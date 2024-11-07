
nibid config chain-id cataclysm-1 && nibid config node https://rpc.nibiru.fi:443

https://rpc.nibiru.fi:443

nibi1yya5gl4v6sj3qn0l06ngz0uuxg995mm95xd0z2

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
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from hypersign-admin --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes --node https://rpc.nibiru.fi:443 | jq -rcs '.[0].txhash'
nibid q tx CE05F35B7FA2C312D0FB8BB9C8EA05A6826A1579FBF80DC928E39ABAA977C470 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
https://explorer.nibiru.fi/cataclysm-1/tx/CE05F35B7FA2C312D0FB8BB9C8EA05A6826A1579FBF80DC928E39ABAA977C470
#156
```

### Uploads factory contract code

```bash
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from hypersign-admin --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx AAFB70F4C8573184EDA6556653CF8AE9966E54A4AE71AA4C020AFA58EAEAB732 | jq -rcs '.[0].logs[0].events[1].attributes[1].value'
https://explorer.nibiru.fi/cataclysm-1/tx/AAFB70F4C8573184EDA6556653CF8AE9966E54A4AE71AA4C020AFA58EAEAB732
#157
```


### Uploads SBT Contract code

```bash
## this artifact is present in /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts folder
nibid tx wasm store ./artifacts/hypersign_kyc_token.wasm --from hypersign-admin --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx 0D149A45B328F900888925B32320AC84C820642C48E6274F6BBEEAE6F2009F0D | jq -rcs '.[0].logs[0].events[1].attributes[1].value' 
https://explorer.nibiru.fi/cataclysm-1/tx/0D149A45B328F900888925B32320AC84C820642C48E6274F6BBEEAE6F2009F0D
#158
```

## Admin for Hypersign FActory

### Admin instantiate the factory contract

```bash
nibid tx wasm instantiate 157 '{"counter":0,"did_doc":"[{\"https://www.w3.org/ns/activitystreams#alsoKnownAs\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans\"}],\"https://w3id.org/security#assertionMethod\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1\"}],\"https://w3id.org/security#authenticationMethod\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1\"}],\"https://w3id.org/security#capabilityDelegationMethod\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1\"}],\"https://w3id.org/security#capabilityInvocationMethod\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1\"}],\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans\"}],\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans\",\"https://w3id.org/security#keyAgreementMethod\":[],\"https://www.w3.org/ns/did#service\":[],\"https://w3id.org/security#verificationMethod\":[{\"https://w3id.org/security#controller\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans\"}],\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1\",\"https://w3id.org/security#publicKeyMultibase\":[{\"@type\":\"https://w3id.org/security#multibase\",\"@value\":\"z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans\"}],\"@type\":[\"https://w3id.org/security#Ed25519VerificationKey2020\"]}]}]","did_doc_proof":"[{\"https://w3id.org/security#challenge\":[{\"@value\":\"AgPGfTzahb8Q8zGyr92C6rrsZDTcG13lfzwN1HXlmEc7\"}],\"http://purl.org/dc/terms/created\":[{\"@type\":\"http://www.w3.org/2001/XMLSchema#dateTime\",\"@value\":\"2024-11-07T02:59:29Z\"}],\"https://w3id.org/security#domain\":[{\"@value\":\"hypersign.id\"}],\"https://w3id.org/security#proofPurpose\":[{\"@id\":\"https://w3id.org/security#authenticationMethod\"}],\"@type\":[\"https://w3id.org/security#Ed25519Signature2020\"],\"https://w3id.org/security#verificationMethod\":[{\"@id\":\"did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1\"}]}]","signature":"z2K94UHYbcQouKp5U4uDJv9XGu5pvXi3m5jZQE6MByYJrotSdnkGJ6FBTgR21wnBjc2uf82eeXNTAsJMsdfCYE5KB","kyc_contract_code_id":156}' --label "Hypersign KYC Factory" --from hypersign-admin --gas-prices 0.025unibi --gas auto --gas-adjustment 1.3  --yes --admin hypersign-admin | jq -rcs '.[0].txhash' 
nibid q tx 34601EC40E5C42812303A8B9F6269B31E44C2B3896726B348A74A77B3F028F03 | jq -rcs '.[0].logs[0].events[1].attributes[0].value'
https://explorer.nibiru.fi/cataclysm-1/tx/34601EC40E5C42812303A8B9F6269B31E44C2B3896726B348A74A77B3F028F03
#nibi1chlnet6j4a0l2j78u4nrdklf4r3qr46jd6hw4t88m4z7eq573f6sf2kcd7
```
==============

### Query hypersign admin did

```bash
nibid query wasm contract-state smart nibi1chlnet6j4a0l2j78u4nrdklf4r3qr46jd6hw4t88m4z7eq573f6sf2kcd7 '{"get_hypersign_admin_d_i_d":{}}'
```
