prepare:
	cd contract && rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --features test-support --release  --target wasm32-unknown-unknown
	cd payment_call && cargo build  --release --target wasm32-unknown-unknown
	cd make_offer_call && cargo build  --release --target wasm32-unknown-unknown
	cd bid_call && cargo build  --release --target wasm32-unknown-unknown

	wasm-strip payment_call/target/wasm32-unknown-unknown/release/payment-call.wasm 2>/dev/null | true
	wasm-strip make_offer_call/target/wasm32-unknown-unknown/release/make-offer-call.wasm 2>/dev/null | true
	wasm-strip contract/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
	wasm-strip bid_call/target/wasm32-unknown-unknown/release/bid-call.wasm 2>/dev/null | true

deploy-testnet: build-contract
	casper-client put-deploy \
  --node-address https://cspr-testnet.mystra.io:7778 \
  --chain-name casper-test \
  --secret-key secret_key.pem \
  --payment-amount 450000000000 \
  --session-path contract/target/wasm32-unknown-unknown/release/contract.wasm


deploy-cep78: build-contract
	casper-client put-deploy \
  --node-address https://cspr-testnet.mystra.io:7778 \
  --chain-name casper-test \
  --secret-key secret_key.pem \
  --payment-amount 660000000000 \
  --session-arg "collection_name:string='MystraStudio_CEP78'" \
  --session-arg "collection_symbol:string='MS_CEP78'" \
  --session-arg "total_token_supply:u64='999999'" \
  --session-arg "ownership_mode:u8='2'" \
  --session-arg "nft_kind:u8='1'" \
  --session-arg "json_schema:string=''" \
  --session-arg "identifier_mode:u8='0'" \
  --session-arg "nft_metadata_kind:u8='2'" \
  --session-arg "events_mode:u8='2'" \
  --session-arg "minting_mode:u8='1'" \
  --session-arg "metadata_mutability:u8='0'" \
  --session-path ./cep78-token.wasm

test: build-contract
	mkdir -p tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cp payment_call/target/wasm32-unknown-unknown/release/payment-call.wasm tests/wasm
	cp make_offer_call/target/wasm32-unknown-unknown/release/make-offer-call.wasm tests/wasm
	cp bid_call/target/wasm32-unknown-unknown/release/bid-call.wasm tests/wasm

	cd tests && cargo  test 

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
