# tools/assetkit/windowstate/

## Responsibility
Paket `windowstate`: merkt Größe und Position eines Wails-Fensters zwischen zwei Starts in einer Datei unter dem Benutzer-Konfigurationsordner. Es kennt Wails nicht; Aufrufer reichen Lesen und Setzen der Position als Funktionen herein.

## Design
- `windowstate.go`:
  - Konstanten `DefaultWidth`/`DefaultHeight` (1280 × 800) und `MinWidth`/`MinHeight` (900 × 600).
  - Typ `State` (`x`, `y`, `width`, `height` als JSON).
  - `Load(path)`: fehlende oder kaputte Datei → Vorgabe und `ok=false` (Position bleibt Wails überlassen); zu kleine Maße werden aufs Minimum gehoben, negative Positionen bleiben (Bildschirm links/oberhalb).
  - `Place(get, set, x, y)`: setzt die absolute Position; der Zug auf (0, 0) misst den Versatz des Arbeitsbereichs, weil Wails relativ zum aktuellen Bildschirm setzt.
  - `Save(path, state)`: legt den Ordner an und schreibt JSON.
  - `ConfigPath(name)`: `<UserConfigDir>/k3c/<name>`, ohne Konfigurationsordner relativ `.work/config/<name>` (kein Temp-Ordner außerhalb des Repos).
- `screen_windows.go` (`//go:build windows`): `OnScreen(state)` fragt `user32.MonitorFromRect` für einen Punkt der Titelleiste (100 px vom linken Rand) ab; fehlt der Bildschirm, ist das Fenster nicht greifbar.
- `screen_other.go` (`//go:build !windows`): `OnScreen` liefert immer true.

## Flow
1. Start: `ConfigPath(name)` → `Load` → bei `ok && OnScreen` Fenster mit `Place` positionieren, sonst Wails-Vorgabe.
2. Beim Schließen: Position und Größe lesen → `Save(ConfigPath(name), State{…})`.

## Integration
- Konsument: `tools/k3c-workbench` (`main.go` › `windowOptions`: `Load`, `MinWidth`/`MinHeight`, `OnScreen` und `Place` in `OnDomReady`; `app.go`: `Save`; `headless.go`: `ConfigPath`).
- Nur Standardbibliothek (`syscall` unter Windows).
- Test: `windowstate_test.go`.
