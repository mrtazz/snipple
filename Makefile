.PHONY: build
build:
	cargo build --all-targets

.PHONY: test
test:
	CARGO_TERM_COLOR=always cargo test --verbose --workspace
