#!/bin/bash

# API Documentation Generation Script
# Usage: ./scripts/generate-docs.sh [rust|python|javascript|all]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if target is provided
if [ $# -eq 0 ]; then
    TARGET="all"
else
    TARGET=$1
fi

# Validate target
if [[ ! "$TARGET" =~ ^(rust|python|javascript|all)$ ]]; then
    print_error "Invalid target: $TARGET. Must be rust, python, javascript, or all."
    exit 1
fi

print_info "Generating API documentation for: $TARGET"

# Create output directory
mkdir -p docs/api

# Generate Rust documentation
if [[ "$TARGET" == "rust" || "$TARGET" == "all" ]]; then
    print_info "Generating Rust documentation..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    cd rust/cypher_guard
    cargo doc --no-deps --features python-bindings
    cd ../..
    
    # Copy Rust docs to output directory
    mkdir -p docs/api/rust
    cp -r rust/cypher_guard/target/doc/* docs/api/rust/
    
    print_success "Rust documentation generated at docs/api/rust/"
fi

# Generate Python documentation
if [[ "$TARGET" == "python" || "$TARGET" == "all" ]]; then
    print_info "Generating Python documentation..."
    
    if ! command -v uv &> /dev/null; then
        print_error "uv not found. Please install uv: pip install uv"
        exit 1
    fi
    
    cd rust/python_bindings
    uv sync
    
    # Install pdoc if not already installed
    if ! uv run pdoc --version &> /dev/null; then
        print_info "Installing pdoc3..."
        uv add --dev pdoc3
    fi
    
    # Generate Python API docs
    uv run pdoc --html --output-dir ../../docs/api/python cypher_guard
    
    cd ../..
    
    print_success "Python documentation generated at docs/api/python/"
fi

# Generate JavaScript documentation
if [[ "$TARGET" == "javascript" || "$TARGET" == "all" ]]; then
    print_info "Generating JavaScript documentation..."
    
    if ! command -v npm &> /dev/null; then
        print_error "npm not found. Please install Node.js."
        exit 1
    fi
    
    cd rust/js_bindings
    npm ci
    
    # Install typedoc if not already installed
    if ! npx typedoc --version &> /dev/null; then
        print_info "Installing typedoc..."
        npm install --save-dev typedoc
    fi
    
    # Generate TypeScript docs
    npx typedoc --out ../../docs/api/javascript index.d.ts --theme default
    
    cd ../..
    
    print_success "JavaScript documentation generated at docs/api/javascript/"
fi

# Create API documentation index if generating all
if [[ "$TARGET" == "all" ]]; then
    print_info "Creating API documentation index..."
    
    cat > docs/api/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cypher Guard API Documentation</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; text-align: center; margin-bottom: 40px; }
        .docs-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-top: 30px; }
        .doc-card { border: 1px solid #e1e5e9; border-radius: 8px; padding: 20px; text-decoration: none; color: inherit; transition: transform 0.2s, box-shadow 0.2s; }
        .doc-card:hover { transform: translateY(-2px); box-shadow: 0 4px 20px rgba(0,0,0,0.15); }
        .doc-card h3 { margin: 0 0 10px 0; color: #0366d6; }
        .doc-card p { margin: 0; color: #666; line-height: 1.5; }
        .language-badge { display: inline-block; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: bold; margin-bottom: 10px; }
        .rust { background: #dea584; color: #000; }
        .python { background: #3776ab; color: white; }
        .javascript { background: #f7df1e; color: #000; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Cypher Guard API Documentation</h1>
        <p style="text-align: center; color: #666; margin-bottom: 30px;">
            Comprehensive API documentation for all Cypher Guard language bindings
        </p>
        
        <div class="docs-grid">
            <a href="rust/" class="doc-card">
                <span class="language-badge rust">Rust</span>
                <h3>Rust API Reference</h3>
                <p>Complete Rust API documentation with examples, type definitions, and implementation details.</p>
            </a>
            
            <a href="python/" class="doc-card">
                <span class="language-badge python">Python</span>
                <h3>Python API Reference</h3>
                <p>Python bindings documentation with function signatures, exception types, and usage examples.</p>
            </a>
            
            <a href="javascript/" class="doc-card">
                <span class="language-badge javascript">JavaScript/TypeScript</span>
                <h3>JavaScript API Reference</h3>
                <p>TypeScript definitions and JavaScript API documentation with examples and type information.</p>
            </a>
        </div>
        
        <div style="margin-top: 40px; padding: 20px; background: #f8f9fa; border-radius: 8px;">
            <h3>Quick Links</h3>
            <ul>
                <li><a href="https://github.com/neo4j-field/cypher-guard">GitHub Repository</a></li>
                <li><a href="https://github.com/neo4j-field/cypher-guard/releases">Releases</a></li>
                <li><a href="../VERSIONING.md">Versioning Guide</a></li>
                <li><a href="../RELEASES.md">Release Notes</a></li>
            </ul>
        </div>
    </div>
</body>
</html>
EOF

    print_success "API documentation index created at docs/api/index.html"
fi

print_success "API documentation generation complete!"
print_info "Documentation available at: docs/api/"
print_info "Open docs/api/index.html in your browser to view the documentation." 