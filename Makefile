SERVER_APP_NAME := qvault
# Define variables
CARGO = cargo

# Check if running inside Docker
IS_DOCKER := $(shell [ -f /.dockerenv ] && echo "yes" || echo "no")

# Check if cargo is installed
HAS_CARGO := $(shell command -v $(CARGO) >/dev/null 2>&1 && echo "yes" || echo "no")

# Ensure cargo is available
ifeq ($(HAS_CARGO),no)
$(warning "Error: cargo is not available. Please install Rust and Cargo.")
endif

# Ensure running inside Docker
ifeq ($(IS_DOCKER),no)
$(warning "Warning: Not running inside Docker. Make sure this is intentional.")
endif

# Print help
help:
	@echo "QueryVault Make options"
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  buildsh   - Shell to build"
	@echo "  build     - Build the project"
	@echo "  run       - Run the executable"
	@echo "  test      - Run tests"
	@echo "  clean     - Clean the build artifacts"
	@echo "  format    - Format the code with rustfmt"
	@echo "  lint      - Lint the code with clippy"
	@echo "  doc       - Generate and open documentation"
	@echo "  help      - Show this help message"

.PHONY: buildsh
buildsh:
ifeq ($(IS_DOCKER),yes)
	$(error "Already in docker container")
endif
	(bash ./dev_container.sh)

.PHONY: build
build:
	(cd $(SERVER_APP_NAME) && cargo build)
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif

start: $(SERVER_APP_NAME)/target/debug build

run:
	(cd $(SERVER_APP_NAME) && cargo run)

.PHONY:clean
clean: stop
	@echo "Cleaning up..."
	rm -rf node_modules package.json package-lock.json
	(cd $(SERVER_APP_NAME) && rm -rf node_modules package-lock.json)
	@echo "Cleanup completed."
