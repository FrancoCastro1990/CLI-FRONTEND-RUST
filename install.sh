#!/bin/bash
# CLI Frontend Generator - Installation Script
# This script installs the CLI tool on Linux/macOS systems

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Repository configuration
REPO_OWNER="FrancoCastro1990"
REPO_NAME="CLI-FRONTEND-RUST"

echo -e "${BLUE}🚀 CLI Frontend Generator - Installation Script${NC}"
echo ""

# Check if we're compiling from source or downloading precompiled
if [ -f "Cargo.toml" ]; then
    echo -e "${BLUE}📦 Compiling from source...${NC}"
    
    # Check if cargo is available
    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${RED}❌ Cargo not found. Please install Rust first:${NC}"
        echo -e "${BLUE}Visit: https://rustup.rs/${NC}"
        exit 1
    fi
    
    # Build the project
    echo -e "${BLUE}🔨 Building project with cargo build --release...${NC}"
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Build completed successfully!${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
    
    # Installation directories
    if [ "$EUID" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
        TEMPLATES_DIR="/usr/local/share/cli-frontend"
    else
        INSTALL_DIR="$HOME/.local/bin"
        TEMPLATES_DIR="$HOME/.cli-template"
    fi
    
    # Create directories
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$TEMPLATES_DIR"
    
    # Copy binary
    cp target/release/cli-frontend "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/cli-frontend"
    
    # Copy templates and architectures
    if [ -d "templates" ]; then
        cp -r templates "$TEMPLATES_DIR/"
    fi
    
    if [ -d "architectures" ]; then
        cp -r architectures "$TEMPLATES_DIR/"
    fi
    
    echo -e "${GREEN}✅ Installation from source completed!${NC}"
    echo -e "${BLUE}📍 Binary: $INSTALL_DIR/cli-frontend${NC}"
    echo -e "${BLUE}📄 Templates: $TEMPLATES_DIR/templates${NC}"
    echo -e "${BLUE}🏗️  Architectures: $TEMPLATES_DIR/architectures${NC}"
    
else
    echo -e "${BLUE}📥 Downloading precompiled version...${NC}"
    
    # Detect OS for platform-specific installer
    OS=""
    case "$(uname -s)" in
        Linux*)     OS="linux";;
        Darwin*)    OS="macos";;
        *)          echo -e "${RED}❌ Unsupported OS: $(uname -s)${NC}"; exit 1;;
    esac
    
    # Download and execute platform-specific installer
    if [ "$OS" = "linux" ]; then
        curl -sSL "https://github.com/$REPO_OWNER/$REPO_NAME/releases/latest/download/install-linux.sh" | bash
    elif [ "$OS" = "macos" ]; then
        curl -sSL "https://github.com/$REPO_OWNER/$REPO_NAME/releases/latest/download/install-macos.sh" | bash
    fi
fi
    USER_INSTALL=true
fi

# Check if binary exists, if not try to build it
if [ ! -f "./target/release/cli-frontend" ]; then
    echo -e "${YELLOW}⚠️  Binary not found. Attempting to build...${NC}"
    
    # Check if cargo is available
    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${RED}❌ Cargo not found. Please install Rust first:${NC}"
        echo -e "${BLUE}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
        exit 1
    fi
    
    # Check if Cargo.toml exists
    if [ ! -f "./Cargo.toml" ]; then
        echo -e "${RED}❌ Cargo.toml not found. Please run this script from the project root directory.${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}🔨 Building project with cargo build --release...${NC}"
    if ! cargo build --release; then
        echo -e "${RED}❌ Build failed. Please check for compilation errors.${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
fi

echo -e "${BLUE}📁 Installing to: $INSTALL_DIR${NC}"
echo -e "${BLUE}📋 Templates to: $TEMPLATES_DIR${NC}"
echo -e "${BLUE}🏗️  Architectures to: $ARCHITECTURES_DIR${NC}"

# Create directories
echo -e "${YELLOW}📁 Creating directories...${NC}"
mkdir -p "$INSTALL_DIR"
mkdir -p "$TEMPLATES_DIR"
mkdir -p "$ARCHITECTURES_DIR"
mkdir -p "$CONFIG_DIR"

# Copy binary
echo -e "${YELLOW}📦 Copying binary...${NC}"
cp "./target/release/cli-frontend" "$INSTALL_DIR/cli-frontend"
chmod +x "$INSTALL_DIR/cli-frontend"

# Copy templates
echo -e "${YELLOW}📋 Copying templates...${NC}"
if [ -d "./templates" ]; then
    cp -r ./templates/* "$TEMPLATES_DIR/"
    echo -e "${GREEN}✅ Templates installed to $TEMPLATES_DIR${NC}"
else
    echo -e "${YELLOW}⚠️  No templates directory found, skipping templates${NC}"
fi

# Copy architectures
echo -e "${YELLOW}🏗️  Copying architecture configurations...${NC}"
if [ -d "./architectures" ]; then
    cp -r ./architectures/* "$ARCHITECTURES_DIR/"
    echo -e "${GREEN}✅ Architectures installed to $ARCHITECTURES_DIR${NC}"
else
    echo -e "${YELLOW}⚠️  No architectures directory found, skipping architectures${NC}"
fi

# Create global configuration
echo -e "${YELLOW}⚙️  Creating configuration...${NC}"
ABS_BASE="$HOME/.cli-template"
ABS_TEMPLATES_DIR="$ABS_BASE/templates"
ABS_ARCHITECTURES_DIR="$ABS_BASE/architectures"
mkdir -p "$ABS_TEMPLATES_DIR"
mkdir -p "$ABS_ARCHITECTURES_DIR"
echo -e "${YELLOW}🛠️  Writing config with absolute paths...${NC}"
cat > "$HOME/.cli-frontend.conf" << EOF
# CLI Frontend Generator Configuration
# Global installation configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Paths configuration (using absolute paths)
templates_dir=$ABS_TEMPLATES_DIR
output_dir=.
architectures_dir=$ABS_ARCHITECTURES_DIR

# Feature settings
default_architecture=screaming-architecture
EOF

echo -e "${GREEN}✅ Configuration created at $HOME/.cli-frontend.conf${NC}"

# Add to PATH if needed (for user installations)
if [ "$USER_INSTALL" = true ] && [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo -e "${YELLOW}📝 Adding $HOME/.local/bin to PATH${NC}"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
    echo -e "${GREEN}✅ PATH updated. Please run 'source ~/.bashrc' or restart your terminal${NC}"
fi

# Test installation
echo -e "${BLUE}🧪 Testing installation...${NC}"
if command -v cli-frontend >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Installation successful!${NC}"
    echo ""
    echo -e "${BLUE}🎉 CLI Frontend Generator is now installed!${NC}"
    echo ""
    echo -e "${YELLOW}Usage examples:${NC}"
    echo "  cli-frontend --help                                    # Show help and available templates"
    echo "  cli-frontend Button --type component                   # Generate a React component"
    echo "  cli-frontend useAuth --type hook                       # Generate a custom hook"
    echo "  cli-frontend UserStore --type store                    # Generate a Redux store"
    echo "  cli-frontend TestApi --type api                        # Generate an API service"
    echo "  cli-frontend UserAuth --type feature                   # Generate a complete feature"
    echo "  cli-frontend UserAuth --type feature --architecture mvc # Generate with MVC architecture"
    echo ""
    echo -e "${YELLOW}Configuration file:${NC} $HOME/.cli-frontend.conf"
    echo -e "${YELLOW}Templates directory:${NC} $TEMPLATES_DIR"
    echo -e "${YELLOW}Architectures directory:${NC} $ARCHITECTURES_DIR"
    echo ""
    echo -e "${GREEN}You can now run 'cli-frontend --help' from any directory!${NC}"
else
    echo -e "${RED}❌ Installation failed. Binary not found in PATH${NC}"
    echo -e "${YELLOW}Please add $INSTALL_DIR to your PATH manually${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}📚 Documentation:${NC}"
echo "  - README.md for general usage"
echo "  - TEMPLATE_GUIDE.md for creating custom templates"
echo "  - ARCHITECTURES_GUIDE.md for architecture patterns"
echo ""
echo -e "${GREEN}🎯 Happy coding!${NC}"