#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Script para ejecutar el convertidor de formatos Dot ignore.
.DESCRIPTION
    Este script ejecuta el binario de vcsconvert para convertir entre diferentes formatos de archivos de ignore.
.PARAMETER InputFile
    Ruta al archivo de entrada a convertir.
.PARAMETER OutputFile
    Ruta donde se guardará el archivo convertido.
.PARAMETER Format
    Formato de salida (git, svn, vcs).
.EXAMPLE
    ./convert.ps1 -InputFile .gitignore -OutputFile .svnignore -Format svn
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$InputFile,
    
    [Parameter(Mandatory=$true)]
    [string]$OutputFile,
    
    [Parameter(Mandatory=$true)]
    [ValidateSet("git", "svn", "vcs")]
    [string]$Format
)

# Determinar la ruta del script y la ubicación del binario
$ScriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootPath = Split-Path -Parent $ScriptPath
$BinaryPath = Join-Path -Path $RootPath -ChildPath "bin\windows\vcsconvert.exe"

# Verificar si el binario existe
if (-not (Test-Path $BinaryPath)) {
    Write-Error "El binario '$BinaryPath' no existe. Por favor, compile el proyecto primero con 'cargo build --release'."
    exit 1
}

# Ejecutar el comando de conversión
Write-Host "Convirtiendo $InputFile a formato $Format..."
& $BinaryPath --input $InputFile --output $OutputFile --format $Format

if ($LASTEXITCODE -eq 0) {
    Write-Host "Conversión completada exitosamente. Archivo guardado en $OutputFile"
} else {
    Write-Error "Error durante la conversión. Código de salida: $LASTEXITCODE"
    exit $LASTEXITCODE
}
