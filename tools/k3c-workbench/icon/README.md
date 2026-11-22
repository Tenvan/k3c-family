# Icon der Workbench
Zahnrad mit goldener Krone (`appicon.png` 256 px, `icon.ico` 16, 32, 48, 256 px), erzeugt von
`tools/icons/gen_icons.py` (Pillow; `python tools/icons/gen_icons.py` von der Repo-Wurzel, erzeugt auch die Icons von
Studio und Spiel). `task k3c-workbench:build` kopiert beide Dateien nach `build/`; `main.go` bettet `appicon.png` für das
Fenster-Icon in Dev-Läufen ein.
