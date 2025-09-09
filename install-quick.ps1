# CLI Frontend Generator - Quick Install Script for Windows
# Downloads precompiled binary from GitHub releases

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

# Configuration
$RepoOwner = "FrancoCastro1990"
$RepoName = "CLI-FRONTEND-RUST"

Write-ColorOutput $Blue "🚀 CLI Frontend Generator - Quick Installer"
Write-ColorOutput $Blue "=============================================="

# Download and execute the full Windows installer
Write-ColorOutput $Blue "📥 Downloading Windows installer..."
try {
    $InstallerUrl = "https://github.com/$RepoOwner/$RepoName/releases/latest/download/install-windows.ps1"
    $TempScript = [System.IO.Path]::GetTempFileName() + ".ps1"
    
    Write-ColorOutput $Blue "⬇️  Downloading from: $InstallerUrl"
    Invoke-WebRequest -Uri $InstallerUrl -OutFile $TempScript -UseBasicParsing
    
    Write-ColorOutput $Blue "▶️  Executing installer..."
    & $TempScript -InstallPath $InstallPath
    
    # Cleanup
    Remove-Item $TempScript -ErrorAction SilentlyContinue
    Write-ColorOutput $Green "✅ Installation completed successfully!"
    
} catch {
    Write-ColorOutput $Red "❌ Failed to download or execute installer: $($_.Exception.Message)"
    Write-ColorOutput $Yellow "💡 Please try manual installation from:"
    Write-ColorOutput $Blue "https://github.com/$RepoOwner/$RepoName/releases/latest"
    exit 1
}