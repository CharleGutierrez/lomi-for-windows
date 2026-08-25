# Lomi for Windows - Automated Installer Script
# Run this script as Administrator

if (!([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Warning "Please run this script as an Administrator!"
    Pause
    Exit
}

$InstallDir = "C:\Program Files\LOMI"
$ServiceName = "LOMI_Daemon"

Write-Host "🚀 Installing Lomi for Windows..." -ForegroundColor Cyan

# 1. Create Directory
if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# 2. Copy Executable
Write-Host "   └ Copying lomi-win.exe to $InstallDir..."
Copy-Item ".\lomi-win.exe" -Destination "$InstallDir\lomi-win.exe" -Force

# 3. Generate Task Scheduler XML
$xmlContent = @"
<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>LOMI AGI Gateway Daemon for Windows</Description>
  </RegistrationInfo>
  <Triggers>
    <BootTrigger>
      <Enabled>true</Enabled>
    </BootTrigger>
  </Triggers>
  <Actions Context="Author">
    <Exec>
      <Command>C:\Program Files\LOMI\lomi-win.exe</Command>
      <Arguments>serve-proxy</Arguments>
    </Exec>
  </Actions>
</Task>
"@

$xmlPath = "$InstallDir\lomi_service.xml"
Set-Content -Path $xmlPath -Value $xmlContent -Encoding Unicode

# 4. Register Background Service
Write-Host "   └ Registering Windows Task Scheduler Daemon..."
schtasks /create /tn $ServiceName /xml $xmlPath /f | Out-Null

Write-Host "✅ SUCCESS: Lomi for Windows has been installed!" -ForegroundColor Green
Write-Host "   The AI Gateway will now automatically start on boot."
Write-Host "   To start it immediately, you can run: schtasks /run /tn $ServiceName"
Write-Host "   Press [Win + Space] once started to access the Spotlight Overlay."
Pause
