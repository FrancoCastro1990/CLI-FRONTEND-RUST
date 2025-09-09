# PowerShell Installation Script for CLI Frontend Generator
# Author: Franco Castro

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\Programs\cli-frontend"
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

# Repository configuration
$RepoOwner = "FrancoCastro1990"
$RepoName = "CLI-FRONTEND-RUST"

Write-ColorOutput $Blue "🚀 CLI Frontend Generator - Windows Installer"
Write-ColorOutput $Blue "=============================================="

# Check if we're compiling from source or downloading precompiled
if (Test-Path ".\Cargo.toml") {
    Write-ColorOutput $Blue "📦 Compiling from source..."
    
    # Check if cargo is available
    try {
        cargo --version | Out-Null
    } catch {
        Write-ColorOutput $Red "❌ Cargo not found. Please install Rust first:"
        Write-ColorOutput $Blue "Visit: https://rustup.rs/"
        exit 1
    }
    
    Write-ColorOutput $Blue "🔨 Building project with cargo build --release..."
    try {
        cargo build --release
        Write-ColorOutput $Green "✅ Build completed successfully!"
    } catch {
        Write-ColorOutput $Red "❌ Build failed"
        exit 1
    }
    
    # Create installation directory
    Write-ColorOutput $Blue "📁 Creating installation directory: $InstallPath"
    if (-not (Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }
    
    # Copy binary
    $BinarySource = ".\target\release\cli-frontend.exe"
    $BinaryDest = Join-Path $InstallPath "cli-frontend.exe"
    Write-ColorOutput $Blue "📋 Copying binary to installation directory..."
    Copy-Item $BinarySource $BinaryDest -Force
    
    # Copy templates and architectures
    if (Test-Path ".\templates") {
        $DestTemplates = Join-Path $InstallPath "templates"
        Write-ColorOutput $Blue "📄 Copying templates..."
        if (Test-Path $DestTemplates) {
            Remove-Item $DestTemplates -Recurse -Force
        }
        Copy-Item ".\templates" $DestTemplates -Recurse -Force
    }
    
    if (Test-Path ".\architectures") {
        $DestArchitectures = Join-Path $InstallPath "architectures"
        Write-ColorOutput $Blue "🏗️  Copying architecture configurations..."
        if (Test-Path $DestArchitectures) {
            Remove-Item $DestArchitectures -Recurse -Force
        }
        Copy-Item ".\architectures" $DestArchitectures -Recurse -Force
    }
    
    Write-ColorOutput $Green "✅ Installation from source completed!"
    
} else {
    Write-ColorOutput $Blue "📥 Downloading precompiled version..."
    
    # Download and execute the full Windows installer
    try {
        $InstallerUrl = "https://github.com/$RepoOwner/$RepoName/releases/latest/download/install-windows.ps1"
        $TempScript = [System.IO.Path]::GetTempFileName() + ".ps1"
        
        Write-ColorOutput $Blue "⬇️  Downloading installer..."
        Invoke-WebRequest -Uri $InstallerUrl -OutFile $TempScript -UseBasicParsing
        
        Write-ColorOutput $Blue "▶️  Executing installer..."
        & $TempScript -InstallPath $InstallPath
        
        # Cleanup
        Remove-Item $TempScript -ErrorAction SilentlyContinue
        
        Write-ColorOutput $Green "✅ Installation completed!"
        return
        
    } catch {
        Write-ColorOutput $Red "❌ Failed to download or execute installer: $($_.Exception.Message)"
        Write-ColorOutput $Yellow "💡 Please try manual installation from:"
        Write-ColorOutput $Blue "https://github.com/$RepoOwner/$RepoName/releases/latest"
        exit 1
    }
}

# Create configuration file
Write-ColorOutput $Blue "⚙️  Creating configuration file..."
$configContent = @"
# CLI Frontend Generator Configuration
# Global installation configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Paths configuration
templates_dir=$($InstallPath -replace '\\', '/')/templates
architectures_dir=$($InstallPath -replace '\\', '/')/architectures
output_dir=.
default_architecture=screaming-architecture
"@

$configPath = Join-Path $env:USERPROFILE ".cli-frontend.conf"
$configContent | Out-File -FilePath $configPath -Encoding UTF8
Write-ColorOutput $Green "✅ Configuration file created at $configPath"

# Add to PATH if not already present
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

Write-ColorOutput $Green ""
Write-ColorOutput $Green "🎉 Installation completed successfully!"
Write-ColorOutput $Green "======================================"
Write-ColorOutput $Green ""
Write-ColorOutput $Blue "📍 Installation location: $InstallPath"
Write-ColorOutput $Blue "🔧 Binary: $InstallPath\cli-frontend.exe"
Write-ColorOutput $Blue "📄 Templates: $InstallPath\templates"
Write-ColorOutput $Blue "🏗️  Architectures: $InstallPath\architectures"
Write-ColorOutput $Blue "⚙️  Configuration: $configPath"
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