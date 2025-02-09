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

## . Admin deploys kyc contract

```
nibid tx wasm store ./artifacts/issuer_kyc.wasm --from validator --gas 100000000

nibid q wasm list-code 

```
This happens in factory contract
<!-- nibid tx wasm instantiate 31 '{"owner_did": "did:hid:123123123" }' --label "Activity" --from validator --gas 100000000 --no-admin -->
```
nibid q wasm list-contract-by-code 33
```

kyc_contract_addr: 
nibi1ad9jhy5xyclavg0q00g68gxlak0n2my3x0ufmwjmuslyh7nfgv6qd3fwt0

## 6. Issuer initialize SBT contract
```
nibid tx wasm execute nibi1ad9jhy5xyclavg0q00g68gxlak0n2my3x0ufmwjmuslyh7nfgv6qd3fwt0 '{"init": {"token_code_id": 21}}' --from issuer  --keyring-backend test --gas 100000000 
```

## 7. Get SBT contract address

```
nibid query wasm contract-state smart nibi1ad9jhy5xyclavg0q00g68gxlak0n2my3x0ufmwjmuslyh7nfgv6qd3fwt0 '{"s_b_t_contract_address":{}}'
```

## 8. User mints NFT
```
nibid tx wasm execute nibi1ad9jhy5xyclavg0q00g68gxlak0n2my3x0ufmwjmuslyh7nfgv6qd3fwt0 '{"mint": {}}' --from user --gas 100000000 
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



---
