# List available commands
default:
    @just --list

# Install dependencies
install:
    cargo install worker-build
    cargo install wasm-pack
    npm install

# Start development server
dev:
    npx wrangler dev

# Build the project
build:
    worker-build --release

# Run tests
test:
    cargo test

# Deploy to Cloudflare Workers
deploy:
    npx wrangler deploy

# Clean build artifacts
clean:
    cargo clean
    rm -rf build/
    rm -rf dist/
    rm -rf node_modules/

# Format code
fmt:
    cargo fmt

# Check code format
fmt-check:
    cargo fmt --check

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Build and deploy
release: build deploy
