VERSION := $(shell git describe --tags --always --dirty)
PREFIX ?= "/usr/local"
RELEASE_BIN := "./target/release/snipple"

.PHONY: build
build:
	SNIPPLE_VERSION=$(VERSION) cargo build --all-targets

.PHONY: release
release:
	SNIPPLE_VERSION=$(VERSION) cargo build --release

.PHONY: test
test:
	CARGO_TERM_COLOR=always cargo test --verbose --workspace

.PHONY: install
install: release
	install -m 0555 $(RELEASE_BIN) $(PREFIX)/bin
