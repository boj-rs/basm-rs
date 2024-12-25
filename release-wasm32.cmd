@echo off
python scripts/static-pie.py --target wasm32-unknown-unknown --lang JavaScript --profile Release --cargo_args %* || goto :error

:; exit 0
endlocal
exit /b 0

:error
endlocal
exit /b %errorlevel%