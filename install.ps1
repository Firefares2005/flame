$ErrorActionPreference = "Stop"

$installDir = "$env:USERPROFILE\.flame"
$exePath = "$installDir\flame.exe"
$url = "https://github.com/Firefares2005/flame/releases/latest/download/flame-windows.exe"

Write-Host "Installing Flame..." -ForegroundColor Yellow

New-Item -ItemType Directory -Force -Path $installDir | Out-Null

Write-Host "Downloading..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $url -OutFile $exePath -UseBasicParsing

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "Added to PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "Flame installed!" -ForegroundColor Yellow
Write-Host "   Restart terminal, then type: flame" -ForegroundColor White