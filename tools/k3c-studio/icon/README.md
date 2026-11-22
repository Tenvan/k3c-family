# Icon des Studios
Papierschnitt-Insel mit Baum, Krone und Pinsel (`appicon.png` 256 px, `icon.ico` 16, 32, 48, 256 px), erzeugt von
`tools/icons/gen_icons.py` (Pillow; `python tools/icons/gen_icons.py` von der Repo-Wurzel, erzeugt auch die Icons von
Workbench und Spiel). `task k3c-studio:build` kopiert beide Dateien nach `build/` (dort liegt, was `wails build` für die
EXE verwendet); `main.go` bettet `appicon.png` für das Fenster-Icon in Dev-Läufen ein (tools/workkit/winicon).
