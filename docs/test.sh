
echo 'Creating All Accounts ----------------------------------------------------------------'
# ISSUER=issuer-1
# echo 'Creating Issuer Account  ...'
# ISSUER_ACCOUNT=$(nibid keys add $ISSUER --keyring-backend test  | jq -r '.address')
# echo "Sending balance to ISSUER_ACCOUNT: ${ISSUER_ACCOUNT} ..."
# TXHASH="$(nibid tx bank send validator $ISSUER_ACCOUNT 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0 -y | jq -r '.txhash')"
# echo "Transction Successful TXHASH: ${TXHASH}"
# echo 
# USER=user-1
# echo 'Creating USER Account  ...'
# USER_ACCOUNT=$(nibid keys add $USER --keyring-backend test  | jq -r '.address')
# echo "Sending balance to USER_ACCOUNT: ${USER_ACCOUNT} ..."
# TXHASH="$(nibid tx bank send validator $USER_ACCOUNT 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0 -y |  jq -r '.txhash')"
# echo "Transction Successful TXHASH: ${TXHASH}"
# echo 
# echo 
echo 'Admin Works ----------------------------------------------------------------'

# TXHASH="$(nibid tx wasm store /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts/cw721_base.wasm --from validator --gas 100000000 -y | jq -rcs '.[0].txhash')"
# echo "Transction Successful TXHASH: ${TXHASH}"
# echo "CW_721_CONTRACT_CODE_ID: ${CW_721_CONTRACT_CODE_ID}"
# CW_721_CONTRACT_CODE_ID="$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')"
# echo 

# echo 'Admin uplods SSI_MANAGER_CONTRACT...'
# TXHASH="$(nibid tx wasm store ./artifacts/ssi_manager.wasm --from validator --gas 100000000 -y | jq -rcs '.[0].txhash')"
# echo "Transction Successful TXHASH: ${TXHASH}"
# echo "SSI_MANAGER_CONTRACT_CODE_ID: ${SSI_MANAGER_CONTRACT_CODE_ID}"
# SSI_MANAGER_CONTRACT_CODE_ID="$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')"
# echo 

STORE_CONTRACT_CODE() {
    # Access parameters
    CONTRACT_TYPE=$1
    CONTRACT_ARTIFCATE_PATH=$2    
    #echo "Admin uplods ${CONTRACT_TYPE}..."
    TXHASH="$(nibid tx wasm store $CONTRACT_ARTIFCATE_PATH --from validator --gas 100000000 -y  | jq -rcs '.[0].txhash')"
    #echo "Transction Successful TXHASH: ${TXHASH}"
    sleep 3
    CODE_ID="$(nibid q tx $TXHASH | jq -r '.logs[0].events[1].attributes[1].value')"
    echo $CODE_ID
}

CONTRACT_INSTANTIATION(){
    CONTRACT_CODE=$1
    INSTANTIATION_BODY=$2

    echo $CONTRACT_CODE
    echo "'$INSTANTIATION_BODY'"

    # echo "nibid tx wasm instantiate $CONTRACT_CODE "'$INSTANTIATION_BODY'" --label "Activity" --from validator --gas 100000000 --no-admin -y | jq -rcs '.[0].txhash'"

    TXHASH="$(nibid tx wasm instantiate $CONTRACT_CODE $INSTANTIATION_BODY --label "Activity" --from validator --gas 100000000 --no-admin  | jq -rcs '.[0].txhash')"
    sleep 2
    CONTRACT_ADDRESS="$(nibid q tx $TXHASH  | jq -r '.logs[0].events[1].attributes[0].value')"
    echo $CONTRACT_ADDRESS
}
 
# echo 'Admin uplods CW_721_CONTRACT...'
# CW_721_CONTRACT_CODE_ID=$(STORE_CONTRACT_CODE CW_721_CONTRACT /Users/hermit/code/hm/hs/kyc/cw-nfts/artifacts/cw721_base.wasm)
# echo "CW_721_CONTRACT_CODE_ID: ${CW_721_CONTRACT_CODE_ID}"
# sleep 2

# echo 'Admin uplods SSI_MANAGER_CONTRACT...'
# SSI_MANAGER_CONTRACT_CODE_ID=$(STORE_CONTRACT_CODE SSI_MANAGER_CONTRACT ../artifacts/ssi_manager.wasm)
# echo "SSI_MANAGER_CONTRACT_CODE_ID: ${SSI_MANAGER_CONTRACT_CODE_ID}"
# sleep 2

# echo 'Admin uplods ISSUER_KYC_CONTRACT...'
# ISSUER_KYC_CONTRACT_CODE_ID=$(STORE_CONTRACT_CODE ISSUER_KYC_CONTRACT ../artifacts/issuer_kyc.wasm)
# echo "ISSUER_KYC_CONTRACT_CODE_ID: ${ISSUER_KYC_CONTRACT_CODE_ID}"
# sleep 2

# echo 'Admin uplods HYPERSIGN_FACTORY_CONTRACT...'
# HYPERSIGN_FACTORY_CONTRACT_CODE_ID=$(STORE_CONTRACT_CODE HYPERSIGN_FACTORY_CONTRACT ../artifacts/hypersign_factory.wasm)
# echo "HYPERSIGN_FACTORY_CONTRACT_CODE_ID: ${HYPERSIGN_FACTORY_CONTRACT_CODE_ID}"
# sleep 2


echo 'Admin instantiate SSI_MANAGER_CONTRACT...'
SSI_MANAGER_CONTRACT_CODE_ID=57
SSI_MANAGER_CONTRACT_ADDRESS=$(CONTRACT_INSTANTIATION $SSI_MANAGER_CONTRACT_CODE_ID "'{"owner_did": "did:hid:12313123123", "did_method": "did:hid:testnet" }'")
echo "SSI_MANAGER_CONTRACT_ADDRESS: ${SSI_MANAGER_CONTRACT_ADDRESS}"
sleep 2



