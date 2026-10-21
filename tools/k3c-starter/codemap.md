# tools/k3c-starter/

## Responsibility

Windows-Starter für den K3C-Server (B-041, BT1.2): ein Wails-Fenster, das `bin/k3c-server` als Kindprozess startet und Status, Adressen im Heimnetz und die Meldungen des Servers zeigt. Eigenes Go-Modul ohne Import des Hauptmoduls; die Server-Logik bleibt in `cmd/k3c-server`.

## Design

- `main.go`: Wails-Start (Fenster 640×560, Single-Instance-Lock), eingebettete Oberfläche `frontend/dist` (`go:embed`, Build mit `task k3c-starter:frontend`); Flags `-headless` und `-dev` (siehe unten); Fenster schließen beendet den Server.
- `server.go` › `findLayout` ermittelt Wurzel, Server-EXE und `dist/` (Arbeitsordner, EXE-Ordner, darüber; `K3C_ROOT` nennt die Wurzel ausdrücklich und gilt zuerst).
- `app.go`: `App` (Bindung ans Fenster: `State`, `Start`, `Stop`); ohne gefundenes Layout öffnet das Fenster trotzdem und nennt den Grund.
- `server.go`: `supervisor` (Port-Prüfung auf Loopback, Prozess starten mit `K3C_HTTP_PORT`, `K3C_DIST`, `K3C_DEV=0`, `/api/health` abwarten, unerwartetes Ende melden, `Stop` = Prozess beenden), `Status`, `lanURLs`, `lineWriter` (Log-Zeilen).
- `proc_windows.go`/`proc_other.go`: Konsolenfenster des Servers verbergen.
- `api.go`: Starter-API für den Entwicklungsmodus, dieselben Aufrufe wie die Wails-Bindungen: `GET /api/starter/state`, `POST /api/starter/start`, `POST /api/starter/stop` (JSON `Status`). Nur Loopback-Gegenstelle, Host `localhost`/Loopback-IP (gegen DNS-Rebinding), gleiche Herkunft (`Origin` passt zum Host, `Sec-Fetch-Site` nur same-origin/none, auch bei GET) und Kopfzeile `X-K3C-Starter: 1`, keine CORS-Freigabe (Vite: `server.cors: false`).
- `headless.go`: `-headless` = Starter ohne Fenster: startet den Server wie `OnStartup`, bedient die API auf `127.0.0.1:K3C_STARTER_API_PORT` (Standard 8194, `devrun.Listen` wartet nach einem Neustart auf den Port), beendet den Server bei Strg+C/SIGTERM wie `OnBeforeClose`.
- `dev.go`: `-dev` (aus der Repo-Wurzel) = `engine/devrun`: baut `tools/k3c-starter` bei Go-Änderungen nach `build/bin/k3c-starter-devchild-<API-Port>.exe` (je Lauf eigene EXE) und führt es mit `-headless` aus (`K3C_ROOT` = Wurzel). Das Go-Modul bindet das Hauptmodul dafür per `replace k3c => ../..` ein, nur `engine/devrun`.
- `frontend/`: React-Oberfläche auf Radix Themes mit den Atomic-Design-Bausteinen aus `tools/webkit` (Alias `@webkit`, wie `tools/k3c-dev/frontend`); eigenes `package.json`, Vite (Port 5194, Proxy `/api` → `K3C_STARTER_API`, Standard `http://127.0.0.1:8194`), Vitest, `tsconfig.json`.
  - `src/App.tsx`: `AppFrame` (Titel „K3C Server“, dunkel, ohne Reiter, Mock-Badge rechts).
  - `src/api/`: `types.ts` (`ServerState` wie `Status` in `server.go`, `Backend`), `wails.ts` (`window.go.main.App.State/Start/Stop`), `mock.ts` (simuliert Start, Lauf, Beenden), `http.ts` (HTTP-Backend gegen `/api/starter/`, Kopfzeile `X-K3C-Starter`), `index.ts` (wählt Wails im Fenster, Mock bei `?mock=1` und in Tests, sonst HTTP).
  - `src/server/`: die Seite (`ServerPage`: Status als `StatusBadge`, Knöpfe mit Sperrlogik), `AddressList` (Kopieren, 1,5 s Rückmeldung), `LogBox` (rutscht nur mit, wenn sie am Ende stand), `useServerState` (Abfrage jede Sekunde).
  - `src/styles/server.css`: nur Layout (Status-Umbruch, Log-Box).

## Flow

1. Start der EXE → `findLayout` → `App.start` → `supervisor.Start` (Port frei? EXE starten) → `awaitHealth` → „Server läuft.“ mit Adressen.
2. Die Oberfläche fragt `App.State()` jede Sekunde ab und zeigt Status, Adressen und Log; Starten/Beenden rufen `App.Start`/`App.Stop`.
3. Schließen des Fensters → `App.stop` → Server beendet, Meldung „Server ist gestoppt.“.

## Entwicklung (Client/Server)

`task k3c-starter:dev` = `dev:server` (Starter mit `-dev` unter devrun, Neubau bei Go-Änderungen, `-headless` mit API auf 8194) + `dev:ui` (Vite 5194 mit Hot Reload, Proxy `/api`). Oberfläche im Browser: `http://localhost:5194/`. `task k3c-starter:window` ist das echte Wails-Fenster (`wails dev`, derselbe Vite-Port, nicht zusammen mit `dev:ui`). Kein `k3c-watch` und kein Fenster-Neubau mehr.

## Integration

- Tasks `k3c-starter:frontend` (Oberfläche nach `frontend/dist`), `k3c-starter:build` (baut `bin/k3c-server`, `dist/` und `bin/k3c-starter`) und `check:starter`; CI-Job `k3c-starter` (Windows).
- Tests: `server_test.go` (Testprogramm als falscher Server auf Loopback, Port belegt, unerwartetes Ende, Layout); Frontend `frontend/src/**/*.test.{ts,tsx}` mit `frontend/vitest.config.ts`.
