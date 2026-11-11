@echo off
rem K3C auf dem Mini-PC messen: Doppelklick misst alle EXE neben dieser Datei. Bericht landet in reports\ neben dieser Datei.
rem Nur eine Engine: messen.cmd -Engine game (natives Spiel, B-802), -Engine rig (Rig-Prototyp, B-778) oder -Engine ebiten
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0measure.ps1" %*
pause
