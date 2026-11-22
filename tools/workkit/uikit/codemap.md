# tools/workkit/uikit/

## Responsibility

HTTP-Schicht der Oberfläche einer Wails-App im Client-Server-Betrieb (`-headless`, `-dev`): RPC für Bindings und Server-Sent Events, damit die Oberfläche im Browser unter Vite läuft.

## Design

- `uikit.go`: `API` mit `Handler` (`POST /api/dev/rpc/{Method}` nur für erlaubte Methoden, `GET /api/dev/events`), Wächter `Guard`, `HostAllowed`, `SameSite`, `Loopback`, Kopf `X-K3C-Dev`, `MaxRPCBytes`, `Reply`/`Fail`.
- `events.go`: `Bus` (`NewBus`, `Publish`) für Server-Sent Events; `methods.go`: `TSMethods`, `CheckMethods` (Abgleich der Methoden-Liste mit `frontend/src/api/wails.ts`).
- `serve.go`: `ResolvePort`, `Listen`, `Serve`, `PageURL`.

## Flow

1. App erzeugt `API` mit ihrer Methodenliste (`uiMethods`) und dem `Bus`.
2. Frontend ruft Methoden per RPC; Live-Daten kommen per Event.

## Integration

- Konsumenten: `tools/k3c-workbench/uiapi.go`, `tools/k3c-studio/uiapi.go`.
