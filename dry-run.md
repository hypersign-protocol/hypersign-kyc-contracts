## Issuer IDentity
```
nibid keys add issuer --keyring-backend test

issuer: {
    address: "nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg",
    name: "issuer",
    mnemonic: "fossil away enjoy victory conduct position window torch middle grab maple head scheme kick idle shoe width monkey village spawn goddess ankle parrot knife"
}

nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi125kz6d2cn5m7e3eag4s7r6lwvpvvllleyh2pvg 
```




## User Identity
```
nibid keys add user --keyring-backend test

user: {
    address: "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5",
    name: "user",
    mnemonic: "mind rate breeze party huge brain solar upon budget find opinion sketch submit awkward evil throw phrase umbrella night person improve ribbon siren cute"
}

nibid tx bank send nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 1000000unibi --keyring-backend test --chain-id nibiru-localnet-0

nibid q bank balances nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5 

balances:
- amount: "100000"
  denom: unibi
pagination:
  next_key: null
  total: "0"
```

----------- ADMIN --------------------------------

## 0. Admin uploads SSI manager code



```
nibid tx wasm store ./artifacts/ssi_manager.wasm --from validator --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes | jq -rcs '.[0].txhash'
nibid q tx CDAC5306FE7E4F2090E02D4A87115FB1B48B4B312DB8304F8C601E07636E08F5 | jq
```

```
nibid tx wasm instantiate 1276 '{"owner_did": "did:hid:12313123123", "did_method": "did:hid:testnet"  }' --label "SSI_Manager_Instantiate" --from nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl  --gas 600000000 --gas-adjustment 1.5 --gas-prices 0.030unibi --no-admin

nibid q wasm list-contract-by-code 1276
```


## 1. Admin uploads kyc contract code

```
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas 100000000
nibid q wasm list-code 
```

## 2. Admin uploads factory contract code

```
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas 100000000
nibid q wasm list-code 
```


## 3. Admin instantiate the factory contract

```
nibid tx wasm instantiate 4 '{"counter": 0 }' --label "Activity" --from validator --gas 100000000 --no-admin
nibid q wasm list-contract-by-code 5


nibid tx wasm instantiate 4 '{"adminDID": "did:dhid:123123", adminDIDProof: "{ keyc schem... }" }' --label "Activity" --from validator --gas 100000000 --no-admin
```

----------- ISSUER --------------------------------
## 4. Issuer onboard himself

```
nibid tx wasm execute nibi1f5djultkcmtxwyyadkjjjjmcncxf5yxz5qkz4qfjnkwqggrw7pdqe27m2h '{"onboard_issuer": {"issuer_did":"rnadomds...", proofOfAdmin: "" , "issuer_kyc_code_id": 33 }}' --from issuer  --keyring-backend test  --gas 100000000 
```

## 5. Get the Issuer KYC  contract address

```
nibid query wasm contract-state smart nibi1f5djultkcmtxwyyadkjjjjmcncxf5yxz5qkz4qfjnkwqggrw7pdqe27m2h '{"get_registered_issuer":{}}'
```

```
nibid q wasm list-contract-by-code 36
```

kyc_contract_addr: 
nibi1vw93hy8tm3xekpz9286428gesmmc8dqxmw8cujsh3fcu3rt0hvdqg6tj60

## 6. Issuer initialize SBT contract
```
nibid tx wasm execute nibi1vw93hy8tm3xekpz9286428gesmmc8dqxmw8cujsh3fcu3rt0hvdqg6tj60 '{"init": {"token_code_id": 21}}' --from issuer  --keyring-backend test --gas 100000000 
```

## 7. Get SBT contract address

```
nibid query wasm contract-state smart nibi1vw93hy8tm3xekpz9286428gesmmc8dqxmw8cujsh3fcu3rt0hvdqg6tj60 '{"s_b_t_contract_address":{}}'
```


----------- USERS -------------------------------- 

## 8. User mints NFT
```
nibid tx wasm execute nibi1mgj39ylnnpv0mjtyl4kh7j57ktfymygsug02jxs593sdpllfw0as90t662 '{"mint": {}}' --from user --gas 100000000 
```

## 9. Check status of NFT in the NFT contract

```
nibid query wasm contract-state smart nibi15k76p44rfhep7qjvgq9unek8037km0e45mrexknf340hsua6penqyhd76a '{"num_tokens":{}}'
```

```
nibid query wasm contract-state smart nibi15k76p44rfhep7qjvgq9unek8037km0e45mrexknf340hsua6penqyhd76a '{"tokens":{"owner": "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5"}}'
```

```
nibid query wasm contract-state smart nibi15k76p44rfhep7qjvgq9unek8037km0e45mrexknf340hsua6penqyhd76a '{"all_nft_info":{"token_id": "1"}}'
```

<!-- 
nibid query wasm contract-state smart nibi17yw4nya4kmqgn0aw4xfecrwc098260n6cnqz72afnmhqxrjp6j3shhpd20 '{"all_nft_info":{"token_id": "1"}}' -->

---
