# rust/tools/

## Responsibility

Eigener Cargo-Workspace der Entwickler-Werkzeuge (Entscheidung 014): gemeinsamer Code, Asset-Logik und die Tauri-Backends von Workbench und Studio. Leitfaden: `docs/standards/rust-werkzeuge.md`.

## Design

- `crates/k3c-kit` ([codemap](crates/k3c-kit/codemap.md)): gemeinsamer Code; hängt von keiner App ab.
- `crates/k3c-assetkit` ([codemap](crates/k3c-assetkit/codemap.md)): Asset-Logik.
- `crates/k3c-workbench` ([codemap](crates/k3c-workbench/codemap.md)) und `crates/k3c-studio` ([codemap](crates/k3c-studio/codemap.md)): Apps; beide nutzen `k3c-kit` und `k3c-assetkit`, nicht einander.
- Toolchain und rustfmt erbt der Workspace aus `rust/`; `clippy.toml` liegt hier, Build-Ordner `rust/tools/target`. `package.json` hält nur die Tauri-CLI.

## Flow

Je App: `--headless` startet MCP-Server und Oberflächen-API ohne Fenster, sonst das Tauri-Fenster; beide Wege nutzen dieselbe Aufruf-Tabelle. Die Frontends liegen unter `tools/<app>/frontend`.

## Integration

- Tasks `tools-rust:build|test|lint|fmt` (`Taskfile.yml` hier), `k3c-workbench:*`, `k3c-studio:*`.
- `task check:diff` prüft die geänderten Crates, `task check:dev` den ganzen Workspace; CI-Job `tools-rust`.
