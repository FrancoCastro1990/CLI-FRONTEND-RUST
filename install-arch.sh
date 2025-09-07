#!/bin/bash

# CLI Frontend Generator - Arch Linux/Manjaro Installer
# Author: Franco Castro

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

INSTALL_DIR="$HOME/.cli-template"
SYSTEM_INSTALL_DIR="/usr/local/share/cli-frontend"
BINARY_NAME="cli-frontend"

echo -e "${BLUE}🚀 CLI Frontend Generator - Arch Linux Installer${NC}"
echo -e "${BLUE}====================================================${NC}"

# Check if running as root for system-wide installation
if [[ $EUID -eq 0 ]]; then
    echo -e "${BLUE}👑 Running as root - Installing system-wide${NC}"
    INSTALL_DIR="/usr/local/share/cli-frontend"
    BINARY_DIR="/usr/local/bin"
    IS_SYSTEM_INSTALL=true
else
    echo -e "${BLUE}👤 Running as user - Installing to home directory${NC}"
    BINARY_DIR="$HOME/.local/bin"
    IS_SYSTEM_INSTALL=false
fi

# Function to install system dependencies
install_dependencies() {
    echo -e "${BLUE}📦 Installing dependencies...${NC}"
    
    if command -v pacman >/dev/null 2>&1; then
        echo -e "${BLUE}🎯 Detected Arch Linux/Manjaro (pacman)${NC}"
        
        if [[ $IS_SYSTEM_INSTALL == true ]]; then
            pacman -S --needed --noconfirm rust curl unzip
        else
            echo -e "${YELLOW}⚠️  Please install dependencies manually or run with sudo:${NC}"
            echo -e "${YELLOW}  sudo pacman -S --needed rust curl unzip${NC}"
            
            # Check if dependencies are available
            if ! command -v cargo >/dev/null 2>&1; then
                echo -e "${RED}❌ Cargo not found. Installing Rust with rustup...${NC}"
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source ~/.cargo/env
            fi
        fi
    elif command -v yay >/dev/null 2>&1; then
        echo -e "${BLUE}🎯 Detected AUR helper (yay)${NC}"
        yay -S --needed rust curl unzip
    elif command -v paru >/dev/null 2>&1; then
        echo -e "${BLUE}🎯 Detected AUR helper (paru)${NC}"
        paru -S --needed rust curl unzip
    else
        echo -e "${YELLOW}⚠️  Could not detect package manager. Please install manually:${NC}"
        echo -e "${YELLOW}  sudo pacman -S --needed rust curl unzip${NC}"
    fi
}

# Check and install dependencies
if ! command -v cargo >/dev/null 2>&1 || ! command -v curl >/dev/null 2>&1; then
    install_dependencies
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

# Create installation directories
echo -e "${BLUE}📁 Creating installation directories...${NC}"
mkdir -p "$INSTALL_DIR"
mkdir -p "$BINARY_DIR"

# Copy binary
echo -e "${BLUE}📋 Installing binary...${NC}"
cp "./target/release/cli-frontend" "$BINARY_DIR/$BINARY_NAME"
chmod +x "$BINARY_DIR/$BINARY_NAME"

# Copy templates
echo -e "${BLUE}📄 Installing templates...${NC}"
if [ -d "$INSTALL_DIR/templates" ]; then
    rm -rf "$INSTALL_DIR/templates"
fi
cp -r "./templates" "$INSTALL_DIR/templates"

# Create desktop entry for system installation
if [[ $IS_SYSTEM_INSTALL == true ]]; then
    echo -e "${BLUE}🖥️  Creating desktop entry...${NC}"
    cat > "/usr/share/applications/cli-frontend.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=CLI Frontend Generator
Comment=Generate React components, hooks, services, and more
Exec=cli-frontend
Icon=code
Terminal=true
Categories=Development;
EOF
fi

# Create symlink configuration for global templates
echo -e "${BLUE}🔗 Setting up template configuration...${NC}"
if [[ $IS_SYSTEM_INSTALL == true ]]; then
    # System-wide templates are available to all users
    echo -e "${GREEN}✅ System templates available at: $INSTALL_DIR/templates${NC}"
else
    # User-specific installation
    mkdir -p "$HOME/.config/cli-frontend"
    cat > "$HOME/.config/cli-frontend/config.toml" << EOF
[templates]
directory = "$INSTALL_DIR/templates"

[output]
base_directory = "./src"
create_directories = true
EOF
fi

# Add to PATH if not already present (user installation only)
if [[ $IS_SYSTEM_INSTALL == false ]]; then
    SHELL_CONFIG=""
    if [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    else
        SHELL_CONFIG="$HOME/.profile"
    fi

    if ! echo "$PATH" | grep -q "$BINARY_DIR"; then
        echo -e "${BLUE}🔧 Adding to PATH in ${SHELL_CONFIG}...${NC}"
        echo "" >> "$SHELL_CONFIG"
        echo "# CLI Frontend Generator" >> "$SHELL_CONFIG"
        echo "export PATH=\"\$PATH:$BINARY_DIR\"" >> "$SHELL_CONFIG"
        echo -e "${GREEN}✅ Added to PATH. Please run: source ${SHELL_CONFIG}${NC}"
    else
        echo -e "${GREEN}✅ Already in PATH${NC}"
    fi
fi

# Create AUR-style package info (optional)
if [[ $IS_SYSTEM_INSTALL == true ]]; then
    mkdir -p "/var/lib/cli-frontend"
    cat > "/var/lib/cli-frontend/PKGINFO" << EOF
pkgname = cli-frontend
pkgver = 1.0.0
pkgdesc = A powerful CLI tool for generating React components, hooks, services, and more
url = https://github.com/FrancoCastro1990/cli-frontend-rust
packager = CLI Frontend Generator Installer
builddate = $(date +%s)
size = $(du -sb "$BINARY_DIR/$BINARY_NAME" "$INSTALL_DIR/templates" | awk '{sum += $1} END {print sum}')
arch = $(uname -m)
license = MIT
depend = 
EOF
fi

echo -e "${GREEN}"
echo -e "🎉 Installation completed successfully!"
echo -e "======================================"
echo -e "${NC}"
echo -e "${BLUE}📍 Installation type: $([ $IS_SYSTEM_INSTALL == true ] && echo "System-wide" || echo "User")${NC}"
echo -e "${BLUE}🔧 Binary: $BINARY_DIR/$BINARY_NAME${NC}"
echo -e "${BLUE}📄 Templates: $INSTALL_DIR/templates${NC}"
echo -e "${BLUE}${NC}"
echo -e "${YELLOW}Usage examples:${NC}"
echo -e "${YELLOW}  cli-frontend MyComponent --type component${NC}"
echo -e "${YELLOW}  cli-frontend MyHook --type hook${NC}"
echo -e "${YELLOW}  cli-frontend MyService --type service${NC}"
echo -e "${YELLOW}${NC}"
echo -e "${BLUE}For help: cli-frontend --help${NC}"

# Arch-specific tips
echo -e "${BLUE}${NC}"
echo -e "${BLUE}📋 Arch Linux specific notes:${NC}"
if [[ $IS_SYSTEM_INSTALL == true ]]; then
    echo -e "${BLUE}  • Installed system-wide (available for all users)${NC}"
    echo -e "${BLUE}  • Desktop entry created${NC}"
    echo -e "${BLUE}  • Package info: /var/lib/cli-frontend/PKGINFO${NC}"
    echo -e "${BLUE}  • To uninstall: rm -rf $BINARY_DIR/$BINARY_NAME $INSTALL_DIR /usr/share/applications/cli-frontend.desktop /var/lib/cli-frontend${NC}"
else
    echo -e "${BLUE}  • Installed for current user only${NC}"
    echo -e "${BLUE}  • Config file: ~/.config/cli-frontend/config.toml${NC}"
    echo -e "${BLUE}  • To uninstall: rm -rf $BINARY_DIR/$BINARY_NAME $INSTALL_DIR ~/.config/cli-frontend${NC}"
fi

# Verify installation
echo -e "${BLUE}🧪 Verifying installation...${NC}"
if "$BINARY_DIR/$BINARY_NAME" --version >/dev/null 2>&1; then
    VERSION=$("$BINARY_DIR/$BINARY_NAME" --version)
    echo -e "${GREEN}✅ Verification successful: ${VERSION}${NC}"
else
    echo -e "${YELLOW}⚠️  Could not verify installation. You may need to reload your shell or logout/login.${NC}"
fi

# Optional: Integration with common Arch tools
if command -v makepkg >/dev/null 2>&1; then
    echo -e "${BLUE}💡 Tip: Consider creating a PKGBUILD for easier management${NC}"
fi

if command -v yay >/dev/null 2>&1 || command -v paru >/dev/null 2>&1; then
    echo -e "${BLUE}💡 Tip: This tool could potentially be packaged for AUR${NC}"
fi