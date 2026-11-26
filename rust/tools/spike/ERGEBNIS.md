# RU65.1 · Ergebnis Spike Tauri 2 (2026-11-25)

Kurz: Alle drei Aufgaben laufen. Command und Event, MCP-Server über `rmcp` (Streamable HTTP, auch Claude Code ruft die
Tools auf) und Dev-Betrieb mit Hot Reload (Frontend per HMR, Rust per Neubau und Neustart) sind belegt. Es gibt keinen
Ausschlussgrund. Offen bleiben nur Prüfungen am Bildschirm durch Ralf (unten).

## Aufbau

| Teil | Ort | Inhalt |
|---|---|---|
| Rust-Backend | `src-tauri/` (eigener Cargo-Workspace) | Tauri 2.12.7, rmcp 3.11.0, axum 0.8.9, tokio 1; 6 Quelldateien, 304 Zeilen samt `build.rs` |
| Frontend | `tools/k3c-workbench/frontend/` + `tools/webkit/` | Kopie aus dem Repo (gleiche relative Lage, damit `@webkit` auflöst); geändert sind nur `src/api/tauri.ts` (neu, 44 Zeilen) und `src/api/index.ts` (eine Weiche) |
| Tests | `src-tauri/tests/spike.rs` | 2 Tests: Command + Event (Tauri-Mock-Laufzeit), MCP über echtes HTTP |
| Messung | `measure.ps1`, `cdp.mjs` | Startzeit und Speicher (Prozessbaum); Fenster-Prüfung per Chrome DevTools Protocol |
| Umgebung | `env.sh` | `CARGO_HOME`, `CARGO_TARGET_DIR`, `TMP`/`TEMP`, Bun- und npm-Cache, WebView2-Datenordner, alles unter `.work/spike-tauri` |
| Belege | `logs/spike.log`, `logs/dev.log`, `shots/*.png` | Log mit Zeitstempel und PID, Screenshots aus dem echten Fenster |

Herkunft der Abhängigkeiten (Prüfung vor dem Hinzufügen): `tauri` und `@tauri-apps/cli` von tauri-apps (MIT/Apache-2.0),
`rmcp` ist das offizielle Rust-SDK der Organisation modelcontextprotocol (Apache-2.0, dieselbe Organisation wie das
Go-SDK `github.com/modelcontextprotocol/go-sdk`, das `tools/workkit/mcpkit` heute nutzt). Mindest-Rust: tauri 1.90,
rmcp 1.88; der Workspace hat 1.99.

Bedienung (Ralf):

```bash
source .work/spike-tauri/env.sh && cd $S
bunx tauri dev                 # Fenster mit Vite-HMR (Port 5291) und Neustart bei Rust-Änderung; MCP auf 127.0.0.1:5280/mcp
bunx tauri build --no-bundle   # Release-EXE: target/release/k3c-spike-tauri.exe
cd src-tauri && cargo test     # beide Tests
```

## Befunde je Aufgabe

### (1) Command und Event: belegt

- **Test** `command_spike_ping_antwortet_und_sendet_event`: `invoke('spike_ping', {msg})` über die IPC der Mock-Laufzeit liefert `pong: test`, das Event `spike:pong` kommt beim Listener an. Grün.
- **Echtes Fenster** (Release-EXE, CDP auf das WebView2): `window.spikePing('cdp')` liefert `pong: cdp`, das Event `spike:pong {"msg":"cdp","n":3}` kommt im JavaScript an, Hin und Zurück 1,5 ms. 100 Commands `info` nacheinander: 0,96 ms je Aufruf (im Dev-Build 1,7 ms).
- **Workbench-Oberfläche echt angebunden:** `info` (Version, MCP-Zustand) und alle Ereignisse kommen aus Rust; der Header zeigt das grüne Badge `MCP http://127.0.0.1:5280/mcp` (Screenshot `shots/release-notice.png`). Die übrigen Seiten zeigen weiter Mock-Daten (kein Port von Produktivcode).
- **Log-Quittung:** Das Frontend quittiert jedes Ereignis mit dem Command `ui_event_ack`; `logs/spike.log` zeigt je Ereignis die Zeile `✅ UI hat Event empfangen` 1–3 ms nach dem Senden.

### (2) MCP-Server über rmcp (Streamable HTTP): belegt

- **Test** `mcp_streamable_http_tools_list_und_call`: `initialize`, `notifications/initialized`, `tools/list`, `tools/call` für `workbench_status` und `notify_ui` über HTTP mit Session-Kopf; Ereignisse `mcp:state` und `ui:notify` gehen an die Oberfläche. Grün.
- **curl gegen die laufende EXE:** Protokoll 2025-06-18, Session-ID im Kopf `Mcp-Session-Id`, Fehler bei falschem Argument als JSON-RPC `-32602` mit Text; ein fremder `Host`-Kopf wird mit 403 abgewiesen (Schutz gegen DNS-Rebinding ist in rmcp Vorgabe, nur Loopback).
- **Claude Code** (2.1.323): `claude -p --strict-mcp-config --mcp-config .work/spike-tauri/mcp-spike.json --no-session-persistence` hat sich verbunden, `workbench_status` und `notify_ui` aufgerufen und die Antwort wörtlich zurückgegeben (33 s samt Modell). Statt `claude mcp add` gewählt, weil `mcp add` die Konfiguration außerhalb von `.work` ändert.
- **MCP → Oberfläche:** `notify_ui` erscheint als Toast der echten `Notices`-Komponente (`shots/release-notice.png`, Text per CDP gelesen: „Hallo aus Rust-MCP“).
- Tool-Definition mit Makros (`#[tool_router]`, `#[tool]`, `#[tool_handler]`), Argument-Schema aus `schemars`; 100 Zeilen für Server, zwei Tools und Bindung an axum.

### (3) Dev-Betrieb mit Hot Reload: belegt

- `bunx tauri dev` startet Vite (Kopie des Workbench-Frontends, Port 5291) und `cargo run`, beobachtet `src-tauri/`.
- **Frontend:** Änderung an `App.tsx` → Vite `hmr update /src/App.tsx`; der Header zeigt den neuen Text, eine vorher gesetzte JavaScript-Marke bleibt erhalten (kein Neuladen).
- **Rust:** Änderung an `state.rs` (`BUILD_TAG` spike-1 → spike-2) → Neubau, alter Prozess beendet, neuer Prozess: 12,2 s bis Prozessstart, 14,6 s bis „UI bereit“; MCP lauscht danach wieder auf 5280 und meldet `Build: spike-2`. Vite läuft dabei weiter.
- Nach dem Beenden von `tauri dev` lief kein Prozess des Spikes mehr und kein Port war belegt.

## Messwerte

Rechner: Entwicklungs-PC, Windows 11, Rust 1.99.1, WebView2 158.0.4370.33. Alle Zahlen sind einzelne Läufe (keine Statistik).

| Messung | Wert | Art |
|---|---|---|
| Download der Crates (`cargo fetch`, 458 Pakete im Lockfile) | 63 s, `cargo-home` 674 MB | belegt |
| Bau kalt, Debug (`cargo build`, 411 Einheiten, Crates schon geladen) | 2 min 28 s | belegt |
| Bau warm, Debug (eine Datei des Spikes geändert) | 9,5 s | belegt |
| Bau kalt, Release (`tauri build --no-bundle`, samt Frontend 2 s) | 2 min 17 s Cargo, 143 s gesamt | belegt |
| Bau warm, Release (eine Datei geändert, thin LTO) | 67 s | belegt; Anteil LTO vermutet |
| Rust-Änderung im Dev-Betrieb bis neues Fenster bereit | 14,6 s (Prozess nach 12,2 s) | belegt |
| Frontend-Bau (`tsc --noEmit && vite build`) | 2 s | belegt |
| Größe Release-EXE (strip, thin LTO, Frontend eingebettet) | 14,6 MB | belegt |
| Größe Debug-EXE | 20,1 MB | belegt |
| Platz `target/` (Debug, Release, Tests, Clippy) | 6,4 GB | belegt |
| Start bis „UI bereit“ (Release, 3 Läufe) | 1632 / 1162 / 1259 ms; davon Fenster samt WebView2 rund 0,8–0,9 s | belegt |
| Speicher im Leerlauf nach 8 s, nur EXE | 29 MB Working Set, 6,5 MB privat | belegt |
| Speicher im Leerlauf, Prozessbaum (EXE + 6 × msedgewebview2) | 416–444 MB Working Set, 332–345 MB privat | belegt |
| IPC `info`, 100 Aufrufe nacheinander | 0,96 ms je Aufruf (Release) | belegt |
| Command + Event Hin und Zurück | 1,5 ms (Release), 2,9 ms (Dev) | belegt |
| Clippy | 3 Hinweise „collapsible if“, keine Fehler | belegt |

Vergleich mit der Wails-Workbench (Go):

| Messung | Wails (Go) | Tauri-Spike | Art |
|---|---|---|---|
| EXE | 19,4 MB (`bin/k3c-workbench.exe`, voller Funktionsumfang) | 14,6 MB (nur Spike) | belegt, aber ungleicher Umfang |
| Backend-Prozess | 50 MB Working Set, 66 MB privat (laufende Instanz mit echten Daten) | 29 MB / 6,5 MB (Leerlauf, Mock-Seiten) | belegt, nicht gleichwertig |
| WebView2-Anteil | 584 MB Working Set (6 Prozesse, langlaufend) | 387–414 MB | belegt, nicht gleichwertig; bei gleicher Seite vermutet gleich, da gleiche WebView2-Laufzeit |
| Bauzeit, Startzeit | nicht gemessen (Go-Cache liegt außerhalb des Repos) | siehe oben | Go baut nach Erfahrung in Sekunden: vermutet |

## Nebenbefunde

- **Testbinaries unter Windows** brechen ohne Common-Controls-v6-Manifest mit `STATUS_ENTRYPOINT_NOT_FOUND` ab; gelöst in `build.rs` (Manifest nur für Testziele per Linker einbetten). Gilt für jede Tauri-App im Workspace.
- **WebView2-Datenordner** liegt bei Tauri sonst unter `%LOCALAPPDATA%\<identifier>`; im Spike per `data_directory` nach `.work/spike-tauri/webview-data` gelegt.
- **Ereignis vor Listener:** `mcp:state` beim Start (Rust +880 ms) kommt vor „UI bereit“ (+980 ms) und geht verloren; die Oberfläche holt den Stand darum wie bei Wails zusätzlich per `info`. `listen` registriert asynchron: erst `await listen`, dann rufen (im CDP-Skript so gelöst).
- **„Frontend unverändert“ geht nicht ganz:** Die Oberfläche braucht einen Tauri-Adapter (`src/api/tauri.ts`, 44 Zeilen, ohne npm-Paket dank `withGlobalTauri`) und eine Zeile in `src/api/index.ts`. Das betrifft RU69 und RU70.
- **rmcp** kennt das Protokoll 2026-07-28 (ohne Sessions, SEP-2567) schon; der Spike nutzt die Vorgabe (Sessions für ältere Versionen).
- **Tauri 3** liegt als Alpha vor (3.0.0-alpha.6); Tauri 2.12 ist die stabile Linie.
- Nach einem Rust-Neustart kennt der neue Prozess die alten MCP-Sessions nicht; ob Claude Code dann selbst neu verbindet, ist ungeprüft (bei Go mit `devrun` gleich).

## Was nur Ralf am PC prüfen kann

- **Ralf:** Fenster bedienen (Klicks, Größe ändern, Dark/Light, DPI, Taskleisten-Icon): `bunx tauri dev` oder die Release-EXE starten. Belegt sind nur Screenshot und JavaScript-Auswertung per CDP.
- **Ralf:** `claude mcp add --transport http k3c-spike http://127.0.0.1:5280/mcp` in der eigenen Sitzung (Claude hat stattdessen `--mcp-config` genutzt, um die Konfiguration nicht zu ändern) und danach wieder `claude mcp remove k3c-spike`.
- **Ralf:** ob beim ersten Start eine Firewall-Abfrage kam (gebunden wird nur 127.0.0.1, darum keine erwartet: vermutet).
