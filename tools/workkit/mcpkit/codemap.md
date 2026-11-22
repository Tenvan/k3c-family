# tools/workkit/mcpkit/

## Responsibility

Gemeinsames Gerüst der MCP-Server von k3c-workbench und k3c-studio (Streamable HTTP, nur 127.0.0.1): Server-Lebenszyklus, Aufrufstatistik, Worktree-Auflösung, Tool-Helper. Kennt keine konkreten Tools.

## Design

- `server.go`: `Options`, `Server` (`New`, `Start`, `Stop`, `Restart`, `URL`, `MCP`, `Root`, `Stats`), `ResolvePort(env, raw, default)`.
- `tool.go`: generische Helper `Add[In]`, `AddImage`, `ReadOnly()` (Annotations, Fehler-/Textergebnis, Aufrufzählung).
- `stats.go`: `Stats` (`Register`, `Begin`, `End`, `ObserveClients`, `Snapshot`), `Call`, `ToolStats`; `text.go`: Kürzen und Ergebnistexte (`Clip`, `ClipArgs`, `OutcomeOf`, `TextResult`, `OrDash`).
- `workspace.go`: `Workspace`, `Source`, `WithWorkspace`, Header `X-K3C-Root`; `checkout.go`: Argument `checkout` (`CheckoutArg`, `Worktrees`); `paths.go`: `InsideRoot`.
- `progress.go` (Fortschrittsmeldungen), `chunked_writer.go` (`ForceChunked`), `observe.go` (`PanicText`), `param_hint.go` (Hinweis auf gültige Parameter).

## Flow

1. App ruft `New(Options)` mit Name, Instructions und Registrierfunktion; `Start` lauscht auf dem Port (Env oder Vorgabe).
2. Jeder Tool-Aufruf läuft über `Add`: Workspace aus Header oder `checkout`, Statistik `Begin`/`End`, Panik-Schutz.

## Integration

- Konsumenten: `tools/k3c-workbench/internal/mcpsrv` (`kit.go`), `tools/k3c-studio/internal/mcpsrv` (`kit.go`).
- Abhängigkeiten: `github.com/modelcontextprotocol/go-sdk/mcp`, `jsonschema-go`.
