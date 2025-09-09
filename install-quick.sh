#!/bin/bash
# CLI Frontend Generator - Quick Install Script
# Downloads precompiled binary from GitHub releases

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
REPO_OWNER="FrancoCastro1990"
REPO_NAME="CLI-FRONTEND-RUST"

# Detect OS and Architecture
OS=""
case "$(uname -s)" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    CYGWIN*)    OS="windows";;
    MINGW*)     OS="windows";;
    *)          echo -e "${RED}❌ Unsupported operating system: $(uname -s)${NC}"; exit 1;;
esac

echo -e "${BLUE}🚀 CLI Frontend Generator - Quick Installer${NC}"
echo -e "${BLUE}Detected OS: ${OS}${NC}"

# Redirect to platform-specific installer
if [ "$OS" = "linux" ]; then
    echo -e "${BLUE}📥 Downloading Linux installer...${NC}"
    curl -sSL "https://github.com/$REPO_OWNER/$REPO_NAME/releases/latest/download/install-linux.sh" | bash
elif [ "$OS" = "macos" ]; then
    echo -e "${BLUE}📥 Downloading macOS installer...${NC}"
    curl -sSL "https://github.com/$REPO_OWNER/$REPO_NAME/releases/latest/download/install-macos.sh" | bash
elif [ "$OS" = "windows" ]; then
    echo -e "${RED}❌ For Windows, please use PowerShell:${NC}"
    echo -e "${YELLOW}iwr -useb https://github.com/$REPO_OWNER/$REPO_NAME/releases/latest/download/install-windows.ps1 | iex${NC}"
    exit 1
else
    echo -e "${RED}❌ Unsupported operating system: $OS${NC}"
    exit 1
fi

# Installation directories
INSTALL_DIR="$HOME/.local/bin"
TEMPLATES_DIR="$HOME/.cli-template"

# Create installation directory
mkdir -p "$INSTALL_DIR"
mkdir -p "$TEMPLATES_DIR"

# Get latest release info
echo -e "${BLUE}⬇️  Downloading binary...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest")

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to fetch release information${NC}"
    exit 1
fi

DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/[^\"]*${ASSET_NAME}[^\"]*" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}❌ Precompiled binary not available for ${OS} ${ARCH}${NC}"
    exit 1
fi

# Download binary
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

# Download templates and architectures
echo -e "${BLUE}📄 Downloading templates and architectures...${NC}"
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
    cp -r "CLI-FRONTEND-RUST-main/templates" "$TEMPLATES_DIR/"
    cp -r "CLI-FRONTEND-RUST-main/architectures" "$TEMPLATES_DIR/"
    rm -rf "$TEMP_DIR"
else
    echo -e "${YELLOW}⚠️  unzip not found. Please manually download templates and architectures from GitHub${NC}"
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
    echo "" >> "$SHELL_CONFIG"
    echo "# CLI Frontend Generator" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
fi

# Create configuration file with absolute paths
echo -e "${BLUE}⚙️  Creating configuration file...${NC}"
CONFIG_FILE="$HOME/.cli-frontend.conf"

cat > "$CONFIG_FILE" << EOF
# CLI Frontend Generator Configuration
# Global installation configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Paths configuration (using absolute paths)
templates_dir=$HOME/.cli-template/templates
output_dir=.
architectures_dir=$HOME/.cli-template/architectures

# Feature settings
default_architecture=screaming-architecture
EOF

echo -e "${GREEN}✅ Configuration file created at: $CONFIG_FILE${NC}"

echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
echo -e "${BLUE}📍 Binary: ${INSTALL_DIR}/${BINARY_NAME}${NC}"
echo -e "${BLUE}📄 Templates: ${TEMPLATES_DIR}/templates${NC}"
echo -e "${BLUE}🏗️  Architectures: ${TEMPLATES_DIR}/architectures${NC}"
echo -e "${BLUE}Usage: cli-frontend MyComponent --type component${NC}"
echo -e "${BLUE}Feature: cli-frontend UserAuth --type feature --architecture mvc${NC}"