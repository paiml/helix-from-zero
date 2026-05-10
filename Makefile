SHELL := /bin/bash
.DELETE_ON_ERROR:
.SUFFIXES:

HELIX ?= helix
INSTANCE ?= dev

.PHONY: help check compile push start stop status logs clean

help: ## Print available targets
	@awk 'BEGIN {FS = ":.*##"; printf "Targets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 }' "$(MAKEFILE_LIST)"

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

clean: ## Remove compiled workspace artifacts
	rm -rf .helix
