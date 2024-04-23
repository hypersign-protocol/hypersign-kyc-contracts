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

## 2. Admin deploys factory contract

```
nibid tx wasm store ./artifacts/hypersign_factory.wasm --from validator --gas 100000000

nibid q wasm list-code 
```
##  3. Admin instantiate the factory contract

```
nibid tx wasm instantiate 34 '{"counter": 0 }' --label "Activity" --from validator --gas 100000000 --no-admin

nibid q wasm list-contract-by-code 34

```

## 4. Issuer onboard himself

```
nibid tx wasm execute nibi1yatzc54ln59caxxnj53rff2s359pezx3hqxpzu2tkyl2f9ud9yvsq60lle '{"onboard_issuer": {"issuer_did":"did:hid:123123123", "issuer_kyc_code_id": 33 }}' --from issuer  --keyring-backend test  --gas 100000000 
```

## 5. Get the Issuer KYC  contract address

```
nibid query wasm contract-state smart nibi1yatzc54ln59caxxnj53rff2s359pezx3hqxpzu2tkyl2f9ud9yvsq60lle '{"get_registered_issuer":{}}'
```