# Icon des Studios

Vorläufig eine Kopie des Workbench-Icons (Kronen-Emoji 👑, `tools/k3c-workbench/icon/`, WZ63.3): `appicon.png` (256 px)
und `icon.ico` (16, 32, 48, 256 px). `task k3c-studio:build` kopiert beide nach `build/` (dort liegt, was `wails build`
für die EXE verwendet); `main.go` bettet `appicon.png` für das Fenster-Icon in Dev-Läufen ein (tools/workkit/winicon).
