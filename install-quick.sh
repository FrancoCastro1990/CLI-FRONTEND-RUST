#!/bin/bash

# CLI Frontend Generator - Quick Install Script
# Downloads precompiled binary from GitHub releases

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_OWNER="FrancoCastro1990"
REPO_NAME="CLI-FRONTEND-RUST"
INSTALL_DIR="$HOME/.cli-template"
BINARY_NAME="cli-frontend"

echo -e "${BLUE}🚀 CLI Frontend Generator - Quick Installer${NC}"
echo -e "${BLUE}===============================================${NC}"

# Detect OS and Architecture
OS=""
ARCH=""
case "$(uname -s)" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    CYGWIN*)    OS="windows";;
    MINGW*)     OS="windows";;
    *)          echo -e "${RED}❌ Unsupported operating system: $(uname -s)${NC}"; exit 1;;
esac

case "$(uname -m)" in
    x86_64*)    ARCH="x86_64";;
    aarch64*)   ARCH="aarch64";;
    arm64*)     ARCH="aarch64";;
    *)          ARCH="x86_64"; echo -e "${YELLOW}⚠️  Unknown architecture, defaulting to x86_64${NC}";;
esac

if [ "$OS" = "windows" ]; then
    BINARY_NAME="cli-frontend.exe"
fi

ASSET_NAME="${BINARY_NAME}-${OS}-${ARCH}"
if [ "$OS" = "windows" ]; then
    ASSET_NAME="${BINARY_NAME}-${OS}-${ARCH}.exe"
fi

echo -e "${BLUE}🔍 Detected: ${OS} ${ARCH}${NC}"
echo -e "${BLUE}📦 Asset name: ${ASSET_NAME}${NC}"

# Get latest release info
echo -e "${BLUE}🔍 Fetching latest release info...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest")

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to fetch release information${NC}"
    echo -e "${YELLOW}💡 Falling back to source compilation...${NC}"
    
    # Check if we're in the project directory
    if [ -f "./Cargo.toml" ]; then
        source ./install.sh
        exit $?
    else
        echo -e "${RED}❌ Please install from source or check your internet connection${NC}"
        exit 1
    fi
fi

DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/[^\"]*${ASSET_NAME}[^\"]*" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${YELLOW}⚠️  Precompiled binary not available for ${OS} ${ARCH}${NC}"
    echo -e "${YELLOW}💡 Falling back to source compilation...${NC}"
    
    if [ -f "./Cargo.toml" ]; then
        source ./install.sh
        exit $?
    else
        echo -e "${RED}❌ Please clone the repository and compile from source${NC}"
        exit 1
    fi
fi

# Create installation directory
echo -e "${BLUE}📁 Creating installation directory: ${INSTALL_DIR}${NC}"
mkdir -p "$INSTALL_DIR"

# Download binary
echo -e "${BLUE}⬇️  Downloading ${ASSET_NAME}...${NC}"
if command -v curl >/dev/null 2>&1; then
    curl -L -o "${INSTALL_DIR}/${BINARY_NAME}" "$DOWNLOAD_URL"
elif command -v wget >/dev/null 2>&1; then
    wget -O "${INSTALL_DIR}/${BINARY_NAME}" "$DOWNLOAD_URL"
else
    echo -e "${RED}❌ Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

# Make binary executable
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

# Download templates (from main branch)
echo -e "${BLUE}📄 Downloading templates...${NC}"
TEMPLATES_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/archive/main.zip"
TEMP_DIR=$(mktemp -d)

if command -v curl >/dev/null 2>&1; then
    curl -L -o "${TEMP_DIR}/templates.zip" "$TEMPLATES_URL"
elif command -v wget >/dev/null 2>&1; then
    wget -O "${TEMP_DIR}/templates.zip" "$TEMPLATES_URL"
fi

# Extract templates
if command -v unzip >/dev/null 2>&1; then
    cd "$TEMP_DIR"
    unzip -q templates.zip
    cp -r "CLI-FRONTEND-RUST-main/templates" "$INSTALL_DIR/"
    rm -rf "$TEMP_DIR"
else
    echo -e "${YELLOW}⚠️  unzip not found. Please manually download templates from GitHub${NC}"
fi

# Add to PATH
SHELL_CONFIG=""
if [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
else
    SHELL_CONFIG="$HOME/.profile"
fi

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo -e "${BLUE}🔧 Adding to PATH in ${SHELL_CONFIG}...${NC}"
    echo "" >> "$SHELL_CONFIG"
    echo "# CLI Frontend Generator" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
    echo -e "${GREEN}✅ Added to PATH. Please run: source ${SHELL_CONFIG}${NC}"
else
    echo -e "${GREEN}✅ Already in PATH${NC}"
fi

echo -e "${GREEN}"
echo -e "🎉 Installation completed successfully!"
echo -e "======================================"
echo -e "${NC}"
echo -e "${BLUE}📍 Installation location: ${INSTALL_DIR}${NC}"
echo -e "${BLUE}🔧 Binary: ${INSTALL_DIR}/${BINARY_NAME}${NC}"
echo -e "${BLUE}📄 Templates: ${INSTALL_DIR}/templates${NC}"
echo -e "${BLUE}${NC}"
echo -e "${YELLOW}Usage examples:${NC}"
echo -e "${YELLOW}  cli-frontend MyComponent --type component${NC}"
echo -e "${YELLOW}  cli-frontend MyHook --type hook${NC}"
echo -e "${YELLOW}  cli-frontend MyService --type service${NC}"
echo -e "${YELLOW}${NC}"
echo -e "${BLUE}For help: cli-frontend --help${NC}"

# Verify installation
echo -e "${BLUE}🧪 Verifying installation...${NC}"
if "${INSTALL_DIR}/${BINARY_NAME}" --version >/dev/null 2>&1; then
    VERSION=$("${INSTALL_DIR}/${BINARY_NAME}" --version)
    echo -e "${GREEN}✅ Verification successful: ${VERSION}${NC}"
else
    echo -e "${YELLOW}⚠️  Could not verify installation. You may need to reload your shell.${NC}"
fi