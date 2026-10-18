# tools/k3c-starter/

## Responsibility

Windows-Starter für den K3C-Server (B-041, BT1.2): ein Wails-Fenster, das `bin/k3c-server` als Kindprozess startet und Status, Adressen im Heimnetz und die Meldungen des Servers zeigt. Eigenes Go-Modul ohne Import des Hauptmoduls; die Server-Logik bleibt in `cmd/k3c-server`.

## Design

- `main.go`: Wails-Start (Fenster 640×560, Single-Instance-Lock), eingebettete Oberfläche `web/`; `findLayout` ermittelt Wurzel, Server-EXE und `dist/`; Fenster schließen beendet den Server.
- `app.go`: `App` (Bindung ans Fenster: `State`, `Start`, `Stop`); ohne gefundenes Layout öffnet das Fenster trotzdem und nennt den Grund.
- `server.go`: `supervisor` (Port-Prüfung auf Loopback, Prozess starten mit `K3C_HTTP_PORT`, `K3C_DIST`, `K3C_DEV=0`, `/api/health` abwarten, unerwartetes Ende melden, `Stop` = Prozess beenden), `Status`, `lanURLs`, `lineWriter` (Log-Zeilen).
- `proc_windows.go`/`proc_other.go`: Konsolenfenster des Servers verbergen.
- `web/index.html`: statische Seite (kein Node-Build), fragt `window.go.main.App.State()` jede Sekunde ab.

## Flow

1. Start der EXE → `findLayout` → `App.start` → `supervisor.Start` (Port frei? EXE starten) → `awaitHealth` → „Server läuft.“ mit Adressen.
2. Schließen des Fensters → `App.stop` → Server beendet, Meldung „Server ist gestoppt.“.

## Integration

- Tasks `k3c-starter:build` (baut `bin/k3c-server`, `dist/` und `bin/k3c-starter`) und `check:starter`; CI-Job `k3c-starter` (Windows).
- Tests: `server_test.go` (Testprogramm als falscher Server auf Loopback, Port belegt, unerwartetes Ende, Layout).
