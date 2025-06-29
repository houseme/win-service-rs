# Deploy rustfs-manager to Windows
$ErrorActionPreference = "Stop"

# Configuration
$InstallDir = "C:\Program Files\RustFSManager"
$ConfigDir = "C:\ProgramData\RustFSManager"
$ServiceName = "RustFSManager"

# Create directories
New-Item -Path $InstallDir -ItemType Directory -Force | Out-Null
New-Item -Path $ConfigDir -ItemType Directory -Force | Out-Null

# Copy binaries and config
Copy-Item -Path ".\target\release\rustfs-manager.exe" -Destination "$InstallDir\" -Force
Copy-Item -Path ".\config\config.toml" -Destination "$ConfigDir\" -Force

# Install as service
sc.exe create $ServiceName binPath= "$InstallDir\rustfs-manager.exe status" DisplayName= "RustFS Manager" start= auto
sc.exe description $ServiceName "Manages RustFS services with win-service-rs"
sc start $ServiceName

Write-Host "Deployment completed successfully!"