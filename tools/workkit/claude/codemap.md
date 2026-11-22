# tools/workkit/claude/

## Responsibility

„In Claude öffnen“: startet eine neue Code-Session in Claude Desktop mit vorbefülltem Prompt über den Deep Link `claude://`.

## Design

- `claude.go`: `Link(prompt)` baut den Link, `Open(prompt)` öffnet ihn; `MaxPrompt` begrenzt die Länge.

## Flow

1. Oberfläche ruft `OpenInClaude(prompt)` der App; `Open` startet den Link.

## Integration

- Konsumenten: `app_claude.go` (Workbench), `uiapi.go` (Studio).
