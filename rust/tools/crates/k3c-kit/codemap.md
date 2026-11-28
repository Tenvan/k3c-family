# rust/tools/crates/k3c-kit/

## Responsibility

Gemeinsamer Code der Rust-Werkzeuge (Entscheidung 014). Importiert keine App.

## Design

- `logger` (Level, Senken, Attribute), `conlog` (farbige Konsolenzeile, Emoji je `ns` in `TOPICS`), `console` (Konsolenpuffer je Quelle), `applog` (JSON-Log unter `logs/` samt Spiegel in den Puffer).
- `mcp`: Gerüst der MCP-Server (Zähler, Checkout je Aufruf, Schreibschutz, Fortschritt, Statistik); `usage`: Nutzungsstatistik samt Datei.
- `uikit`: HTTP-API der Oberfläche im Browser-Dev mit Schutz (nur Loopback, gleiche Seite).
- `proc`: Kindprozesse, die sich samt Enkeln beenden lassen; `claude`: Deep Link „In Claude öffnen“; `winicon`: Fenster-Icon unter Windows.

## Flow

Eine App baut Logger und Senken, hängt ihre Tools an `mcp` und ihre Methoden an `uikit`; Kindprozesse startet sie über `proc`.

## Integration

- Konsumenten: `k3c-workbench`, `k3c-studio`.
- Tests neben den Modulen; `task tools-rust:test -- -p k3c-kit`.
