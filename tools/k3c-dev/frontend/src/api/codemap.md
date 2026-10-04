# tools/k3c-dev/frontend/src/api/

## Responsibility

Backend-Vertrag der Oberfläche (Port/Adapter): typisiert alle Aufrufe und Ereignisse zwischen React-Frontend und Go-App und liefert je nach Laufzeit eine echte Wails-Implementierung oder eine In-Memory-Mock-Implementierung.

## Design

- Ports: `types.ts` definiert `Backend` (Request/Response-Methoden, `on()` für Events, `openUrl()`, Flag `mock`), die DTOs (`ServiceStatus`, `Source`, `ConsoleLine`, `LogView`, `ErrorsView`, `McpOverview`, `McpUsage`, `TaskRun`, `PlanningData`, `GitHubData` …) und das Event-Mapping `Events`/`EventName` (`service:state`, `console:line`, `mcp:call`, `task:state`, `planning:changed` …). Kommentare nennen den Go-Gegentyp.
- Adapter Wails: `wails.ts` – `hasWails()` prüft `window.go.main.App` + `window.runtime`; `wailsBackend()` delegiert 1:1 an das `GoApp`-Interface und `EventsOn`. Die generierten `wailsjs/`-Dateien werden bewusst nicht importiert (Typecheck ohne Wails-CLI).
- Adapter Mock: `mock.ts` (`mockBackend()`) mit Event-Emitter und Listener-Map, zusammengesetzt aus Teil-Mocks je Seite: `mockServices.ts`, `mockLogs.ts`, `mockLogFiles.ts`, `mockMcp.ts`, `mockTasks.ts`, `mockPlanning.ts` (erfundene Daten, simulierte Events).
- Factory/Singleton: `index.ts` exportiert `backend` (Wails wenn vorhanden, sonst Mock) und re-exportiert die Typen.

## Flow

1. Beim Modulimport entscheidet `index.ts` über `hasWails()` und baut genau eine `backend`-Instanz.
2. Komponenten und Hooks rufen `backend.<methode>()` (Promise) auf; Wails leitet an die Go-Methode weiter, der Mock beantwortet aus seinem Zustand.
3. Für Live-Daten registrieren sie `backend.on(event, fn)` und erhalten eine Unsubscribe-Funktion; Go emittiert per Wails-Runtime, der Mock per `emit()`.
4. Fehler kommen als abgelehntes Promise (Wails: String, Mock: Error) und werden mit `lib/errors.ts` zu Text.

## Integration

- Konsumenten: alle Ordner unter `src/` (`logs/`, `mcp/`, `planning/`, `services/`, `tasks/`, `App.tsx`) über `import { backend } from '../api'`.
- Gegenstück: Go-App `tools/k3c-dev/app.go` (Wails-Bindings `window.go.main.App`, `window.runtime`).
- Mock-Betrieb: `npx vite` im Frontend (Port 5181) ohne Wails.
