VERSION := $(shell git describe --tags --always --dirty)
BUILD_DAY := $(shell date -u +"%Y-%m-%d")
MANPAGE := man/snipple.1
PREFIX ?= "/usr/local"
RELEASE_BIN := "./target/release/snipple"

.PHONY: man
man: $(MANPAGE)

$(MANPAGE): $(MANPAGE).md
	sed "s/VERSION_PLACEHOLDER/${VERSION}/g" $< | \
	 	sed "s/DATE_PLACEHOLDER/${BUILD_DAY}/g" | \
	 	pandoc --standalone -f markdown -t man -o $@


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
install: release man
	install -d $(PREFIX)/bin
	install -m 0555 $(RELEASE_BIN) $(PREFIX)/bin
	install -d $(PREFIX)/share/man/man1/
	install -m 0644 $(MANPAGE) $(PREFIX)/share/man/man1/

.PHONY: local-install
local-install:
	$(MAKE) install PREFIX=usr/local
