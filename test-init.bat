@echo off
REM Test jRust Project Generator
REM This script creates a test jRust project in ./temp and verifies it works

setlocal enabledelayedexpansion

set PROJECT_NAME=test-jrust-app
set TEMP_DIR=.\temp
set PROJECT_PATH=%TEMP_DIR%\%PROJECT_NAME%

echo === jRust Test Project Generator ===
echo.

REM Clean up existing temp directory
if exist "%TEMP_DIR%" (
    echo Cleaning up existing temp directory...
    rmdir /s /q "%TEMP_DIR%"
)

REM Create temp directory
echo Creating temp directory...
mkdir "%TEMP_DIR%"

REM Build jRust CLI first
echo Building jRust CLI...
cargo build --bin jrust --quiet
if errorlevel 1 (
    echo Error: Failed to build jRust CLI
    exit /b 1
)

REM Initialize new jRust project
echo Initializing jRust project: %PROJECT_NAME%
cd "%TEMP_DIR%"
..\target\debug\jrust.exe init %PROJECT_NAME%
if errorlevel 1 (
    echo Error: Failed to initialize project
    cd ..
    exit /b 1
)
cd ..

REM Check if project was created
if not exist "%PROJECT_PATH%" (
    echo Error: Project directory not created
    exit /b 1
)

echo Project created successfully
echo.

REM Display project structure
echo Project structure:
dir /s /b "%PROJECT_PATH%"
echo.

REM Check the jRust source file
echo Source file (src\index.jr):
echo ----------------------------------------
type "%PROJECT_PATH%\src\index.jr"
echo ----------------------------------------
echo.

REM Run syntax check
echo Running syntax check...
cd "%PROJECT_PATH%"
..\..\target\debug\jrust.exe check
if errorlevel 1 (
    echo Error: Syntax check failed
    cd ..\..
    exit /b 1
)
cd ..\..
echo.

REM Build the project
echo Building jRust project...
cd "%PROJECT_PATH%"
..\..\target\debug\jrust.exe build
if errorlevel 1 (
    echo Error: Build failed
    cd ..\..
    exit /b 1
)
cd ..\..
echo.

REM Check if Rust code was generated
if exist "%PROJECT_PATH%\generated" (
    echo Generated Rust code:
    echo ----------------------------------------
    for /r "%PROJECT_PATH%\generated" %%f in (*.rs) do (
        echo File: %%f
        type "%%f"
        echo.
    )
    echo ----------------------------------------
    echo.
)

REM Run the compiled executable
echo Running the compiled program...
echo =========================================
cd "%PROJECT_PATH%"
..\..\target\debug\jrust.exe run
if errorlevel 1 (
    echo Error: Failed to run program
    cd ..\..
    exit /b 1
)
cd ..\..
echo =========================================
echo.

echo All tests passed!
echo.
echo Test project location: %PROJECT_PATH%
echo To manually test: cd %PROJECT_PATH% ^&^& ..\..\target\debug\jrust.exe run

endlocal
