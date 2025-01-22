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
    NODE_ENV=development npx wrangler dev

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

test-all:
    #!/usr/bin/env bash
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

    # Multi-series Line Chart
    curl -X POST http://localhost:8787 \
        -H "Content-Type: application/json" \
        -d '{
            "graph_type": "line",
            "series": [
                {
                    "name": "Team A",
                    "color": "#FF6384",
                    "data": [
                        {"value": 30}, {"value": 40}, {"value": 35}, {"value": 50}, {"value": 45}
                    ]
                },
                {
                    "name": "Team B",
                    "color": "#36A2EB",
                    "data": [
                        {"value": 20}, {"value": 25}, {"value": 30}, {"value": 35}, {"value": 40}
                    ]
                }
            ],
            "title": "Team Performance Comparison",
            "x_label": "Week",
            "y_label": "Score"
        }' \
        -o images/multi_series_line.png

    # Multi-series Bar Chart
    curl -X POST http://localhost:8787 \
        -H "Content-Type: application/json" \
        -d '{
            "graph_type": "bar",
            "series": [
                {
                    "name": "2023年",
                    "color": "#FF6384",
                    "data": [
                        {"value": 100}, {"value": 120}, {"value": 130}, {"value": 110}
                    ]
                },
                {
                    "name": "2024年",
                    "color": "#36A2EB",
                    "data": [
                        {"value": 110}, {"value": 130}, {"value": 140}, {"value": 120}
                    ]
                }
            ],
            "title": "四半期売上比較",
            "x_label": "四半期",
            "y_label": "売上（百万円）"
        }' \
        -o images/multi_series_bar.png

    # Multi-series Area Chart
    curl -X POST http://localhost:8787 \
        -H "Content-Type: application/json" \
        -d '{
            "graph_type": "area",
            "series": [
                {
                    "name": "Desktop",
                    "color": "#FF6384",
                    "data": [
                        {"value": 50}, {"value": 55}, {"value": 60}, {"value": 58}, {"value": 62}
                    ]
                },
                {
                    "name": "Mobile",
                    "color": "#36A2EB",
                    "data": [
                        {"value": 30}, {"value": 35}, {"value": 40}, {"value": 45}, {"value": 48}
                    ]
                }
            ],
            "title": "デバイス別アクセス数",
            "x_label": "月",
            "y_label": "アクセス数（万）"
        }' \
        -o images/multi_series_area.png

    # Multi-series Radar Chart
    curl -X POST http://localhost:8787 \
        -H "Content-Type: application/json" \
        -d '{
            "graph_type": "radar",
            "series": [
                {
                    "name": "Product A",
                    "color": "#FFB3B3",
                    "data": [
                        {"value": 80, "label": "Quality"},
                        {"value": 70, "label": "Price"},
                        {"value": 90, "label": "Design"},
                        {"value": 85, "label": "Features"},
                        {"value": 75, "label": "Support"}
                    ]
                },
                {
                    "name": "Product B",
                    "color": "#B3E0FF",
                    "data": [
                        {"value": 70, "label": "Quality"},
                        {"value": 85, "label": "Price"},
                        {"value": 75, "label": "Design"},
                        {"value": 80, "label": "Features"},
                        {"value": 90, "label": "Support"}
                    ]
                }
            ],
            "title": "製品比較分析"
        }' \
        -o images/multi_series_radar.png

