@echo off
setlocal

set PROJECT_NAME=test-jrust-app
set TEMP_DIR=.\temp
set PROJECT_PATH=%TEMP_DIR%\%PROJECT_NAME%

echo === jRust Test Project Generator ===
echo.

if exist "%PROJECT_PATH%" (
    echo Cleaning up existing test project...
    rmdir /s /q "%PROJECT_PATH%"
)

if not exist "%TEMP_DIR%" (
    echo Creating temp directory...
    mkdir "%TEMP_DIR%"
)

echo Building jRust CLI...
cargo build --release --quiet

echo Initializing test project: %PROJECT_NAME%
cd "%TEMP_DIR%"
..\target\release\jrust.exe init %PROJECT_NAME%

echo.
echo Generated project structure:
cd "%PROJECT_NAME%"
dir /s /b *.jr jrust.toml 2>nul

echo.
echo Checking jRust code...
..\..\target\release\jrust.exe check

echo.
echo Building project...
..\..\target\release\jrust.exe build

echo.
echo Running project...
..\..\target\release\jrust.exe run

echo.
echo Test project generation completed successfully!
echo Project location: %PROJECT_PATH%

endlocal
