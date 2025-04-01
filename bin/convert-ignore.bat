@echo off
rem Script para convertir archivos .gitignore y .svnignore a formato .DotIgnore
rem Parte del proyecto DotIgnore

echo ===============================================
echo        Dot ignore Format Converter Tool       
echo ===============================================

rem Verificar argumentos
if "%~1"=="" (
    echo Error: Debe especificar un archivo de origen.
    echo Uso: convert-ignore [archivo.gitignore|archivo.svnignore] [archivo.DotIgnore]
    echo.
    echo Ejemplos:
    echo   convert-ignore .gitignore
    echo   convert-ignore proyecto/.svnignore proyecto/.DotIgnore
    exit /b 1
)

rem Determinar argumentos
set SOURCE=%~1
set DEST=%~2

rem Si no se especifica destino, usar .DotIgnore en la misma ubicación
if "%DEST%"=="" (
    for %%I in ("%SOURCE%") do set DEST=%%~dpI.DotIgnore
)

rem Verificar que el archivo de origen existe
if not exist "%SOURCE%" (
    echo Error: El archivo %SOURCE% no existe.
    exit /b 1
)

rem Ejecutar la conversión
echo.
echo Convirtiendo %SOURCE% a formato .DotIgnore...
vcs convert-ignore -s "%SOURCE%" -d "%DEST%"

rem Verificar si la conversión fue exitosa
if %ERRORLEVEL% neq 0 (
    echo Error: No se pudo convertir el archivo.
    exit /b 1
)

echo.
echo Conversión completada con éxito.
echo Archivo generado: %DEST%
echo. 
