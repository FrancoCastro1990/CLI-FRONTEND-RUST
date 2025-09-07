# CLI Frontend Generator - Quick Install Script for Windows
# Downloads precompiled binary from GitHub releases

param(
    [string]$InstallPath = "$env:USERPROFILE\.cli-template"
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

# Configuration
$RepoOwner = "FrancoCastro1990"
$RepoName = "CLI-FRONTEND-RUST"
$BinaryName = "cli-frontend.exe"
$AssetName = "cli-frontend-windows-x86_64.exe"

Write-ColorOutput $Blue "🚀 CLI Frontend Generator - Quick Installer"
Write-ColorOutput $Blue "=============================================="

# Get latest release info
Write-ColorOutput $Blue "🔍 Fetching latest release info..."
try {
    $LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
} catch {
    Write-ColorOutput $Red "❌ Failed to fetch release information"
    Write-ColorOutput $Yellow "💡 Falling back to source compilation..."
    
    if (Test-Path ".\Cargo.toml") {
        & ".\install.ps1" -InstallPath $InstallPath
        exit $LASTEXITCODE
    } else {
        Write-ColorOutput $Red "❌ Please install from source or check your internet connection"
        exit 1
    }
}

$DownloadUrl = $null
foreach ($asset in $LatestRelease.assets) {
    if ($asset.name -eq $AssetName) {
        $DownloadUrl = $asset.browser_download_url
        break
    }
}

if (-not $DownloadUrl) {
    Write-ColorOutput $Yellow "⚠️  Precompiled binary not available"
    Write-ColorOutput $Yellow "💡 Falling back to source compilation..."
    
    if (Test-Path ".\Cargo.toml") {
        & ".\install.ps1" -InstallPath $InstallPath
        exit $LASTEXITCODE
    } else {
        Write-ColorOutput $Red "❌ Please clone the repository and compile from source"
        exit 1
    }
}

# Create installation directory
Write-ColorOutput $Blue "📁 Creating installation directory: $InstallPath"
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}

# Download binary
Write-ColorOutput $Blue "⬇️  Downloading $AssetName..."
$BinaryPath = Join-Path $InstallPath $BinaryName
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $BinaryPath -UseBasicParsing
    Write-ColorOutput $Green "✅ Binary downloaded successfully"
} catch {
    Write-ColorOutput $Red "❌ Failed to download binary: $($_.Exception.Message)"
    exit 1
}

# Download templates
Write-ColorOutput $Blue "📄 Downloading templates..."
$TemplatesUrl = "https://github.com/$RepoOwner/$RepoName/archive/main.zip"
$TempDir = [System.IO.Path]::GetTempPath()
$ZipPath = Join-Path $TempDir "templates.zip"

try {
    Invoke-WebRequest -Uri $TemplatesUrl -OutFile $ZipPath -UseBasicParsing
    
    # Extract templates
    $ExtractPath = Join-Path $TempDir "cli-frontend-temp"
    if (Test-Path $ExtractPath) {
        Remove-Item $ExtractPath -Recurse -Force
    }
    
    Expand-Archive -Path $ZipPath -DestinationPath $ExtractPath -Force
    
    $SourceTemplatesPath = Join-Path $ExtractPath "CLI-FRONTEND-RUST-main\templates"
    $DestTemplatesPath = Join-Path $InstallPath "templates"
    
    if (Test-Path $DestTemplatesPath) {
        Remove-Item $DestTemplatesPath -Recurse -Force
    }
    
    Copy-Item -Path $SourceTemplatesPath -Destination $DestTemplatesPath -Recurse -Force
    
    # Cleanup
    Remove-Item $ZipPath -Force -ErrorAction SilentlyContinue
    Remove-Item $ExtractPath -Recurse -Force -ErrorAction SilentlyContinue
    
    Write-ColorOutput $Green "✅ Templates downloaded successfully"
} catch {
    Write-ColorOutput $Yellow "⚠️  Failed to download templates: $($_.Exception.Message)"
    Write-ColorOutput $Yellow "💡 Please manually download templates from GitHub"
}

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
Write-ColorOutput $Blue "🔧 Binary: $InstallPath\$BinaryName"
Write-ColorOutput $Blue "📄 Templates: $InstallPath\templates"
Write-ColorOutput $Blue ""
Write-ColorOutput $Yellow "Usage examples:"
Write-ColorOutput $Yellow "  cli-frontend MyComponent --type component"
Write-ColorOutput $Yellow "  cli-frontend MyHook --type hook"
Write-ColorOutput $Yellow "  cli-frontend MyService --type service"
Write-ColorOutput $Yellow ""
Write-ColorOutput $Blue "For help: cli-frontend --help"

# Verify installation
Write-ColorOutput $Blue "🧪 Verifying installation..."
try {
    $version = & $BinaryPath --version
    Write-ColorOutput $Green "✅ Verification successful: $version"
} catch {
    Write-ColorOutput $Yellow "⚠️  Could not verify installation. You may need to restart your terminal."
}