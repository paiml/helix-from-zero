SHELL := /bin/bash
.DELETE_ON_ERROR:
.SUFFIXES:

CARGO ?= cargo
HELIX ?= helix
INSTANCE ?= dev
COVERAGE_FLOOR ?= 100

.PHONY: help all check compile push start stop status logs \
	build test fmt fmt-check lint coverage coverage-html \
	verify comply demo clean

help: ## Print available targets
	@awk 'BEGIN {FS = ":.*##"; printf "Targets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2 }' "$(MAKEFILE_LIST)"

all: verify ## Default — fmt-check + lint + test + coverage floor + contract lint

# ---------------------------------------------------------------------
# Helix CLI verbs (lessons 3.1.x)
# ---------------------------------------------------------------------

check: ## helix check — lint schema.hx + queries.hx (lesson 3.1.2)
	$(HELIX) check $(INSTANCE)

compile: ## helix compile — compile queries to workspace artifact (lesson 3.1.2)
	$(HELIX) compile

push: ## helix push dev — deploy to local instance on :6969 (lesson 3.1.3)
	$(HELIX) push $(INSTANCE)

start: ## helix start — start the dev instance
	$(HELIX) start $(INSTANCE)

stop: ## helix stop — stop the dev instance
	$(HELIX) stop $(INSTANCE)

status: ## helix status — show running instances
	$(HELIX) status

logs: ## helix logs — tail the dev instance
	$(HELIX) logs $(INSTANCE)

# ---------------------------------------------------------------------
# Rust workspace gates
# ---------------------------------------------------------------------

build: ## cargo build --workspace
	$(CARGO) build --workspace --all-targets

test: ## cargo test --workspace
	$(CARGO) test --workspace

fmt: ## cargo fmt --all
	$(CARGO) fmt --all

fmt-check: ## cargo fmt --all --check (CI gate)
	$(CARGO) fmt --all -- --check

lint: ## cargo clippy with -D warnings
	$(CARGO) clippy --workspace --all-targets -- -D warnings

coverage: ## cargo llvm-cov with --fail-under-lines $(COVERAGE_FLOOR)
	$(CARGO) llvm-cov --workspace --fail-under-lines $(COVERAGE_FLOOR)

coverage-html: ## cargo llvm-cov with HTML output at target/llvm-cov/html
	$(CARGO) llvm-cov --workspace --html --open

verify: fmt-check lint test coverage ## Full CI gate: fmt-check + clippy + test + coverage floor

pv-lint: ## pv lint contracts/ — formal contract spec linter (cargo install pv)
	pv lint contracts/

comply: ## pmat comply — paiml-wide compliance gate
	pmat comply

# ---------------------------------------------------------------------
# Lesson 5.1.3 demo (requires `helix push dev` to be running)
# ---------------------------------------------------------------------

demo: ## Run the lesson 5.1.3 typed Rust client demo
	HELIX_URL=http://localhost:6969 \
	  $(CARGO) run -p helix-client --example list_top_films

clean: ## Cargo clean + remove .helix workspace artifacts
	$(CARGO) clean
	rm -rf .helix
