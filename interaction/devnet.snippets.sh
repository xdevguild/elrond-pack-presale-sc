PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET="./wallets/test.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
######################################################################

TREASURY_WALLET="erd1q9elscglgsnvnjf828444kwya82t7sjgzxzltfyegta5s9muyrzscpux7s"
TREASURY_WALLET_HEX="0x$(erdpy wallet bech32 --decode ${TREASURY_WALLET})"

TOKEN_ID="MEX-450e50"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

EGLD_PRICE_RATE=1000000000000000000  # 1 EGLD
TOKEN_PRICE_RATE=800000000000000000000  # 800 ESDT
START_TIMESTAMP=1650557438
END_TIEMSTAMP=1662757438

BONUS_PERCENTAGES="500000000000000000 0 1000000000000000000 1000 2000000000000000000 1500"

BUY_AMOUNT=1000000000000000000 # 1 EGLD

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-devnet.interaction.json" \
    --metadata-payable \
    --gas-limit=50000000 \
    --arguments ${TREASURY_WALLET_HEX} ${TOKEN_ID_HEX} ${EGLD_PRICE_RATE} ${TOKEN_PRICE_RATE} ${START_TIMESTAMP} ${END_TIEMSTAMP}
    
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=50000000 \
    --arguments ${TREASURY_WALLET_HEX} ${TOKEN_ID_HEX} ${EGLD_PRICE_RATE} ${TOKEN_PRICE_RATE} ${START_TIMESTAMP} ${END_TIEMSTAMP}
}

addBonusPercentages() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="addBonusPercentages" \
    --arguments ${BONUS_PERCENTAGES}
}

buy() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="buy" \
    --value=${BUY_AMOUNT}
}

setTreasuryWallet() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setTreasuryWallet" \
    --arguments ${TREASURY_WALLET_HEX}
}

setTokenId() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setTokenId" \
    --arguments ${TOKEN_ID_HEX}
}

setPriceRates() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setPriceRates" \
    --arguments ${EGLD_PRICE_RATE} ${TOKEN_PRICE_RATE}
}

setStartTimestamp() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setStartTimestamp" \
    --arguments ${START_TIMESTAMP}
}

setEndTimestamp() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setEndTimestamp" \
    --arguments ${END_TIEMSTAMP}
}

recoverToken() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="recoverToken" \
    --arguments ${TOKEN_ID_HEX}
}

# config

viewPresale() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="viewPresale"
}