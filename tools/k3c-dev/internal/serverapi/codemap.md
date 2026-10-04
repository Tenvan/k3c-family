# tools/k3c-dev/internal/serverapi/

## Responsibility

Schreibgeschützter HTTP-Client für die Diagnose-Schnittstelle des Go-Spielservers (`GET /api/status`, Raum-Zusammenfassungen) und Formatierung der Antworten als kompakter Text.

## Design

- `Client` (`serverapi.go`): `FromEnv(get)` (Basis-URL aus Env, z. B. mit Port-Versatz), `Status(ctx)`, `Room(ctx, code)`; DTOs `Status`, `Room`, `Summary`, `TickMs`, `Failure`.
- Fehlerhelper: `meldung`/`failf` (sprechende deutsche Fehler), `errRoomMissing`; gemeinsames `get` mit Timeout und JSON-Decode.
- `format.go`: reine Funktionen `FormatStatus`, `FormatRooms`, `FormatSummary`, `uptime`.

## Flow

1. MCP-Tool ruft `Server.serverClient(ctx)` (Worktree-Port) -> `FromEnv`.
2. `Status`/`Room` -> `get` -> JSON in DTO.
3. `Format*` erzeugt die Textantwort für den MCP-Client.

## Integration

- Konsumenten: MCP-Tools `server_status`, `rooms_list`, `room_snapshot` (`internal/mcpsrv/tools_server.go`, `worktree_services.go`).
- Gegenstelle: `engine/net` im Go-Server (`cmd/k3c-server`), HTTP auf 8080 bzw. Worktree-Versatz. Keine internen Abhängigkeiten.
