# Get crate version by parsing the line that starts with version.
CRATE_VERSION ?= $(shell grep ^version Cargo.toml | awk '{print $$3}')

# Set path to cargo executable
CARGO ?= cargo

PKG_MANAGER ?= $(shell command -v dnf yum|head -n1)
PRE_COMMIT = $(shell command -v bin/venv/bin/pre-commit ~/.local/bin/pre-commit pre-commit | head -n1)

#=================================================
# Testing and validation
#=================================================

.PHONY: validate
validate: ## Validate code
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy -p procsys@$(CRATE_VERSION) -- -D warnings

.PHONY: test
test: extract_test_data ## Run unit tests
	$(CARGO) test

.PHONY: create_test_data
create_test_data: ## create test data fixtures archive
	./test_data/ttar.sh -C ./test_data/ -c -f ./test_data/fixtures.ttar fixtures

.PHONY: extract_test_data
extract_test_data: ## extract test data fixtures archive
	/bin/rm -rf ./test_data/fixtures
	./test_data/ttar.sh -C ./test_data/ -x -f ./test_data/fixtures.ttar

.PHONY: pre-commit
pre-commit:   ## Run pre-commit
ifeq ($(PRE_COMMIT),)
	@echo "FATAL: pre-commit was not found, make .install.pre-commit to installing it." >&2
	@exit 2
endif
	$(PRE_COMMIT) run -a

.PHONY: codespell
codespell: ## Run codespell
	@echo "running codespell"
	@codespell -S ./target,./targets,./test_data -L crate

#=================================================
# Publish crate
#=================================================

.PHONY: crate-publish
crate-publish: ## Publish crate
	@if [ "v$(CRATE_VERSION)" != "$(GIT_TAG)" ]; then\
		echo "Git tag is not equivalent to the version set in Cargo.toml. Please checkout the correct tag";\
		exit 1;\
	fi
	@echo "It is expected that you have already done 'cargo login' before running this command. If not command may fail later"
	$(CARGO) publish --dry-run
	$(CARGO) publish


.PHONY: clean
clean: ## Cleanup
	rm -rf target
	rm -rf ./test_data/fixtures

#=================================================
# Required tools installation tartgets
#=================================================

.PHONY: install.tools
install.tools: .install.pre-commit .install.codespell ## Install needed tools

.PHONY: .install.pre-commit
.install.pre-commit:
	if [ -z "$(PRE_COMMIT)" ]; then \
		python3 -m pip install --user pre-commit; \
	fi

.PHONY: .install.codespell
.install.codespell:
	sudo ${PKG_MANAGER} -y install codespell

#=================================================
# Help menu
#=================================================

_HLP_TGTS_RX = '^[[:print:]]+:.*?\#\# .*$$'
_HLP_TGTS_CMD = grep -E $(_HLP_TGTS_RX) $(MAKEFILE_LIST)
_HLP_TGTS_LEN = $(shell $(_HLP_TGTS_CMD) | cut -d : -f 1 | wc -L)
_HLPFMT = "%-$(_HLP_TGTS_LEN)s %s\n"
.PHONY: help
help: ## Print listing of key targets with their descriptions
	@printf $(_HLPFMT) "Target:" "Description:"
	@printf $(_HLPFMT) "--------------" "--------------------"
	@$(_HLP_TGTS_CMD) | sort | \
		awk 'BEGIN {FS = ":(.*)?## "}; \
			{printf $(_HLPFMT), $$1, $$2}'
