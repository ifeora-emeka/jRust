#!/bin/bash
# Test jRust Project Generator
# This script creates a test jRust project in ./temp and verifies it works

set -e  # Exit on error

PROJECT_NAME="test-jrust-app"
TEMP_DIR="./temp"
PROJECT_PATH="$TEMP_DIR/$PROJECT_NAME"

echo "=== jRust Test Project Generator ==="
echo ""

# Clean up existing temp directory
if [ -d "$TEMP_DIR" ]; then
    echo "🧹 Cleaning up existing temp directory..."
    rm -rf "$TEMP_DIR"
fi

# Create temp directory
echo "📁 Creating temp directory..."
mkdir -p "$TEMP_DIR"

# Build jRust CLI first
echo "🔨 Building jRust CLI..."
cargo build --bin jrust --quiet

# Initialize new jRust project
echo "🚀 Initializing jRust project: $PROJECT_NAME"
cd "$TEMP_DIR"
../target/debug/jrust init "$PROJECT_NAME"
cd "../"

# Check if project was created
if [ ! -d "$PROJECT_PATH" ]; then
    echo "❌ Error: Project directory not created"
    exit 1
fi

echo "✅ Project created successfully"
echo ""

# Display project structure
echo "📦 Project structure:"
tree -L 2 "$PROJECT_PATH" 2>/dev/null || find "$PROJECT_PATH" -maxdepth 2 -type f -o -type d
echo ""

# Check the jRust source file
echo "📄 Source file (src/index.jr):"
echo "----------------------------------------"
cat "$PROJECT_PATH/src/index.jr"
echo "----------------------------------------"
echo ""

# Run syntax check
echo "🔍 Running syntax check..."
cd "$PROJECT_PATH"
../../target/debug/jrust check
cd "../../"
echo ""

# Build the project
echo "🔨 Building jRust project..."
cd "$PROJECT_PATH"
../../target/debug/jrust build
cd "../../"
echo ""

# Check if Rust code was generated
if [ -d "$PROJECT_PATH/generated" ]; then
    echo "📝 Generated Rust code:"
    echo "----------------------------------------"
    find "$PROJECT_PATH/generated" -name "*.rs" -exec echo "File: {}" \; -exec cat {} \; -exec echo "" \;
    echo "----------------------------------------"
    echo ""
fi

# Run the compiled executable
echo "▶️  Running the compiled program..."
echo "========================================="
cd "$PROJECT_PATH"
../../target/debug/jrust run
cd "../../"
echo "========================================="
echo ""

echo "✅ All tests passed!"
echo ""
echo "Test project location: $PROJECT_PATH"
echo "To manually test: cd $PROJECT_PATH && ../../target/debug/jrust run"
