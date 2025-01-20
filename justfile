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

test-all: build
    #!/usr/bin/env bash
    set -euxo pipefail

    # Start the server in the background
    npx wrangler dev &
    SERVER_PID=$!

    # Wait for the server to start
    sleep 3

    # Create images directory if it doesn't exist
    mkdir -p images

    # Line Chart
    curl "http://localhost:8787/api?type=line&data=10,20,15,25,30,20,35,40,30,45&colors=%23B3E0FF" \
        -o images/line_chart.png

    # Bar Chart
    curl "http://localhost:8787/api?type=bar&data=10,20,15,25,30,20,35,40,30,45&colors=%23FFB3B3" \
        -o images/bar_chart.png

    # Scatter Plot
    curl "http://localhost:8787/api?type=scatter&data=10,20,15,25,30,20,35,40,30,45&colors=%23FFE6B3" \
        -o images/scatter_plot.png

    # Custom Chart
    curl "http://localhost:8787/api?type=bar&data=10,20,15,25,30&title=Monthly%20Sales%202024&x_label=Month&y_label=Sales%20(millions)&colors=%23E6B3FF" \
        -o images/custom_chart.png

    # Custom Chart Japanese
    curl "http://localhost:8787/api?type=bar&data=10,20,15,25,30&title=月間売上推移%202024年&x_label=月&y_label=売上（百万円）" \
        -o images/custom_chart_ja.png

    # Pie Chart
    curl "http://localhost:8787/api?type=pie&data=30,20,50&labels=A,B,C&colors=%23FFB3B3,%23B3E0FF,%23FFE6B3&title=Distribution" \
        -o images/pie_chart.png

    # Donut Chart
    curl "http://localhost:8787/api?type=donut&data=35,25,40&labels=製品A,製品B,製品C&colors=%23FFB3B3,%23B3E0FF,%23FFE6B3&title=売上構成比" \
        -o images/donut_chart.png

    # Area Chart
    curl "http://localhost:8787/api?type=area&data=30,40,35,50,45,20,25,30,35,40&labels=Week1,Week2,Week3,Week4,Week5&colors=%23FFB3B3,%23B3E0FF&title=Team%20Performance&x_label=Week&y_label=Score" \
        -o images/area_chart.png

    # Kill the server
    kill $SERVER_PID