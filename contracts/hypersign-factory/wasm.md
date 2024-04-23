## 1. Issuer IDentity
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

## 2. User Identity
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

## 3. Admin deploys kyc contract

```
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas 100000000

nibid q wasm list-code 

nibid tx wasm instantiate 30 '{"counter": 0 }' --label "Activity" --from validator --gas 100000000 --no-admin

nibid q wasm list-contract-by-code 30

```

## 4. Issuer onboard himself

```
nibid tx wasm execute nibi1w2tythwkgxjc3v537jup59u7ygxcfcnxwu095d255dddljcyr6nsl07d95 '{"onboard_issuer": {"issuer_did":"did:hid:123123123", "issuer_kyc_code_id": 29 }}' --from validator --gas 100000000 
```

## 5. Check the contract address

```
nibid query wasm contract-state smart nibi1w2tythwkgxjc3v537jup59u7ygxcfcnxwu095d255dddljcyr6nsl07d95 '{"get_registered_issuer":{}}'
```