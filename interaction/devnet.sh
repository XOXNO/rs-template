ADDRESS=

PROXY=https://devnet-gateway.xoxno.com
CHAIN_ID=D

PROJECT="./output-docker/template/template.wasm"
PROJECT_VERIFY="./output-docker/template/template-0.0.0.source.json"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 --gas-limit=250000000 \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send || return
}

upgrade() {
    mxpy contract upgrade ${ADDRESS} --bytecode=${PROJECT} --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 --gas-limit=250000000 \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send || return
}


verify() {
    mxpy --verbose contract verify "${ADDRESS}"  \
    --packaged-src=./output-docker/template/template-0.0.0.source.json --verifier-url="https://devnet-play-api.multiversx.com" \
    --docker-image="multiversx/sdk-rust-contract-builder:v8.0.1" --ledger --ledger-account-index=0 --ledger-address-index=0  || return 
}

build() {
    mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v8.0.1"
}