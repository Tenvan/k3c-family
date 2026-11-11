@echo off
rem K3C-Prototypen (Bevy und Ebitengine) auf dem Mini-PC messen: Doppelklick. Bericht landet in reports\ neben dieser Datei.
rem Nur eine Engine: messen.cmd -Engine bevy  oder  messen.cmd -Engine ebiten
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0measure.ps1" %*
pause
