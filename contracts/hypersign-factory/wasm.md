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
nibid tx wasm store ./artifacts/counting_contract.wasm --from validator --gas 100000000

nibid q wasm list-code 

nibid tx wasm instantiate 22 '{"counter": 0, "minimal_donation": { "amount": "10", "denom": "unibi"}}' --label "Activity" --from validator --gas 100000000 --no-admin

nibid q wasm list-contract-by-code 22

```
kyc_contract_addr: 
nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg

```
nibid query wasm contract-state smart nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg '{"value":{}}'
```
<!-- 
## 4. Issuer deploys the NFT contract with Kyc_contract_address as an admin

```
nibid tx wasm store ./artifacts/cw721_base.wasm --from issuer --gas 100000000 --keyring-backend test

nibid q wasm list-code 

nibid tx wasm instantiate 12 '{"name": "Kyc SBT", "symbol": "kycsbt", "minter": "nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg"}' --label "Activit" --from issuer --gas 100000000 --no-admin --keyring-backend test

nibid q wasm list-contract-by-code 16

```
issuer contract address: nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j
 -->

## 4. ISsuer deploys NFT contract through KYC

```
nibid tx wasm execute nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg '{"deploy": {"token_code_id": 21}}' --from issuer --gas 100000000  --keyring-backend test
```

### Get the NFT contract address
```
nibid query wasm contract-state smart nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg '{"get_proxy_message":{}}'
```

### Check the minter - it should be kyc contract address
```
nibid query wasm contract-state smart nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j '{"minter":{}}'
```

<!-- ## 5. Admin whitelist nft_contract_address in KYC contract through reg_issuer_contract()

```
nibid tx wasm execute nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg '{"poke": {"proxy_contract_addr": "nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j"}}' --from validator --gas 100000000 --fees=200000unibi

nibid query wasm contract-state smart nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg '{"get_proxy_message":{}}'
``` -->

## 5. User calls mintNFT() - donate of the KYC contract

```
nibid tx wasm execute nibi1durey747cm82sfqm0ph93wk63ljnm4j60euyp2y3krj8dnfqzacq6dh6yg '{"donate": {}}' --from user --amount 10unibi --gas 100000000 --fees=200000unibi --keyring-backend test
```

## 6. Check in NFT contract if user is the owner of the NFT (token - id 2)

```
nibid query wasm contract-state smart nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j '{"owner_of":{"token_id": "2"}}'
```

this is user
```
data:
  approvals: []
  owner: nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5
```


## Return total number of tokens issued
```
nibid query wasm contract-state smart nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j '{"num_tokens":{}}'
```

## Check metadata of the nft 

```
nibid query wasm contract-state smart nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j '{"nft_info":{"token_id": "2"}}'
```

## Check metadata of the nft (all)
```
nibid query wasm contract-state smart nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j '{"all_nft_info":{"token_id": "2"}}'
```

## Check nos tokens owned by a user

```
nibid query wasm contract-state smart nibi1h6828as2z5av0xqtlh4w9m75wxewapk8z9l2flvzc29zeyzhx6fqvy4y7j '{"tokens":{"owner": "nibi13yzstuzzw3ur6lpmn9xh6utx0ym052mq2eagm5"}}'
```

