VERSION := $(shell git describe --tags --always --dirty)
BUILD_DAY := $(shell date -u +"%Y-%m-%d")
MANPAGE := man/snipple.1

.PHONY: man
man: $(MANPAGE)

.PHONY: build
build:
	SNIPPLE_VERSION=$(VERSION) cargo build --all-targets

.PHONY: release
release:
	cargo build --release

.PHONY: test
test:
	CARGO_TERM_COLOR=always cargo test --verbose --workspace

$(MANPAGE): $(MANPAGE).md
	sed "s/VERSION_PLACEHOLDER/${VERSION}/g" $< | \
	 	sed "s/DATE_PLACEHOLDER/${BUILD_DAY}/g" | \
	 	pandoc --standalone -f markdown -t man -o $@
