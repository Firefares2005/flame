# Flame installer for Windows
$ErrorActionPreference = "Stop"

$repo = "Firefares2005/flame"
$installDir = "$env:USERPROFILE\.flame"
$exePath = "$installDir\flame.exe"

Write-Host "Installing Flame..." -ForegroundColor Yellow

$release = Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest"
$asset = $release.assets | Where-Object { $_.name -like "*windows*" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "No Windows binary found in latest release." -ForegroundColor Red
    exit 1
}

New-Item -ItemType Directory -Force -Path $installDir | Out-Null

Write-Host "Downloading $($asset.name)..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $exePath

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "Added to PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "Flame installed!" -ForegroundColor Yellow
Write-Host "   Run: flame" -ForegroundColor White
Write-Host ""
Write-Host "Restart your terminal for PATH to take effect." -ForegroundColor Yellow
