#!/bin/bash

# CLI Frontend Generator - Quick Install Script (Fixed Version)
# Downloads precompiled binary from GitHub releases and creates proper config

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
BINARY_NAME="cli-frontend"

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

echo -e "${BLUE}🚀 Detected OS: ${OS}${NC}"

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

echo -e "${BLUE}🚀 CLI Frontend Generator - Linux Installer (Fixed)${NC}"
echo -e "${BLUE}================================================${NC}"

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

# ⭐ FIXED: Create configuration file with absolute paths
echo -e "${BLUE}⚙️  Creating configuration file...${NC}"
CONFIG_FILE="$HOME/.cli-frontend.conf"

echo -e "${YELLOW}📝 Writing config to: $CONFIG_FILE${NC}"

cat > "$CONFIG_FILE" << 'EOF'
# CLI Frontend Generator Configuration
# Global installation configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Paths configuration (using absolute paths)
templates_dir=/home/USER_PLACEHOLDER/.cli-template/templates
output_dir=.
architectures_dir=/home/USER_PLACEHOLDER/.cli-template/architectures

# Feature settings
default_architecture=screaming-architecture
EOF

# Replace USER_PLACEHOLDER with actual username
sed -i "s/USER_PLACEHOLDER/$(whoami)/g" "$CONFIG_FILE"

echo -e "${GREEN}✅ Configuration file created at: $CONFIG_FILE${NC}"

# Verify the configuration file was created
if [ -f "$CONFIG_FILE" ]; then
    echo -e "${GREEN}✅ Verification: Config file exists${NC}"
    echo -e "${BLUE}📄 Config contents:${NC}"
    cat "$CONFIG_FILE"
else
    echo -e "${RED}❌ ERROR: Config file was not created!${NC}"
fi

echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
echo -e "${BLUE}📍 Binary: ${INSTALL_DIR}/${BINARY_NAME}${NC}"
echo -e "${BLUE}📄 Templates: ${TEMPLATES_DIR}/templates${NC}"
echo -e "${BLUE}🏗️  Architectures: ${TEMPLATES_DIR}/architectures${NC}"
echo -e "${BLUE}⚙️  Config: ${CONFIG_FILE}${NC}"
echo -e "${BLUE}Usage: cli-frontend MyComponent --type component${NC}"
echo -e "${BLUE}Feature: cli-frontend UserAuth --type feature --architecture mvc${NC}"