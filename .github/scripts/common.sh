#!/bin/bash
# Common functions for CLI Frontend Generator installers
# Shared across all installation scripts

# Setup colors for consistent output
setup_colors() {
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
}

# Repository configuration
setup_repo_config() {
    REPO_OWNER="FrancoCastro1990"
    REPO_NAME="CLI-FRONTEND-RUST"
    BINARY_NAME="cli-frontend"
}

# Detect system architecture
detect_architecture() {
    case "$(uname -m)" in
        x86_64*)    echo "x86_64";;
        aarch64*)   echo "aarch64";;
        arm64*)     echo "aarch64";;
        *)          echo "x86_64";;
    esac
}

# Detect operating system
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        CYGWIN*)    echo "windows";;
        MINGW*)     echo "windows";;
        *)          echo "unknown";;
    esac
}

# Print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Print header
print_header() {
    local title=$1
    print_status "$BLUE" "🚀 $title"
    print_status "$BLUE" "=============================================="
}

# Create installation directories
create_install_dirs() {
    local install_dir=$1
    local templates_dir=$2
    
    mkdir -p "$install_dir"
    mkdir -p "$templates_dir"
    
    if [[ $? -eq 0 ]]; then
        print_status "$GREEN" "✅ Installation directories created"
    else
        print_status "$RED" "❌ Failed to create installation directories"
        exit 1
    fi
}

# Download latest release info from GitHub API
get_latest_release() {
    local repo_owner=$1
    local repo_name=$2
    
    print_status "$BLUE" "🔍 Fetching latest release info..."
    
    local release_info
    if command -v curl >/dev/null 2>&1; then
        release_info=$(curl -s "https://api.github.com/repos/$repo_owner/$repo_name/releases/latest")
    elif command -v wget >/dev/null 2>&1; then
        release_info=$(wget -qO- "https://api.github.com/repos/$repo_owner/$repo_name/releases/latest")
    else
        print_status "$RED" "❌ Neither curl nor wget found"
        exit 1
    fi
    
    if [[ $? -ne 0 ]] || [[ -z "$release_info" ]]; then
        print_status "$RED" "❌ Failed to fetch release information"
        exit 1
    fi
    
    echo "$release_info"
}

# Extract download URL for specific asset
get_download_url() {
    local release_info=$1
    local asset_pattern=$2
    
    local download_url
    download_url=$(echo "$release_info" | grep -o "https://github.com/[^\"]*$asset_pattern[^\"]*" | head -1)
    
    if [[ -z "$download_url" ]]; then
        print_status "$RED" "❌ Could not find download URL for: $asset_pattern"
        exit 1
    fi
    
    echo "$download_url"
}

# Download binary with retry logic
download_binary() {
    local url=$1
    local output_path=$2
    local max_retries=3
    local retry_count=0
    
    print_status "$BLUE" "⬇️  Downloading binary..."
    
    while [[ $retry_count -lt $max_retries ]]; do
        if command -v curl >/dev/null 2>&1; then
            if curl -L -o "$output_path" "$url"; then
                break
            fi
        elif command -v wget >/dev/null 2>&1; then
            if wget -O "$output_path" "$url"; then
                break
            fi
        else
            print_status "$RED" "❌ Neither curl nor wget found"
            exit 1
        fi
        
        retry_count=$((retry_count + 1))
        if [[ $retry_count -lt $max_retries ]]; then
            print_status "$YELLOW" "⚠️  Download failed, retrying in 2 seconds... ($retry_count/$max_retries)"
            sleep 2
        fi
    done
    
    if [[ $retry_count -eq $max_retries ]]; then
        print_status "$RED" "❌ Failed to download binary after $max_retries attempts"
        exit 1
    fi
    
    # Verify download
    if [[ ! -f "$output_path" ]] || [[ ! -s "$output_path" ]]; then
        print_status "$RED" "❌ Downloaded file is empty or doesn't exist"
        exit 1
    fi
    
    chmod +x "$output_path"
    print_status "$GREEN" "✅ Binary downloaded successfully"
}

# Download and extract templates and architectures
download_templates() {
    local repo_owner=$1
    local repo_name=$2
    local install_dir=$3
    
    print_status "$BLUE" "📄 Downloading templates and architectures..."
    
    local templates_url="https://github.com/$repo_owner/$repo_name/archive/main.zip"
    local temp_dir
    temp_dir=$(mktemp -d)
    
    # Download templates
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$temp_dir/templates.zip" "$templates_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$temp_dir/templates.zip" "$templates_url"
    else
        print_status "$RED" "❌ Neither curl nor wget found"
        exit 1
    fi
    
    if [[ $? -ne 0 ]]; then
        print_status "$YELLOW" "⚠️  Failed to download templates"
        rm -rf "$temp_dir"
        return 1
    fi
    
    # Extract templates
    cd "$temp_dir" || exit 1
    
    if command -v unzip >/dev/null 2>&1; then
        unzip -q templates.zip
    else
        print_status "$YELLOW" "⚠️  unzip not found, templates extraction skipped"
        rm -rf "$temp_dir"
        return 1
    fi
    
    # Copy templates and architectures
    local source_dir="$repo_name-main"
    if [[ -d "$source_dir/templates" ]]; then
        cp -r "$source_dir/templates" "$install_dir/"
    fi
    
    if [[ -d "$source_dir/architectures" ]]; then
        cp -r "$source_dir/architectures" "$install_dir/"
    fi
    
    # Cleanup
    rm -rf "$temp_dir"
    
    print_status "$GREEN" "✅ Templates and architectures downloaded successfully"
}

# Add binary to PATH
add_to_path() {
    local binary_dir=$1
    local shell_name=$2
    
    # Determine shell configuration file
    local shell_config=""
    if [[ -n "$shell_name" ]]; then
        case "$shell_name" in
            "bash")
                shell_config="$HOME/.bashrc"
                ;;
            "zsh")
                shell_config="$HOME/.zshrc"
                ;;
            *)
                shell_config="$HOME/.profile"
                ;;
        esac
    else
        # Auto-detect shell
        if [[ -n "$BASH_VERSION" ]]; then
            shell_config="$HOME/.bashrc"
        elif [[ -n "$ZSH_VERSION" ]] || [[ "$SHELL" == */zsh ]]; then
            shell_config="$HOME/.zshrc"
        elif [[ "$SHELL" == */bash ]]; then
            shell_config="$HOME/.bash_profile"
        else
            shell_config="$HOME/.profile"
        fi
    fi
    
    # Check if already in PATH
    if echo "$PATH" | grep -q "$binary_dir"; then
        print_status "$GREEN" "✅ Binary directory already in PATH"
        return 0
    fi
    
    print_status "$BLUE" "🔧 Adding to PATH..."
    
    # Add to PATH
    {
        echo ""
        echo "# CLI Frontend Generator"
        echo "export PATH=\"\$PATH:$binary_dir\""
    } >> "$shell_config"
    
    print_status "$GREEN" "✅ Added to PATH. Please run: source $shell_config"
}

# Create configuration file
create_config_file() {
    local templates_dir=$1
    local architectures_dir=$2
    local config_file="$HOME/.cli-frontend.conf"
    
    print_status "$BLUE" "⚙️  Creating configuration file..."
    
    cat > "$config_file" << EOF
# CLI Frontend Generator Configuration
# Auto-generated by installer

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Paths configuration (using absolute paths)
templates_dir=$templates_dir
output_dir=.
architectures_dir=$architectures_dir

# Feature settings
default_architecture=screaming-architecture
EOF

    if [[ $? -eq 0 ]]; then
        print_status "$GREEN" "✅ Configuration file created at: $config_file"
    else
        print_status "$YELLOW" "⚠️  Could not create configuration file"
    fi
}

# Print installation success message
print_success() {
    local binary_path=$1
    local templates_dir=$2
    local architectures_dir=$3
    local config_file="$HOME/.cli-frontend.conf"
    
    print_status "$GREEN" "🎉 Installation completed successfully!"
    print_status "$BLUE" "📍 Binary: $binary_path"
    print_status "$BLUE" "📄 Templates: $templates_dir"
    print_status "$BLUE" "🏗️  Architectures: $architectures_dir"
    print_status "$BLUE" "⚙️  Configuration: $config_file"
    print_status "$YELLOW" "Usage: cli-frontend MyComponent --type component"
    print_status "$YELLOW" "Feature: cli-frontend UserAuth --type feature --architecture mvc"
}