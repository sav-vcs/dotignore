@echo off
REM Script para ejecutar el convertidor de formatos Dot ignore

setlocal enabledelayedexpansion

REM Verificar si se proporcionaron los argumentos necesarios
if "%~1"=="" (
    echo Error: Se requiere especificar un archivo de entrada.
    echo Uso: run-conversion.bat archivo_entrada archivo_salida formato
    exit /b 1
)

if "%~2"=="" (
    echo Error: Se requiere especificar un archivo de salida.
    echo Uso: run-conversion.bat archivo_entrada archivo_salida formato
    exit /b 1
)

if "%~3"=="" (
    echo Error: Se requiere especificar un formato de salida (git, svn, vcs).
    echo Uso: run-conversion.bat archivo_entrada archivo_salida formato
    exit /b 1
)

set INPUT_FILE=%~1
set OUTPUT_FILE=%~2
set FORMAT=%~3

REM Determinar la ruta del script y la ubicación del binario
set SCRIPT_DIR=%~dp0
set ROOT_DIR=%SCRIPT_DIR%..
set BINARY_PATH=%ROOT_DIR%\bin\windows\vcsconvert.exe

REM Verificar si el binario existe
if not exist "%BINARY_PATH%" (
    echo Error: El binario '%BINARY_PATH%' no existe. Por favor, compile el proyecto primero con 'cargo build --release'.
    exit /b 1
)

REM Ejecutar el comando de conversión
echo Convirtiendo %INPUT_FILE% a formato %FORMAT%...
"%BINARY_PATH%" --input "%INPUT_FILE%" --output "%OUTPUT_FILE%" --format "%FORMAT%"

if %ERRORLEVEL% equ 0 (
    echo Conversión completada exitosamente. Archivo guardado en %OUTPUT_FILE%
) else (
    echo Error durante la conversión. Código de salida: %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

endlocal
