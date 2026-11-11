@echo off
rem K3C-Prototypen (Ebitengine, Rig) auf dem Mini-PC messen: Doppelklick. Bericht landet in reports\ neben dieser Datei.
rem Nur eine Engine: messen.cmd -Engine ebiten  oder  messen.cmd -Engine rig (Rig-Prototyp, B-778)
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0measure.ps1" %*
pause
