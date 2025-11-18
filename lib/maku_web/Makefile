.PHONY: build clean install help

# Ensure rustup's toolchain is in PATH
SHELL := /bin/bash
export PATH := $(HOME)/.cargo/bin:$(PATH)

help:
	@echo "Available targets:"
	@echo "  make build   - Build WASM package (default)"
	@echo "  make clean   - Remove built artifacts"
	@echo "  make install - Install wasm-pack if not already installed"
	@echo "  make help    - Show this help message"

build:
	@echo "Building WASM package..."
	wasm-pack build --target web --no-pack --out-dir lib --out-name index

clean:
	@echo "Cleaning build artifacts..."
	rm -rf pkg target

install:
	@command -v wasm-pack >/dev/null 2>&1 || { \
		echo "Installing wasm-pack..."; \
		cargo install wasm-pack; \
	}
	@rustup target list --installed | grep -q wasm32-unknown-unknown || { \
		echo "Adding wasm32-unknown-unknown target..."; \
		rustup target add wasm32-unknown-unknown; \
	}
	@echo "Setup complete!"

# Default target
.DEFAULT_GOAL := build
