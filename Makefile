MODULE = $(shell go list -m)
VERSION ?= $(shell git describe --tags --always --dirty --match=v* 2> /dev/null || echo "1.0.0")

ENV_FILE ?= .env
DSN ?= $(shell sed -n 's/^CONNECTION_STRING=\(.*\)/\1/p' $(ENV_FILE))

.PHONY: default
default: help

.PHONY: all
all: ## build the rust binary
	make build

# generate help info from comments: thanks to https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help: ## help information about make commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: start
run: ## run the API server
	@RUST_LOG=debug cargo run

.PHONY: build
build:  ## build the API server binary
	@cargo build

.PHONY: clean
clean: ## remove temporary files
	@cargo clean

.PHONY: fix
fix: ## remove temporary files
	@cargo fix --allow-dirty

.PHONY: test
test: ## run all rust unit tests
	@cargo test

.PHONY: version
version: ## display the version of the API server
	@echo $(VERSION)

.PHONY: db-start
db-start: ## start the database server
	@docker-compose -f ./docker-compose.postgres.yml up --build

.PHONY: lint
lint: ## run clippy on all rust package
	@cargo clippy

.PHONY: format
format: ## run "cargo fmt" on all rust packages
	@cargo fmt
