# tools/k3c-starter/

## Responsibility

Windows-Starter für den K3C-Server (B-041, BT1.2): ein Wails-Fenster, das `bin/k3c-server` als Kindprozess startet und Status, Adressen im Heimnetz und die Meldungen des Servers zeigt. Eigenes Go-Modul ohne Import des Hauptmoduls; die Server-Logik bleibt in `cmd/k3c-server`.

## Design

- `main.go`: Wails-Start (Fenster 640×560, Single-Instance-Lock), eingebettete Oberfläche `frontend/dist` (`go:embed`, Build mit `task k3c-starter:frontend`); `findLayout` ermittelt Wurzel, Server-EXE und `dist/`; Fenster schließen beendet den Server.
- `app.go`: `App` (Bindung ans Fenster: `State`, `Start`, `Stop`); ohne gefundenes Layout öffnet das Fenster trotzdem und nennt den Grund.
- `server.go`: `supervisor` (Port-Prüfung auf Loopback, Prozess starten mit `K3C_HTTP_PORT`, `K3C_DIST`, `K3C_DEV=0`, `/api/health` abwarten, unerwartetes Ende melden, `Stop` = Prozess beenden), `Status`, `lanURLs`, `lineWriter` (Log-Zeilen).
- `proc_windows.go`/`proc_other.go`: Konsolenfenster des Servers verbergen.
- `frontend/`: React-Oberfläche auf Radix Themes mit den Atomic-Design-Bausteinen aus `tools/webkit` (Alias `@webkit`, wie `tools/k3c-dev/frontend`); eigenes `package.json`, Vite (Port 5194), Vitest, `tsconfig.json`.
  - `src/App.tsx`: `AppFrame` (Titel „K3C Server“, dunkel, ohne Reiter, Mock-Badge rechts).
  - `src/api/`: `types.ts` (`ServerState` wie `Status` in `server.go`, `Backend`), `wails.ts` (`window.go.main.App.State/Start/Stop`), `mock.ts` (simuliert Start, Lauf, Beenden), `index.ts` (wählt Wails oder Mock).
  - `src/server/`: die Seite (`ServerPage`: Status als `StatusBadge`, Knöpfe mit Sperrlogik), `AddressList` (Kopieren, 1,5 s Rückmeldung), `LogBox` (rutscht nur mit, wenn sie am Ende stand), `useServerState` (Abfrage jede Sekunde).
  - `src/styles/server.css`: nur Layout (Status-Umbruch, Log-Box).

## Flow

1. Start der EXE → `findLayout` → `App.start` → `supervisor.Start` (Port frei? EXE starten) → `awaitHealth` → „Server läuft.“ mit Adressen.
2. Die Oberfläche fragt `App.State()` jede Sekunde ab und zeigt Status, Adressen und Log; Starten/Beenden rufen `App.Start`/`App.Stop`.
3. Schließen des Fensters → `App.stop` → Server beendet, Meldung „Server ist gestoppt.“.

## Integration

- Tasks `k3c-starter:frontend` (Oberfläche nach `frontend/dist`), `k3c-starter:build` (baut `bin/k3c-server`, `dist/` und `bin/k3c-starter`) und `check:starter`; CI-Job `k3c-starter` (Windows).
- Tests: `server_test.go` (Testprogramm als falscher Server auf Loopback, Port belegt, unerwartetes Ende, Layout); Frontend `frontend/src/**/*.test.{ts,tsx}` mit `frontend/vitest.config.ts`.
