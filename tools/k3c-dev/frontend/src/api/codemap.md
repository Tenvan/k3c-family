# tools/k3c-dev/frontend/src/api/

## Responsibility

Backend-Vertrag der Oberfläche (Port/Adapter): typisiert alle Aufrufe und Ereignisse zwischen React-Frontend und Go-App und liefert je nach Laufzeit die Wails-Implementierung (Fenster), die HTTP-Implementierung (`k3c-dev -headless`/`-dev`, Browser) oder eine In-Memory-Mock-Implementierung.

## Design

- Ports: `types.ts` definiert `Backend` (Request/Response-Methoden, `on()` für Events, `openUrl()`, Flag `mock`), die DTOs (`ServiceStatus`, `Source`, `ConsoleLine`, `LogView`, `ErrorsView`, `McpOverview`, `McpUsage`, `TaskRun`, `TaskGates`/`TaskGateState`, `Notice`, `PlanningData` mit `PlanProject`/`PlanSprint`/`PlanSession`/`PlanTicket`, `WorktreeDiff`, `GitHubData` …) und das Event-Mapping `Events`/`EventName` (`service:state`, `console:line`, `mcp:call`, `task:state`, `ui:notify`, `planning:changed` …). Kommentare nennen den Go-Gegentyp.
- Adapter Wails: `wails.ts` – `hasWails()` prüft `window.go.main.App` + `window.runtime`; `wailsBackend()` delegiert 1:1 an das `GoApp`-Interface (u. a. `ServicesReload`, `TaskGatesView`/`TaskGateToggle`/`TaskGateSave`, `PlanningSet`, `PlanningGet`, `WorktreeCheck`, `GitHubStatus`, `OpenInClaude`, `StudioOpen`), `openUrl` über `runtime.BrowserOpenURL`, Ereignisse über `EventsOn`. Die generierten `wailsjs/`-Dateien werden bewusst nicht importiert (Typecheck ohne Wails-CLI).
- Adapter HTTP: `http.ts` – `httpBackend()` ruft jede Methode als `POST /api/dev/rpc/<GoApp-Name>` mit den Argumenten als JSON-Array und Kopfzeile `X-K3C-Dev: 1` (Antwort `{result}`/`{error}`, Fehler als `Error`), Ereignisse über eine gemeinsame `EventSource` auf `/api/dev/events` (Nutzlast JSON), `openUrl` per `window.open`. Gegenstück `tools/k3c-dev/uiapi.go`; Test `http.test.ts` mit fetch-/EventSource-Attrappen.
- Adapter Mock: `mock.ts` (`mockBackend()`) mit Event-Emitter und Listener-Map, zusammengesetzt aus Teil-Mocks je Seite: `mockServices.ts` (Dienste, `servicesReload`, simulierte `service:state`-Ereignisse), `mockLogs.ts`, `mockLogFiles.ts`, `mockMcp.ts`, `mockTasks.ts` (Katalog, Läufe und `mockGates()` für die Freigabe-Schlösser), `mockPlanning.ts` (`mockPlanning(changed)`: Sprints, Tickets, GitHub-Stand; `planningSet` ändert Kopf-Felder im Speicher und löst `planning:changed` aus; `mockDoc` wie `planning.Get`) und `mockProjects.ts` (`withProjects` hängt Projekte, den ABN-Sprint und Projekt-Felder an die Mock-Planung, `setRank` verschiebt den Rang wie `planning.Set`). `mock.ts` löst zusätzlich eine `ui:notify`-Probe aus; `studioOpen` wirft dort den Hinweis „Asset-Studio nicht gebaut“. Alle Daten sind erfunden.
- Factory/Singleton: `index.ts` exportiert `backend` (Wails wenn vorhanden, im Test oder mit `?mock=1` der Mock, sonst HTTP) und re-exportiert die Typen.

## Flow

1. Beim Modulimport entscheidet `index.ts` über `hasWails()` und `wantsMock()` (`?mock=1`, Vitest) und baut genau eine `backend`-Instanz.
2. Komponenten und Hooks rufen `backend.<methode>()` (Promise) auf; Wails leitet an die Go-Methode weiter, der Mock beantwortet aus seinem Zustand.
3. Für Live-Daten registrieren sie `backend.on(event, fn)` und erhalten eine Unsubscribe-Funktion; Go emittiert per Wails-Runtime bzw. Server-Sent Events (`emitAll`), der Mock per `emit()`.
4. Fehler kommen als abgelehntes Promise (Wails: String, Mock: Error) und werden mit `@webkit/errors` (`tools/webkit/errors.ts`) zu Text.

## Integration

- Konsumenten: alle Ordner unter `src/` (`logs/`, `mcp/`, `planning/`, `services/`, `tasks/`, `ui/organisms/Notices.tsx`, `App.tsx`, `StudioLink.tsx`) über `import { backend } from '../api'`.
- Gegenstück: Go-App `tools/k3c-dev/app.go` und die `app_*.go`-Dateien (Wails-Bindings `window.go.main.App`, `window.runtime`).
- Client-Server-Betrieb: `task k3c-dev:dev` (Vite auf 5191, Proxy `/api` auf die Oberflächen-API von `k3c-dev -dev`, `K3C_DEV_API`). Mock-Betrieb: `http://localhost:5191/?mock=1`.
