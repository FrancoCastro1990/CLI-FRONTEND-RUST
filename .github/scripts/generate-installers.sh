#!/bin/bash
# Installer Generator for CLI Frontend Generator
# Generates platform-specific installers using common functions

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Initialize
setup_colors
setup_repo_config

# Generate Linux installer
generate_linux_installer() {
    cat << 'EOF'
#!/bin/bash
# CLI Frontend Generator - Linux Quick Installer
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INSTALL_DIR="$HOME/.local/bin"
TEMPLATES_DIR="$HOME/.cli-template"
BINARY_NAME="cli-frontend"

echo -e "${BLUE}🚀 CLI Frontend Generator - Linux Installer${NC}"
echo -e "${BLUE}=============================================${NC}"

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$TEMPLATES_DIR"

# Get latest release
echo -e "${BLUE}⬇️  Downloading binary...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest")

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to fetch release information${NC}"
    exit 1
fi

DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/download/[^\"]*cli-frontend-linux-x86_64[^\"]*" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}❌ Could not find download URL${NC}"
    exit 1
fi

# Download with retry
for i in {1..3}; do
    if curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$DOWNLOAD_URL"; then
        if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]] && [[ -s "$INSTALL_DIR/$BINARY_NAME" ]]; then
            break
        fi
    fi
    echo -e "${YELLOW}⚠️  Download failed, retrying in 2 seconds... ($i/3)${NC}"
    sleep 2
done

chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Download templates
echo -e "${BLUE}📄 Downloading templates and architectures...${NC}"
TEMPLATES_URL="https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/archive/main.zip"
TEMP_DIR=$(mktemp -d)
curl -L -o "$TEMP_DIR/templates.zip" "$TEMPLATES_URL"
cd "$TEMP_DIR"
unzip -q templates.zip
cp -r "CLI-FRONTEND-RUST-main/templates" "$TEMPLATES_DIR/"
cp -r "CLI-FRONTEND-RUST-main/architectures" "$TEMPLATES_DIR/"
rm -rf "$TEMP_DIR"

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
    echo -e "${BLUE}🔧 Adding to PATH...${NC}"
    echo "" >> "$SHELL_CONFIG"
    echo "# CLI Frontend Generator" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
    echo -e "${GREEN}✅ Added to PATH. Please run: source $SHELL_CONFIG${NC}"
fi

# Create configuration
echo -e "${BLUE}⚙️  Creating configuration file...${NC}"
CONFIG_FILE="$HOME/.cli-frontend.conf"

cat > "$CONFIG_FILE" << EOL
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
EOL

echo -e "${GREEN}✅ Configuration file created at: $CONFIG_FILE${NC}"

echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
echo -e "${BLUE}📍 Binary: $INSTALL_DIR/$BINARY_NAME${NC}"
echo -e "${BLUE}📄 Templates: $TEMPLATES_DIR/templates${NC}"
echo -e "${BLUE}🏗️  Architectures: $TEMPLATES_DIR/architectures${NC}"
echo -e "${BLUE}⚙️  Configuration: $CONFIG_FILE${NC}"
echo -e "${YELLOW}Usage: cli-frontend MyComponent --type component${NC}"
echo -e "${YELLOW}Feature: cli-frontend UserAuth --type feature --architecture mvc${NC}"
EOF
}

# Generate macOS installer
generate_macos_installer() {
    cat << 'EOF'
#!/bin/bash
# CLI Frontend Generator - macOS Quick Installer
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INSTALL_DIR="$HOME/.local/bin"
TEMPLATES_DIR="$HOME/.cli-template"
BINARY_NAME="cli-frontend"

echo -e "${BLUE}🚀 CLI Frontend Generator - macOS Installer${NC}"
echo -e "${BLUE}=============================================${NC}"

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    ASSET_NAME="cli-frontend-macos-aarch64"
else
    ASSET_NAME="cli-frontend-macos-x86_64"
fi

echo -e "${BLUE}🔍 Detected architecture: $ARCH${NC}"

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$TEMPLATES_DIR"

# Get latest release
echo -e "${BLUE}⬇️  Downloading binary...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest")

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Failed to fetch release information${NC}"
    exit 1
fi

DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | grep -o "https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/download/[^\"]*$ASSET_NAME[^\"]*" | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}❌ Could not find download URL for $ASSET_NAME${NC}"
    exit 1
fi

# Download with retry
for i in {1..3}; do
    if curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$DOWNLOAD_URL"; then
        if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]] && [[ -s "$INSTALL_DIR/$BINARY_NAME" ]]; then
            break
        fi
    fi
    echo -e "${YELLOW}⚠️  Download failed, retrying in 2 seconds... ($i/3)${NC}"
    sleep 2
done

chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Download templates
echo -e "${BLUE}📄 Downloading templates and architectures...${NC}"
TEMPLATES_URL="https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/archive/main.zip"
TEMP_DIR=$(mktemp -d)
curl -L -o "$TEMP_DIR/templates.zip" "$TEMPLATES_URL"
cd "$TEMP_DIR"
unzip -q templates.zip
cp -r "CLI-FRONTEND-RUST-main/templates" "$TEMPLATES_DIR/"
cp -r "CLI-FRONTEND-RUST-main/architectures" "$TEMPLATES_DIR/"
rm -rf "$TEMP_DIR"

# Add to PATH
SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ] || [ "$SHELL" = "/bin/bash" ]; then
    SHELL_CONFIG="$HOME/.bash_profile"
else
    SHELL_CONFIG="$HOME/.profile"
fi

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo -e "${BLUE}🔧 Adding to PATH...${NC}"
    echo "" >> "$SHELL_CONFIG"
    echo "# CLI Frontend Generator" >> "$SHELL_CONFIG"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
    echo -e "${GREEN}✅ Added to PATH. Please run: source $SHELL_CONFIG${NC}"
fi

# Create configuration
echo -e "${BLUE}⚙️  Creating configuration file...${NC}"
CONFIG_FILE="$HOME/.cli-frontend.conf"

cat > "$CONFIG_FILE" << EOL
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
EOL

echo -e "${GREEN}✅ Configuration file created at: $CONFIG_FILE${NC}"

echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
echo -e "${BLUE}📍 Binary: $INSTALL_DIR/$BINARY_NAME${NC}"
echo -e "${BLUE}📄 Templates: $TEMPLATES_DIR/templates${NC}"
echo -e "${BLUE}🏗️  Architectures: $TEMPLATES_DIR/architectures${NC}"
echo -e "${BLUE}⚙️  Configuration: $CONFIG_FILE${NC}"
echo -e "${YELLOW}Usage: cli-frontend MyComponent --type component${NC}"
echo -e "${YELLOW}Feature: cli-frontend UserAuth --type feature --architecture mvc${NC}"
EOF
}

# Generate Windows installer
generate_windows_installer() {
    cat << 'EOF'
# CLI Frontend Generator - Windows Quick Installer
param(
    [string]$InstallPath = "$env:LOCALAPPDATA\Programs\cli-frontend"
)

# Color functions
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    } else {
        $input | Write-Output
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

$Green = [System.ConsoleColor]::Green
$Red = [System.ConsoleColor]::Red
$Yellow = [System.ConsoleColor]::Yellow
$Blue = [System.ConsoleColor]::Cyan

Write-ColorOutput $Blue "🚀 CLI Frontend Generator - Windows Installer"
Write-ColorOutput $Blue "=============================================="

# Create directories
Write-ColorOutput $Blue "📁 Creating installation directory..."
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}

# Download binary
Write-ColorOutput $Blue "⬇️  Downloading binary..."
try {
    $LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest"
    $DownloadUrl = $null
    foreach ($asset in $LatestRelease.assets) {
        if ($asset.name -eq "cli-frontend-windows-x86_64.exe") {
            $DownloadUrl = $asset.browser_download_url
            break
        }
    }
    
    if (-not $DownloadUrl) {
        Write-ColorOutput $Red "❌ Could not find Windows binary in latest release"
        exit 1
    }
    
    $BinaryPath = Join-Path $InstallPath "cli-frontend.exe"
    
    # Download with retry
    $maxRetries = 3
    $retryCount = 0
    
    do {
        try {
            Invoke-WebRequest -Uri $DownloadUrl -OutFile $BinaryPath -UseBasicParsing
            if ((Test-Path $BinaryPath) -and ((Get-Item $BinaryPath).Length -gt 0)) {
                break
            }
        } catch {
            $retryCount++
            if ($retryCount -lt $maxRetries) {
                Write-ColorOutput $Yellow "⚠️  Download failed, retrying in 2 seconds... ($retryCount/$maxRetries)"
                Start-Sleep -Seconds 2
            }
        }
    } while ($retryCount -lt $maxRetries)
    
    if ($retryCount -eq $maxRetries) {
        Write-ColorOutput $Red "❌ Failed to download binary after $maxRetries attempts"
        exit 1
    }
    
    Write-ColorOutput $Green "✅ Binary downloaded successfully"
} catch {
    Write-ColorOutput $Red "❌ Failed to download binary: $($_.Exception.Message)"
    exit 1
}

# Download templates and architectures
Write-ColorOutput $Blue "📄 Downloading templates and architectures..."
try {
    $TemplatesUrl = "https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/archive/main.zip"
    $TempDir = [System.IO.Path]::GetTempPath()
    $ZipPath = Join-Path $TempDir "templates.zip"
    
    Invoke-WebRequest -Uri $TemplatesUrl -OutFile $ZipPath -UseBasicParsing
    
    $ExtractPath = Join-Path $TempDir "cli-frontend-temp"
    if (Test-Path $ExtractPath) {
        Remove-Item $ExtractPath -Recurse -Force
    }
    
    Expand-Archive -Path $ZipPath -DestinationPath $ExtractPath -Force
    
    $SourceTemplatesPath = Join-Path $ExtractPath "CLI-FRONTEND-RUST-main\templates"
    $DestTemplatesPath = Join-Path $InstallPath "templates"
    
    $SourceArchitecturesPath = Join-Path $ExtractPath "CLI-FRONTEND-RUST-main\architectures"
    $DestArchitecturesPath = Join-Path $InstallPath "architectures"
    
    if (Test-Path $DestTemplatesPath) {
        Remove-Item $DestTemplatesPath -Recurse -Force
    }
    
    if (Test-Path $DestArchitecturesPath) {
        Remove-Item $DestArchitecturesPath -Recurse -Force
    }
    
    Copy-Item -Path $SourceTemplatesPath -Destination $DestTemplatesPath -Recurse -Force
    Copy-Item -Path $SourceArchitecturesPath -Destination $DestArchitecturesPath -Recurse -Force
    
    # Cleanup
    Remove-Item $ZipPath -Force -ErrorAction SilentlyContinue
    Remove-Item $ExtractPath -Recurse -Force -ErrorAction SilentlyContinue
    
    Write-ColorOutput $Green "✅ Templates and architectures downloaded successfully"
} catch {
    Write-ColorOutput $Yellow "⚠️  Failed to download templates and architectures: $($_.Exception.Message)"
}

# Create configuration file
Write-ColorOutput $Blue "⚙️  Creating configuration file..."
$ConfigPath = Join-Path $env:USERPROFILE ".cli-frontend.conf"
$NormalizedInstallPath = $InstallPath -replace '\\\\', '/'

$ConfigLines = @(
    "# CLI Frontend Generator Configuration",
    "# Auto-generated by installer",
    "",
    "# General settings",
    "default_type=component",
    "create_folder=true", 
    "enable_hooks=true",
    "",
    "# Paths",
    "templates_dir=$NormalizedInstallPath/templates",
    "architectures_dir=$NormalizedInstallPath/architectures", 
    "output_dir=.",
    "",
    "# Features",
    "default_architecture=screaming-architecture"
)

try {
    $ConfigLines | Out-File -FilePath $ConfigPath -Encoding UTF8 -Force
    Write-ColorOutput $Green "✅ Configuration file created: $ConfigPath"
} catch {
    Write-ColorOutput $Yellow "⚠️  Could not create configuration file: $($_.Exception.Message)"
}

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$InstallPath*") {
    Write-ColorOutput $Blue "🔧 Adding to user PATH..."
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallPath", "User")
    Write-ColorOutput $Green "✅ Added to user PATH permanently"
    
    # Also add to current session PATH
    $env:PATH += ";$InstallPath"
    Write-ColorOutput $Green "✅ Added to current session PATH"
} else {
    Write-ColorOutput $Green "✅ Already in user PATH"
    # Ensure it's in current session as well
    if ($env:PATH -notlike "*$InstallPath*") {
        $env:PATH += ";$InstallPath"
        Write-ColorOutput $Green "✅ Added to current session PATH"
    }
}

Write-ColorOutput $Green "🎉 Installation completed successfully!"
Write-ColorOutput $Blue "📍 Binary: $InstallPath\cli-frontend.exe"
Write-ColorOutput $Blue "📄 Templates: $InstallPath\templates"
Write-ColorOutput $Blue "🏗️  Architectures: $InstallPath\architectures"
Write-ColorOutput $Yellow "Usage: cli-frontend MyComponent --type component"
Write-ColorOutput $Yellow "Feature: cli-frontend UserAuth --type feature --architecture mvc"

# Test installation
Write-ColorOutput $Blue "🧪 Testing installation..."
try {
    $version = & "$InstallPath\cli-frontend.exe" --version 2>$null
    if ($version) {
        Write-ColorOutput $Green "✅ Installation test successful: $version"
        Write-ColorOutput $Green "✅ You can now use 'cli-frontend' from anywhere!"
    } else {
        Write-ColorOutput $Yellow "⚠️  Installation completed but test failed. Try restarting your terminal."
    }
} catch {
    Write-ColorOutput $Yellow "⚠️  Could not test installation. The binary is installed but may need terminal restart."
}
EOF
}

# Generate universal installer
generate_universal_installer() {
    cat << 'EOF'
#!/bin/bash
# CLI Frontend Generator - Universal Installer
set -e

OS=""
case "$(uname -s)" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    *)          echo "❌ Unsupported OS: $(uname -s)"; exit 1;;
esac

echo "🚀 Detected OS: $OS"

if [ "$OS" = "linux" ]; then
    curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-linux.sh | bash
elif [ "$OS" = "macos" ]; then
    curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-macos.sh | bash
fi
EOF
}

# Generate README
generate_install_readme() {
    cat << 'EOF'
# CLI Frontend Generator - Installation Guide

## Quick Install (Recommended)

### Linux/macOS (One-liner):
```bash
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install.sh | bash
```

### Windows (PowerShell):
```powershell
iwr -useb https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-windows.ps1 | iex
```

## Platform-Specific Installers

### Linux:
```bash
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-linux.sh | bash
```

### macOS:
```bash
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-macos.sh | bash
```

### Windows:
```powershell
iwr -useb https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-windows.ps1 | iex
```

## Manual Installation

1. Download the binary for your platform from the assets below
2. Place it in your PATH
3. Download templates and architectures from the repository
4. Run: `cli-frontend --help`

## Usage

### Basic Templates
```bash
# Create a React component
cli-frontend MyComponent --type component

# Create a custom hook
cli-frontend useAuth --type hook

# Create a service
cli-frontend ApiService --type service
```

### Feature Generation with Architectures
```bash
# Create a complete feature with default structure
cli-frontend UserManagement --type feature

# Create a feature with specific architecture
cli-frontend UserAuth --type feature --architecture mvc
cli-frontend Payment --type feature --architecture clean
cli-frontend Dashboard --type feature --architecture atomic
```

### Available Architectures
- `mvc` - Model-View-Controller
- `mvp` - Model-View-Presenter  
- `mvvm` - Model-View-ViewModel
- `flux` - Flux Architecture
- `redux` - Redux Pattern
- `clean` - Clean Architecture
- `component` - Component-Based
- `atomic` - Atomic Design
- `micro` - Micro-Frontends
- `event` - Event-Driven
- `screaming` - Screaming Architecture

See all available templates: `cli-frontend --help`
EOF
}

# Main function to generate all installers
main() {
    local output_dir=${1:-"./release-assets"}
    
    print_header "Installer Generator"
    
    # Create output directory
    mkdir -p "$output_dir"
    
    # Generate installers
    print_status "$BLUE" "📄 Generating Linux installer..."
    generate_linux_installer > "$output_dir/install-linux.sh"
    chmod +x "$output_dir/install-linux.sh"
    
    print_status "$BLUE" "📄 Generating macOS installer..."
    generate_macos_installer > "$output_dir/install-macos.sh"
    chmod +x "$output_dir/install-macos.sh"
    
    print_status "$BLUE" "📄 Generating Windows installer..."
    generate_windows_installer > "$output_dir/install-windows.ps1"
    
    print_status "$BLUE" "📄 Generating universal installer..."
    generate_universal_installer > "$output_dir/install.sh"
    chmod +x "$output_dir/install.sh"
    
    print_status "$BLUE" "📄 Generating installation README..."
    generate_install_readme > "$output_dir/INSTALL_README.md"
    
    print_status "$GREEN" "✅ All installers generated successfully!"
    print_status "$BLUE" "📁 Output directory: $output_dir"
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi