
ISSUER=issuer-6
USER=user-6

CREATE_ACCOUNT(){
    U=$1
    echo 'Creating Account  ...'
    USER_ACCOUNT=$(nibid keys add $U --keyring-backend test  | jq -r '.address')
    echo "Sending balance to $U: ${USER_ACCOUNT} ..."
    TXHASH="$(nibid tx bank send validator $USER_ACCOUNT 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0 -y | jq -r '.txhash')"
    sleep 2
    echo "Transction Successful TXHASH: ${TXHASH}"
    echo 
}

CREATE_ACCOUNT $ISSUER
CREATE_ACCOUNT $USER






# echo 'Creating USER Account  ...'
# USER_ACCOUNT=$(nibid keys add $USER --keyring-backend test  | jq -r '.address')
# echo "Sending balance to USER_ACCOUNT: ${USER_ACCOUNT} ..."
# TXHASH="$(nibid tx bank send validator $USER_ACCOUNT 10000000unibi --keyring-backend test --chain-id nibiru-localnet-0 -y |  jq -r '.txhash')"
# echo "Transction Successful TXHASH: ${TXHASH}"
# echo 
# echo 