@echo off
rem K3C Bevy-Prototyp auf dem Mini-PC messen: Doppelklick. Bericht landet in reports\ neben dieser Datei.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0measure.ps1"
pause
