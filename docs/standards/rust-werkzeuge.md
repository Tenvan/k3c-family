# Leitfaden: Werkzeuge nach Rust (RU66 bis RU71)

Verbindlich für alle Sessions der Sprints RU66, RU67, RU69, RU70 und RU71 (Entscheidung 014, angenommen 🧑 2026-11-26).
Die Sessions laufen autonom auf dem Second Worker (`Lane: zweit`). **Es gibt keine Rückfragen:** Was hier nicht steht,
entscheidet die Session nach dem nächstliegenden Punkt dieses Leitfadens und meldet es im Ergebnis unter
`### Für den Hauptrechner`.

## Vorentscheidungen (🧑 2026-11-26)

1. **Tauri 2 statt Wails, Frontends bleiben.** React-Frontends (`tools/k3c-workbench/frontend`, `tools/k3c-studio/frontend`)
   und `tools/webkit` ändern sich nur am Adapter (`src/api/tauri.ts` neu, eine Weiche in `src/api/index.ts`, Muster aus
   dem Spike). Seiten, Komponenten und Tests bleiben.
2. **Eigener Cargo-Workspace `rust/tools/`** (eigene `Cargo.toml` mit `[workspace]`, eigenes `Cargo.lock`, Build-Ordner
   `rust/tools/target/`), damit `cargo test --workspace` des Spiels die rund 400 Tauri-Crates nicht mitbaut. Crates:
   - `rust/tools/crates/k3c-kit` – gemeinsamer Code (RU66): Module `mcp`, `applog`, `console`, `proc`, `usage`,
     `winicon`, `conlog`, `uikit`, `claude`; ein Crate mit Modulen (nicht ein Crate je Paket).
   - `rust/tools/crates/k3c-assetkit` – Asset-Logik (RU67).
   - `rust/tools/crates/k3c-studio` – Studio-App (RU70), Tauri-Anwendung (`src/`, `build.rs`, `tauri.conf.json`).
   - `rust/tools/crates/k3c-workbench` – Workbench-App (RU69), ebenso.
   - Spiel-Crates (`k3c-data`, `k3c-level`, `k3c-sim` …) werden per Pfad eingebunden, wenn nötig
     (`k3c-data = { path = "../../../crates/k3c-data" }`); Läufe gehen weiter über die Spiel-EXE wie in Go (kein
     Verhalten ändern).
3. **Abhängigkeiten freigegeben** (Versionen fest im Lockfile): `tauri` 2.12 (`tauri-build` 2.6, Feature `test` für
   Tests), `rmcp` 3.11 (Features `server`, `macros`, `transport-streamable-http-server`), `axum` 0.8, `tokio` 1,
   `serde`/`serde_json` 1, `schemars` 1, `reqwest` 0.12 (`default-features = false`, Features `json` und `native-tls` – Schannel, kein C-Compiler; normal für `k3c-assetkit` und GitHub-Abfragen, sonst nur dev), `windows` (Job-Objekt,
   CTRL_BREAK, Fenster-Icon), `notify` (Datei-Watch), `regex`, `chrono` oder `time`, `sha2`, `walkdir`, `tempfile` (nur
   Tests, Ordner unter `.work/`), Tauri-Plugins `window-state` und `single-instance`. Weitere Crates nur, wenn reines
   Rust, Lizenz MIT oder Apache-2.0 und verbreitet (über 1 Mio. Downloads); im Ergebnis melden.
4. **Go bleibt lauffähig, bis RU71 löscht.** Bis dahin laufen die Rust-Apps auf eigenen Ports, damit Go-Workbench und
   Go-Studio (die der Worker selbst über MCP nutzt) ungestört bleiben:

   | App | MCP | UI-API (Browser-Dev) | Vite |
   |---|---|---|---|
   | Rust-Workbench (bis RU71) | 5280 | 8290 | 5291 |
   | Rust-Studio (bis RU71) | 5281 | 8293 | 5293 → 5294 |
   | nach RU71 | 5180 / 5181 wie heute | 8191 / 8193 | 5191 / 5193 |

   Die Ports stehen je App an einer Stelle (Konstanten-Modul), RU71 ändert nur diese Stelle und `.mcp.json` bleibt gleich.
5. **Keine Haltepunkte.** Die Abnahme am PC (Fenster, Gefühl, Bauzeit) sammelt Ralf am Ende in **B-1295**; die Sessions
   laufen ohne Stopp durch. RU71 löscht Go, sobald RU69 und RU70 ihre Kriterien ohne die PC-Abnahme erfüllen (🧑
   2026-11-26: „Ja, direkt löschen“).
6. **Reihenfolge:** RU66 → RU67 → RU70 (Studio) → RU69 (Workbench) → RU71. Die Workbench zuletzt, weil jede Session an ihr
   hängt.

## Orakel und Verhalten

- **Orakel ist die Go-Fassung** (Entscheidung 012 sinngemäß): Tool-Namen, Parameter, Pflichtfelder, Wertebereiche und
  der Antworttext jedes MCP-Tools bleiben gleich; ebenso Commands/Bindings der Oberfläche (Name, Argumente, JSON-Form),
  Ereignis-Namen und Dateiformate unter `logs/`, `reports/`, `saves/`, `.work/`.
- **Tests werden mitportiert:** Jede Go-Testdatei des portierten Pakets hat eine Entsprechung (`*_tests.rs` neben dem
  Modul oder `tests/`). Ein Go-Test, der nicht portierbar ist, steht mit Grund im Ergebnis.
- **Planung:** gegen die Golden aus RU68 (`testdata/planung/`, README dort); `plan_*` muss sie bitgleich erfüllen.
- **Frontend-Vertrag:** `src/api/types.ts` (bzw. die Typen der App) ist die Form der Antworten; die Vitest-Suiten der
  Frontends laufen unverändert gegen den Mock. Der Tauri-Adapter wird per Test gegen einen Tauri-Mock geprüft (Muster
  Spike-Test `command_spike_ping_antwortet_und_sendet_event`).
- **Kein neues Verhalten** im Port. Funde (Fehler in Go) werden nicht stillschweigend behoben, sondern im Ergebnis
  gemeldet; ein offensichtlicher Absturz darf behoben werden (im Ergebnis nennen).

## Muster aus dem Spike (`rust/tools/spike/`)

Der Spike RU65.1 liegt als Referenz unter `rust/tools/spike/` (Quellen mit eigenem `[workspace]` und `Cargo.lock`; der Workspace `rust/tools/` nimmt ihn per `exclude = ["spike"]` aus; RU71 löscht ihn). Die Pfade in `tauri.conf.json` passen nur zum Aufbau in `.work/spike-tauri/`. `frontend-api/` zeigt den Adapter `tauri.ts` und die Weiche (`index.ts.diff`).
Messwerte und Befunde: `rust/tools/spike/ERGEBNIS.md`.

- **MCP:** `rmcp` mit `#[tool_router]`, `#[tool]`, `#[tool_handler]`; Argument-Schema über `schemars`; Streamable HTTP
  über `StreamableHttpService` an axum gebunden, nur `127.0.0.1`; Host-Schutz ist Vorgabe. Die Eigenheiten aus
  `workkit/mcpkit` (Kopf `X-K3C-Root`, Argument `checkout`, Schreibschutz, Fortschritt, Aufrufstatistik, Instructions)
  baut `k3c-kit::mcp` als axum-Middleware und Hilfen nach.
- **Fenster und Oberfläche:** `withGlobalTauri: true`, Adapter ohne npm-Paket; Events mit `app.emit`; die Oberfläche holt
  den Stand beim Start zusätzlich per Command (Ereignis vor Listener geht verloren).
- **Eine Aufruf-Tabelle je App:** Dieselben Funktionen bedienen Tauri-Commands (Fenster) und die axum-UI-API
  (Browser-Dev mit `httpBackend`), nicht zwei Wege wie heute (Bindings und `uiapi.go`).
- **Tests unter Windows** brauchen ein Common-Controls-Manifest: `build.rs` und `test-manifest.xml` wie im Spike
  (`cargo:rustc-link-arg-tests=/MANIFEST:EMBED`).
- **WebView2-Datenordner** explizit setzen (`%LOCALAPPDATA%\k3c-workbench` bzw. `k3c-studio`).
- **Kein `devrun`-Port:** Fenster-Dev über `bunx tauri dev`; Browser-Dev = Vite + Backend mit `--headless` (Neustart
  von Hand oder per Task). `engine/devrun` entfällt in RU71.

## Ablage, Tasks, CI

- **Taskfile:** Namensraum `tools-rust` in `rust/tools/Taskfile.yml` (in die Wurzel eingebunden) mit `build`, `test`,
  `lint` (`cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --check`), `run`, `dev`, je App
  `workbench:build|run|dev`, `studio:build|run|dev`. `check:all` ruft ab RU66.1 `tools-rust:test` und `tools-rust:lint`.
- **`check:diff`:** erkennt Änderungen unter `rust/tools/` und fährt dort `cargo test -p <crate>` und clippy (RU66.1
  erweitert `tools/checkdiff`, Muster wie für `rust/`).
- **CI:** eigener Job `tools-rust` (Windows) in `.github/workflows/ci.yml`, nur wenn `rust/tools/**` oder die Frontends
  geändert sind; Cache eigener Schlüssel.
- **Schreiben nur im Repo:** `CARGO_TARGET_DIR` bleibt `rust/tools/target`, Temporäres unter `.work/`; Tests nutzen
  `tempfile::tempdir_in(".work")` oder den Ordner aus `K3C_TMP`.
- **Logging:** Jede Meldung beginnt mit einem Emoji (CLAUDE.md › Logging), `k3c-kit::conlog` übernimmt die Topics aus
  `engine/conlog`.
- **Größe:** Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen (clippy `too_many_lines` auf 60).

## Fertig je Session

1. Portierte Tests grün: `task tools-rust:test` (bzw. `cargo test -p <crate>` im Workspace `rust/tools`).
2. `task tools-rust:lint` grün.
3. Go-Code unverändert (außer RU71 und ausdrücklich genannten Adapter-Stellen der Frontends); `task check:diff` grün.
4. Ergebnis mit Nachweis je Kriterium, Abweichungen unter `### Für den Hauptrechner`.
