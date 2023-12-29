VERSION := $(shell git describe --tags --always --dirty)

.PHONY: build
build:
	SNIPPLE_VERSION=$(VERSION) cargo build --all-targets

.PHONY: release
release:
	cargo build --release

.PHONY: test
test:
	CARGO_TERM_COLOR=always cargo test --verbose --workspace
