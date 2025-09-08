# PowerShell Installation Script for CLI Frontend Generator
# Author: Franco Castro

param(
    [string]$InstallPath = "$env:USERPROFILE\.cli-template"
)

# Color definitions
$Green = [System.ConsoleColor]::Green
$Red = [System.ConsoleColor]::Red
$Yellow = [System.ConsoleColor]::Yellow
$Blue = [System.ConsoleColor]::Cyan

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

Write-ColorOutput $Blue "🚀 CLI Frontend Generator - Windows Installer"
Write-ColorOutput $Blue "=============================================="

# Check if binary exists, if not try to build it
if (-not (Test-Path ".\target\release\cli-frontend.exe")) {
    Write-ColorOutput $Yellow "⚠️  Binary not found. Attempting to build..."
    
    # Check if cargo is available
    try {
        cargo --version | Out-Null
    } catch {
        Write-ColorOutput $Red "❌ Cargo not found. Please install Rust first:"
        Write-ColorOutput $Blue "Visit: https://rustup.rs/"
        exit 1
    }
    
    # Check if Cargo.toml exists
    if (-not (Test-Path ".\Cargo.toml")) {
        Write-ColorOutput $Red "❌ Cargo.toml not found. Please run this script from the project root directory."
        exit 1
    }
    
    Write-ColorOutput $Blue "🔨 Building project with cargo build --release..."
    try {
        cargo build --release
        Write-ColorOutput $Green "✅ Build completed successfully!"
    } catch {
        Write-ColorOutput $Red "❌ Build failed. Please check for compilation errors."
        exit 1
    }
}

# Create installation directory
Write-ColorOutput $Blue "📁 Creating installation directory: $InstallPath"
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}

# Copy binary
Write-ColorOutput $Blue "📋 Copying binary to installation directory..."
Copy-Item ".\target\release\cli-frontend.exe" "$InstallPath\cli-frontend.exe" -Force

# Copy templates
Write-ColorOutput $Blue "📄 Copying templates..."
if (Test-Path "$InstallPath\templates") {
    Remove-Item -Path "$InstallPath\templates" -Recurse -Force
}
Copy-Item -Path ".\templates" -Destination "$InstallPath\templates" -Recurse -Force

# Copy architectures
Write-ColorOutput $Blue "🏗️  Copying architecture configurations..."
if (Test-Path "$InstallPath\architectures") {
    Remove-Item -Path "$InstallPath\architectures" -Recurse -Force
}
Copy-Item -Path ".\architectures" -Destination "$InstallPath\architectures" -Recurse -Force

# Add to PATH if not already present
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$InstallPath*") {
    Write-ColorOutput $Blue "🔧 Adding to PATH..."
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallPath", "User")
    Write-ColorOutput $Green "✅ Added to PATH. Please restart your terminal or run:"
    Write-ColorOutput $Yellow "`$env:PATH += ';$InstallPath'"
} else {
    Write-ColorOutput $Green "✅ Already in PATH"
}

Write-ColorOutput $Green ""
Write-ColorOutput $Green "🎉 Installation completed successfully!"
Write-ColorOutput $Green "======================================"
Write-ColorOutput $Green ""
Write-ColorOutput $Blue "📍 Installation location: $InstallPath"
Write-ColorOutput $Blue "🔧 Binary: $InstallPath\cli-frontend.exe"
Write-ColorOutput $Blue "📄 Templates: $InstallPath\templates"
Write-ColorOutput $Blue "🏗️  Architectures: $InstallPath\architectures"
Write-ColorOutput $Blue ""
Write-ColorOutput $Yellow "Usage examples:"
Write-ColorOutput $Yellow "  cli-frontend MyComponent --type component"
Write-ColorOutput $Yellow "  cli-frontend MyHook --type hook"
Write-ColorOutput $Yellow "  cli-frontend MyService --type service"
Write-ColorOutput $Yellow "  cli-frontend UserAuth --type feature"
Write-ColorOutput $Yellow "  cli-frontend UserAuth --type feature --architecture mvc"
Write-ColorOutput $Yellow ""
Write-ColorOutput $Blue "For help: cli-frontend --help"

# Verify installation
Write-ColorOutput $Blue "🧪 Verifying installation..."
try {
    $version = & "$InstallPath\cli-frontend.exe" --version
    Write-ColorOutput $Green "✅ Verification successful: $version"
} catch {
    Write-ColorOutput $Yellow "⚠️  Could not verify installation. You may need to restart your terminal."
}