#Requires -Version 5.1

param(
  [string]$InstallDir = "",
  [bool]$AddToPath = $true,
  [string]$Version = "",
  [switch]$Help
)

$GitHubRepo = "christianhelle/httpgenerator"
$BinaryName = "httpgenerator.exe"
$DocumentationUrl = "https://christianhelle.com/httpgenerator/"

function Write-Info
{
  param([string]$Message)
  Write-Host "[INFO] $Message" -ForegroundColor Cyan
}

function Write-Success
{
  param([string]$Message)
  Write-Host "[OK] $Message" -ForegroundColor Green
}

function Write-WarningMessage
{
  param([string]$Message)
  Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-ErrorMessage
{
  param([string]$Message)
  Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Show-Usage
{
  Write-Host ""
  Write-Host "HTTP File Generator installation script for Windows" -ForegroundColor Blue
  Write-Host ""
  Write-Host "Usage:" -ForegroundColor Yellow
  Write-Host "  irm https://christianhelle.com/httpgenerator/install.ps1 | iex" -ForegroundColor White
  Write-Host ""
  Write-Host "Parameters:" -ForegroundColor Yellow
  Write-Host "  -InstallDir <path>   Installation directory" -ForegroundColor White
  Write-Host "  -AddToPath <bool>    Add the install directory to the user PATH (default: true)" -ForegroundColor White
  Write-Host "  -Version <tag>       Install a specific GitHub Release tag" -ForegroundColor White
  Write-Host "  -Help                Show this help" -ForegroundColor White
  Write-Host ""
}

function Get-DefaultInstallDir
{
  $candidates = @(
    "$env:LOCALAPPDATA\Programs\httpgenerator",
    "$env:USERPROFILE\.local\bin",
    "$env:USERPROFILE\bin"
  )

  foreach ($candidate in $candidates)
  {
    if (Test-Path $candidate -PathType Container)
    {
      return $candidate
    }
  }

  return "$env:LOCALAPPDATA\Programs\httpgenerator"
}

function Test-IsInPath
{
  param([string]$Directory)

  $pathDirs = $env:PATH -split ';' | ForEach-Object { $_.Trim('"').TrimEnd('\') }
  $targetDir = $Directory.TrimEnd('\')

  return $pathDirs -contains $targetDir
}

function Add-ToUserPath
{
  param([string]$Directory)

  if (Test-IsInPath -Directory $Directory)
  {
    Write-Info "Directory already in PATH: $Directory"
    return
  }

  $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
  if ([string]::IsNullOrWhiteSpace($userPath))
  {
    $userPath = $Directory
  }
  elseif ($userPath.EndsWith(';'))
  {
    $userPath += $Directory
  }
  else
  {
    $userPath += ";$Directory"
  }

  [Environment]::SetEnvironmentVariable("PATH", $userPath, "User")
  $env:PATH += ";$Directory"

  Write-Success "Added to user PATH: $Directory"
  Write-WarningMessage "Restart your terminal for PATH changes to take effect"
}

function Get-LatestRelease
{
  Write-Info "Fetching latest release information..."

  $apiUrl = "https://api.github.com/repos/$GitHubRepo/releases/latest"
  $response = Invoke-RestMethod -Uri $apiUrl -ErrorAction Stop
  return $response.tag_name
}

function Get-ArchiveName
{
  param([string]$ResolvedVersion)

  $architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture

  switch ($architecture)
  {
    ([System.Runtime.InteropServices.Architecture]::X64)
    {
      return "httpgenerator-$ResolvedVersion-win-x64.zip"
    }
    ([System.Runtime.InteropServices.Architecture]::Arm64)
    {
      Write-WarningMessage "Windows ARM64 releases are not published yet. Falling back to the x64 archive."
      return "httpgenerator-$ResolvedVersion-win-x64.zip"
    }
    default
    {
      throw "Unsupported Windows architecture: $architecture"
    }
  }
}

function Install-HttpGenerator
{
  param(
    [string]$ResolvedVersion,
    [string]$TargetDir
  )

  $archiveName = Get-ArchiveName -ResolvedVersion $ResolvedVersion
  $downloadUrl = "https://github.com/$GitHubRepo/releases/download/$ResolvedVersion/$archiveName"
  $tempDir = Join-Path $env:TEMP "httpgenerator-install-$(Get-Random)"
  $archivePath = Join-Path $tempDir $archiveName

  try
  {
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

    Write-Info "Downloading $archiveName..."
    Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -ErrorAction Stop

    Write-Info "Extracting archive..."
    Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force

    if (-not (Test-Path $TargetDir))
    {
      Write-Info "Creating installation directory: $TargetDir"
      New-Item -ItemType Directory -Path $TargetDir -Force | Out-Null
    }

    $sourceBinary = Get-ChildItem -Path $tempDir -Filter $BinaryName -File -Recurse | Select-Object -First 1
    if ($null -eq $sourceBinary)
    {
      throw "The downloaded archive did not contain $BinaryName."
    }

    $targetBinary = Join-Path $TargetDir $BinaryName

    Write-Info "Installing to $TargetDir..."
    Copy-Item -Path $sourceBinary.FullName -Destination $targetBinary -Force

    return $targetBinary
  } finally
  {
    if (Test-Path $tempDir)
    {
      Remove-Item -Path $tempDir -Recurse -Force
    }
  }
}

function Test-Installation
{
  param([string]$BinaryPath)

  if (-not (Test-Path $BinaryPath))
  {
    Write-WarningMessage "Could not verify installation because $BinaryPath was not found."
    return
  }

  $versionOutput = & $BinaryPath --version 2>$null
  if ($LASTEXITCODE -eq 0)
  {
    Write-Success "Installation verified: $versionOutput"
  }
  else
  {
    Write-WarningMessage "The installed binary did not return a version string."
  }
}

function Main
{
  if ($Help)
  {
    Show-Usage
    return
  }

  if ([string]::IsNullOrWhiteSpace($InstallDir))
  {
    $InstallDir = Get-DefaultInstallDir
  }

  Write-Info "Starting HTTP File Generator installation for Windows..."
  Write-Info "Target directory: $InstallDir"

  $resolvedVersion = if ([string]::IsNullOrWhiteSpace($Version)) { Get-LatestRelease } else { $Version }
  Write-Info "Installing release: $resolvedVersion"

  $binaryPath = Install-HttpGenerator -ResolvedVersion $resolvedVersion -TargetDir $InstallDir

  if ($AddToPath)
  {
    Add-ToUserPath -Directory $InstallDir
  }

  Test-Installation -BinaryPath $binaryPath

  Write-Success "Installation complete."
  Write-Info "Documentation: $DocumentationUrl"
}

Main
