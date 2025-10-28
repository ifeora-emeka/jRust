#!/bin/bash

set -e

PROJECT_NAME="test-jrust-app"
TEMP_DIR="./temp"
PROJECT_PATH="$TEMP_DIR/$PROJECT_NAME"

echo "=== jRust Test Project Generator ==="
echo ""

if [ -d "$PROJECT_PATH" ]; then
    echo "🧹 Cleaning up existing test project..."
    rm -rf "$PROJECT_PATH"
fi

if [ ! -d "$TEMP_DIR" ]; then
    echo "📁 Creating temp directory..."
    mkdir -p "$TEMP_DIR"
fi

echo "📦 Building jRust CLI..."
cargo build --release --quiet

echo "🚀 Initializing test project: $PROJECT_NAME"
cd "$TEMP_DIR"
../target/release/jrust init "$PROJECT_NAME"

echo ""
echo "📝 Generated project structure:"
cd "$PROJECT_NAME"
find . -type f -name "*.jr" -o -name "jrust.toml" | sort

echo ""
echo "🔍 Checking jRust code..."
../../target/release/jrust check

echo ""
echo "🔨 Building project..."
../../target/release/jrust build

echo ""
echo "▶️  Running project..."
../../target/release/jrust run

echo ""
echo "✅ Test project generation completed successfully!"
echo "📂 Project location: $PROJECT_PATH"
