@echo off
python scripts/static-pie.py --target x86_64-pc-windows-msvc --lang C --profile Debug --cargo_args %* || goto :error

:; exit 0
endlocal
exit /b 0

:error
endlocal
exit /b %errorlevel%