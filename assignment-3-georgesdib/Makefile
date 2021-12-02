run-tmp:
	SKIP_WASM_BUILD= cargo run -- --dev --tmp -lruntime=debug

purge:
	cargo run -- purge-chain --dev -y

restart: purge run

build-full:
	cargo build --release

.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: run
run:
	cargo run --release -- --dev --tmp

.PHONY: build
build:
	SKIP_WASM_BUILD= cargo build --release
