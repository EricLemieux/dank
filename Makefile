CMD := dank
INSTALL_DIR := /usr/local/bin

.PHONY: build
build:
	shellcheck dank

.PHONY: install
install: build
	install $(CMD) $(INSTALL_DIR)
	dank --version
