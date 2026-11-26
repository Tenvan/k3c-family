# RU70 · DEV · Studio-Backend in Rust: Welten, Hub, Assets, Spiel-EXE-Anbindung, MCP

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-26 (Leitfaden 🧑 2026-11-26)

## Ausgangslage

RU66 legt den Cargo-Workspace `rust/tools/` mit `k3c-kit` an (Module `mcp`, `applog`, `console`, `proc`, `usage`, `winicon`, `conlog`, `uikit`, `claude`), RU67 das Crate `k3c-assetkit`. Der Spike RU65.1 (`rust/tools/spike/`) zeigt Tauri 2 mit `rmcp`, Commands, Events und den Frontend-Adapter (`frontend-api/`). Entscheidung 014 ist angenommen (🧑 2026-11-26); alle Vorentscheidungen stehen im Leitfaden `docs/standards/rust-werkzeuge.md`.

Das Go-Studio `tools/k3c-studio` hat rund 19.500 Zeilen Go und 11.800 Zeilen Tests: Wurzel (`app*.go`, `main.go`, `headless.go`, `uiapi.go`), `internal/mcpsrv`, `world`, `balance`, `paperassets`, `simrun`, `review`, `assetsui`, `hubplan`, `gameexe`, `gamedata`, `studioview`, `datadoc`, `exeengine`. Seit RU40/RU41 rechnet es nur noch über die Spiel-EXE (`k3c-game sim …`), die Go-Engine ist entfernt.

## Ziel

Das Studio-Backend läuft als Tauri-2-App in Rust (`rust/tools/crates/k3c-studio`) hinter dem bis auf den Adapter unveränderten React-Frontend: Welten, Inseln, Hub, Assets, Balancing und Berichte über die Spiel-EXE, MCP-Server mit allen Tools der Go-Fassung auf 127.0.0.1:5281 (bis RU71). Die Go-Fassung bleibt bis RU71 unverändert lauffähig.

## Beteiligte und Zielgruppen

Der Second Worker (Lane `zweit`, Modell Sonnet) setzt autonom um, ohne Rückfragen und ohne Haltepunkt. Die Abnahme am PC (Fenster, Gefühl, Bauzeit) ist nicht Teil von RU70: 🧑 sammelt sie in B-1295 (Sprint RU71).

## Anforderungen

Backend von `tools/k3c-studio` in Rust, Frontend bis auf den Adapter unverändert. Reihenfolge der Sessions: 1. Gerüst (Fenster, Aufruf-Tabelle, Tauri-Adapter, UI-API, MCP 5281 mit den Berichte-Tools, Golden der Tool-Liste, Tasks). 2. Spiel-EXE (`gameexe`), `level_generate`, „Im Spiel ansehen“. 3.–6. Welten, Inseln, Chunks samt `world_*` und Bindings. 7. Hub. 8.–13. Assets (`review`, `assetsui`, `paperassets`, `assets_*`). 14.–20. Balancing, Sim, Berichte (`balance`, `simrun`, `datadoc`, `sim_*`, `replay_run`, Bindings `Balance*`). 21. Review.

### Vorentscheidungen (verbindlich für alle Sessions)

- **V1 Crate und Module:** `rust/tools/crates/k3c-studio` ist lib + bin `k3c-studio` (`src/main.rs` ruft nur `k3c_studio::run()`). Ein Modul je Go-Paket: `src/app/` (Wurzel: `app.go` → `app/mod.rs`, `main.go` → `app/window.rs`, `headless.go` → `app/headless.rs`, `uiapi.go` → `app/rpc.rs`, `app_<x>.go` → `app/<x>.rs`), `src/mcpsrv/`, `src/world/`, `src/hubplan/`, `src/assetsui/`, `src/review/`, `src/paperassets/`, `src/studioview/`, `src/gameexe/`, `src/exeengine/`, `src/gamedata/`, `src/datadoc/`, `src/balance/`, `src/simrun/`; alle als `pub mod` in `lib.rs`. Go-Datei `x.go` → `x.rs`; die nach dem Paket benannte Datei (`world.go`, `hubplan.go`, `review.go`, `studioview.go`, `datadoc.go`, `exeengine.go`, `server.go`) → `mod.rs`. Test `x_test.go` → `x_tests.rs` neben `x.rs`, eingebunden mit `#[cfg(test)] #[path = "x_tests.rs"] mod tests;`; Hilfs-Testdateien (`helpers_test.go`, `*_helpers_test.go`, `tools_assets_paper_test.go`) → `test_helpers.rs` (`#[cfg(test)] pub(crate) mod test_helpers;`). Prozess-, HTTP-, Tauri- und Golden-Tests unter `tests/`.
- **V2 Ports:** einzige Stelle `src/ports.rs`: MCP 5281, UI-API 8293, Vite 5294, Fenstertitel `K3C Studio (Rust)`. Eigene Umgebungsvariablen bis RU71: `K3C_STUDIO_RS_PORT`, `K3C_STUDIO_RS_UI_API_PORT` (nicht die Go-Namen, damit ein gesetztes `K3C_STUDIO_PORT` nie das Go-Studio stört). RU71 ändert nur diese Datei.
- **V3 Fenster:** wie `main.go` (`windowOptions`): Größe und Mindestgröße wie `assetkit/windowstate`, Hintergrund `#0b1026`, verdeckter Start bis zur gemerkten Position; Größe/Position über das Plugin `window-state`, Zweitstart holt das Fenster nach vorn (Plugin `single-instance`, Gegenstück zu `secondInstance`). Tauri-`identifier` `de.tenvan.k3c.studio-rs`, WebView2-Datenordner `%LOCALAPPDATA%\k3c-studio`, `withGlobalTauri: true`. Kein `-dev`-Supervisor (`engine/devrun` entfällt laut Leitfaden).
- **V4 Eine Aufruf-Tabelle:** `src/app/rpc.rs` hält je Go-Methode der App (`uiMethods` in `uiapi.go`, gleiche Namen in PascalCase) einen Eintrag `(&str, fn(&App, &[serde_json::Value]) -> Result<Value, String>)`; Argumente positionell als JSON-Array wie `devRpc`/Wails, Fehlertexte wie Go. Dieselbe Tabelle bedient den Tauri-Command `rpc {method, args}` und die UI-API über `k3c_kit::uikit` (`POST /api/dev/rpc/<Methode>`, SSE `/api/dev/events`, Prüfung von Host, Origin und `X-K3C-Dev`). Ereignisse (`mcp:state`, `world:changed`, `balance:runs`, `balance:reports`) gehen über eine Senke `Arc<dyn Fn(&str, Value) + Send + Sync>`: im Fenster an `AppHandle::emit` und den Bus, headless nur an den Bus (wie `emitAll`). Test `ui_methods_wie_wails_ts` liest die Schlüssel von `GoApp` aus `tools/k3c-studio/frontend/src/api/wails.ts`; die Liste `PENDING_METHODS` in `src/app/rpc_tests.rs` schrumpft je Session und ist am Ende leer.
- **V5 Frontend-Adapter:** nur `tools/k3c-studio/frontend/src/api/**` ändert sich. `http.ts` wird zur Fabrik `rpcBackend(rpc, on, openUrl)` mit derselben Methodenzuordnung, `httpBackend()` nutzt sie; neu `tauri.ts` (`hasTauri`, `tauriBackend`: `rpc` = `__TAURI__.core.invoke('rpc', {method, args})`, abgelehnte Aufrufe als `Error(String(e))`, `on` = `event.listen` mit Abmeldung wie im Spike, `openUrl` = `invoke('open_url', {url})`); Weiche in `index.ts`: `hasTauri() ? tauriBackend() : hasWails() ? wailsBackend() : wantsMock() ? mockBackend() : httpBackend()`. `wails.ts` bleibt für die Go-Fassung. Test `tauri.test.ts` mit einem Fake-`window.__TAURI__`. Rust-Command `open_url`: nur `http://`/`https://`, Öffnen über `ShellExecuteW` (Crate `windows`).
- **V6 MCP:** `rmcp` mit `#[tool_router]`/`#[tool]`, Streamable HTTP unter `/mcp` nur auf 127.0.0.1, Eigenheiten (Kopf `X-K3C-Root`, Argument `checkout`, Checkout-Zeile der Schreib-Tools, Aufrufstatistik, Fortschritt) aus `k3c_kit::mcp`. `checkout_policy.go` wird 1:1 portiert. Tool-Namen, Beschreibungen, Parameter samt Beschreibung, Pflichtfelder, Annotations und Antworttexte gleich Go. `instructions.md` liegt als Byte-Kopie unter `src/mcpsrv/instructions.md` (Test vergleicht mit der Go-Datei, solange es sie gibt). Die Prompts `asset-<id>` gehören zu den Asset-Tools.
- **V7 Golden der Tool-Liste:** RU70.1 erfasst `rust/tools/crates/k3c-studio/tests/golden/go-tools.json` aus der Go-Fassung: `task k3c-studio:headless:build`, Start von `tools/k3c-studio/build/bin/k3c-studio-headless.exe -headless` aus der Repo-Wurzel mit `K3C_STUDIO_PORT=5289` und `K3C_STUDIO_UI_API_PORT=8299` (nie 5181, 5182, 8193), Erfassung mit `cargo run -p k3c-studio --example capture_go_tools -- http://127.0.0.1:5289/mcp` (initialize, tools/list; je Tool `name`, `description`, `annotations`, `inputSchema`; nach Name sortiert), Prozess danach beenden. Scheitert der Bau, wird gegen das laufende Go-Studio auf 5181 nur gelesen erfasst (im Ergebnis nennen). Test `tests/mcp_tools_golden.rs` normalisiert beide Seiten (Schema nur `properties` mit `type` ohne `null`, `description`, `enum`, `items.type` sowie sortiertes `required`; `$schema`, `title`, `format`, `default`, `additionalProperties` entfallen) und prüft: jedes Rust-Tool steht gleich in der Golden; Golden-Namen = Rust-Namen ∪ `PENDING_TOOLS`; `PENDING_TOOLS` ∩ Rust-Namen leer; solange `tools/k3c-studio/internal/mcpsrv` existiert, sind die `Name: "…"` der Go-Quellen (ohne `*_test.go`, ohne Prompts) gleich den Golden-Namen. Die Golden wird nie von Hand geändert.
- **V8 Spiel-EXE:** Läufe nur über die Spiel-EXE wie in Go (`gameexe`), keine Einbindung von `k3c-sim` oder `k3c-level`. Tests mit Spiel-EXE: ohne frische EXE `eprintln!("übersprungen: …")` und `return` (Gegenstück zu `t.Skip`); Tests bauen die EXE nie. Go-`TestHelperProcess`-Muster: eigenes Bin `gameexe-helper` (`[[bin]]`, Pfad `tests/support/gameexe_helper.rs`), Prozess-Tests in `tests/gameexe_run.rs` mit `env!("CARGO_BIN_EXE_gameexe-helper")`; `default-run = "k3c-studio"`. Go-Tests, die nur mit `-short` übersprungen werden (bun, git), laufen in Rust immer.
- **V9 Fremde Go-Pakete:** `k3c/data` (`data.Files`) → Lesen von der Platte aus `K3C_DATA_DIR` oder `<root>/data/` (`src/balance/datafiles.rs`, ohne Einbetten; Abweichung gewollt). `tools/ruletable/table` (`Block`, `Ersetzen`, `Gegenprobe`) → `src/hubplan/table.rs` samt Tests aus `table_test.go`. `tools/assetkit` → `k3c_assetkit`; `assetkit/windowstate` → Plugin `window-state`. `github.com/google/jsonschema-go` → Crate `jsonschema` (Rust, MIT, verbreitet); Fehlertexte der Schema-Prüfung dürfen abweichen, Tests prüfen dort nur Fehler ja/nein.
- **V10 Lücken in k3c-kit oder k3c-assetkit:** nicht dort ändern (nicht im Bereich), sondern im Studio-Crate unter `src/kitx.rs` nachbauen und unter `### Für den Hauptrechner` melden.
- **V11 Aufteilung gekoppelter Pakete** (`world`, `balance`, `paperassets`, `simrun`): Braucht eine Datei der Session Typen oder Funktionen aus einer Datei einer späteren Session, werden Typen vollständig wie in Go vorgezogen; Funktionen dürfen als Signatur mit Rumpf `unimplemented!("RU70.N")` (N = spätere Session) stehen, wenn kein Test dieser Session sie aufruft. Die spätere Session ersetzt sie. Ist es einfacher, eine spätere Datei ganz vorzuziehen, ist das erlaubt (im Ergebnis nennen; die spätere Session übernimmt sie nur noch). Am Ende ist `rg 'unimplemented!\("RU70' rust/tools/crates/k3c-studio` leer.
- **V12 Allgemein:** Temporäres nur über `tempfile::tempdir_in(<root>/.work)` oder `K3C_TMP`; Log-Meldungen mit Emoji über `k3c_kit::conlog`/`applog`; Datei ≤ 400, Funktion ≤ 60 Zeilen; `cargo test` und `clippy` im Workspace `rust/tools` (Ziel `rust/tools/target`).
- **V13 Tasks und Tauri-Bau** (in `rust/tools/Taskfile.yml`, Namensraum `tools-rust`): `studio:build` (`bunx tauri build --no-bundle` im Crate-Ordner, EXE `rust/tools/target/release/k3c-studio.exe`, nicht nach `bin/`), `studio:run` (bauen, aus der Repo-Wurzel starten), `studio:dev` (Browser-Dev: Backend `cargo run -p k3c-studio -- --headless` und Vite mit `K3C_STUDIO_UI_PORT=5294`, `K3C_STUDIO_API=http://127.0.0.1:8293`), `studio:window` (`bunx tauri dev`). `@tauri-apps/cli` 2.x als devDependency in `rust/tools/package.json` (falls RU66 sie nicht anlegt), Version fest in `rust/tools/bun.lock`. `tauri.conf.json`: `frontendDist` `../../../../tools/k3c-studio/frontend/dist`, `devUrl` `http://localhost:5294`, `beforeDevCommand` Vite auf 5294 im Frontend-Ordner, `beforeBuildCommand` `task k3c-studio:frontend` (Repo-Wurzel). Icon: Kopie von `tools/k3c-studio/icon/icon.ico` nach `icons/icon.ico`.
- **V14 Go unverändert:** außer `tools/k3c-studio/frontend/src/api/**` ändert RU70 nichts unter `tools/`, `engine/`, `data/`. Funde in Go werden gemeldet, nicht behoben.
- **V15 Sim:** Für Lane `zweit` gilt die Sim-Sperre nicht, die Tests kommen trotzdem ohne echte Sim-Läufe aus. `sim_run` übernimmt die Sperrlogik der Go-Fassung (`K3C_SIM_ENTSPERRT`) unverändert.

## Nicht-Ziele

Workbench (RU69). Go entfernen, Ports und `.mcp.json` umstellen (RU71). Neue Studio-Funktionen. Rust-Crates statt Spiel-EXE im Prozess (eigenes Folge-Ticket nach Entscheidung 014). Abnahme am PC (B-1295, RU71). Änderungen an `k3c-kit`, `k3c-assetkit`, Go-Code, Seiten und Komponenten des Frontends.

## Regeln und Einschränkungen

Orakel ist die Go-Fassung: Tool-Namen, Parameter, Antworten des MCP, Methoden-Namen und JSON-Form der Aufruf-Tabelle, Ereignis-Namen und Dateiformate bleiben gleich. Jede Go-Testdatei der portierten Pakete hat eine Rust-Entsprechung oder steht mit Grund im Ergebnis. Balancing schreibt nur unter `data/worlds/` (Beschluss 🧑 2026-11-22, `writeguard`). Datei ≤ 400, Funktion ≤ 60 Zeilen. Keine Rückfragen: Offenes entscheidet die Session nach Leitfaden und Vorentscheidungen und meldet es unter `### Für den Hauptrechner`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/tools/crates/k3c-studio/**`, `rust/tools/Cargo.toml`, `rust/tools/Cargo.lock`, `rust/tools/Taskfile.yml`, `rust/tools/package.json`, `rust/tools/bun.lock`, `tools/k3c-studio/frontend/src/api/**`
- **Abhängig von:** RU66, RU67
- **Braucht 🧑:** –
- **Sperre:** deps, taskfile

## Beispiele

- `world_check` der Rust-Fassung meldet für dieselbe Welt dieselben Befunde mit demselben Text wie die Go-Fassung.
- `POST /api/dev/rpc/WorldList` auf 8293 und `invoke('rpc', {method: 'WorldList', args: []})` im Fenster liefern dasselbe JSON wie die Go-Methode `WorldList`.
- `tests/mcp_tools_golden.rs` schlägt fehl, wenn ein Tool eine andere Beschreibung oder ein Pflichtfeld weniger hat als in `tests/golden/go-tools.json`.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go; nie den portierten Test an Rust anpassen. Ein Fehler in Go wird nicht still behoben, sondern gemeldet (offensichtlicher Absturz darf behoben werden, im Ergebnis nennen). Fehlt eine Funktion in `k3c-kit`/`k3c-assetkit`: V10. Fehlt die Spiel-EXE oder `bun`/`git`: betroffene Tests melden „übersprungen“ wie in Go. Fehlende MCP-Funktion der Planung: Erweiterung vorschlagen (nicht selbst ändern).

## Akzeptanzkriterien

- **AC-01** Die Rust-Studio-App startet als Fenster (`task tools-rust:studio:run`) und headless (`k3c-studio --headless`, Browser-Dev `task tools-rust:studio:dev`) und bedient MCP auf 127.0.0.1:5281 und die UI-API auf 8293; ein Tauri-Mock-Test belegt Command `rpc` und Ereignis `mcp:state`.
- **AC-02** Alle MCP-Tools der Go-Fassung sind mit gleichen Namen, Beschreibungen, Annotations, Parametern und Pflichtfeldern vorhanden: `tests/mcp_tools_golden.rs` vergleicht die Tool-Liste mit der aus Go erfassten Golden `tests/golden/go-tools.json` und ist grün, `PENDING_TOOLS` ist leer.
- **AC-03** Jede Methode von `GoApp` (`tools/k3c-studio/frontend/src/api/wails.ts`) steht in der Aufruf-Tabelle, die Tauri-Command und UI-API bedient (`ui_methods_wie_wails_ts` grün, `PENDING_METHODS` leer); die Ereignisse `mcp:state`, `world:changed`, `balance:runs`, `balance:reports` kommen wie in Go.
- **AC-04** Jede Go-Testdatei der portierten Pakete hat eine Rust-Entsprechung oder steht mit Grund im Session-Ergebnis; Tests mit Spiel-EXE überspringen ohne EXE; `rg 'unimplemented!\("RU70' rust/tools/crates/k3c-studio` ist leer.
- **AC-05** Das Studio-Frontend hat den Tauri-Adapter (`src/api/tauri.ts`, Weiche in `src/api/index.ts`, Test `tauri.test.ts`); die Frontend-Vitest des Studios (`task k3c-studio:vitest`) ist grün; außerhalb von `tools/k3c-studio/frontend/src/api/` ist `tools/k3c-studio` unverändert.
- **AC-06** `task tools-rust:test`, `task tools-rust:lint` und `task check:all` sind grün.

## Offene Fragen

keine (Leitfaden, 🧑 2026-11-26)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU70.1 | `RU70.1-geruest-fenster-mcp-adapter.md` | Umsetzung | autonom | fertig |
| RU70.2 | `RU70.2-spiel-exe-level-studioview.md` | Umsetzung | autonom | fertig |
| RU70.3 | `RU70.3-welten-lesen-format.md` | Umsetzung | autonom | fertig |
| RU70.4 | `RU70.4-welten-bearbeiten-overrides.md` | Umsetzung | autonom | offen |
| RU70.5 | `RU70.5-welten-story-natur-endboss.md` | Umsetzung | autonom | offen |
| RU70.6 | `RU70.6-welten-mcp-bindings.md` | Umsetzung | autonom | offen |
| RU70.7 | `RU70.7-hub-planer-mcp.md` | Umsetzung | autonom | offen |
| RU70.8 | `RU70.8-assets-review-paket.md` | Umsetzung | autonom | offen |
| RU70.9 | `RU70.9-assets-oberflaeche-methoden.md` | Umsetzung | autonom | offen |
| RU70.10 | `RU70.10-paperassets-basis-stil-patch.md` | Umsetzung | autonom | offen |
| RU70.11 | `RU70.11-paperassets-anlegen-set-job.md` | Umsetzung | autonom | offen |
| RU70.12 | `RU70.12-mcp-assets-grundtools-prompts.md` | Umsetzung | autonom | offen |
| RU70.13 | `RU70.13-mcp-assets-render-recherche-review.md` | Umsetzung | autonom | offen |
| RU70.14 | `RU70.14-balance-berichtstypen-kennzahlen.md` | Umsetzung | autonom | offen |
| RU70.15 | `RU70.15-balance-ziele-hinweise-baselines.md` | Umsetzung | autonom | offen |
| RU70.16 | `RU70.16-balance-tuning-fortschritt-insights.md` | Umsetzung | autonom | offen |
| RU70.17 | `RU70.17-balance-auswertung-texte.md` | Umsetzung | autonom | offen |
| RU70.18 | `RU70.18-simrun-register-spec-stempel.md` | Umsetzung | autonom | offen |
| RU70.19 | `RU70.19-simrun-offline-berichte-datadoc.md` | Umsetzung | autonom | offen |
| RU70.20 | `RU70.20-mcp-sim-balance-bindings.md` | Umsetzung | autonom | offen |
| RU70.21 | `RU70.21-review-abnahme.md` | Review | autonom | offen |

Reihenfolge = Tabelle; jede Session hängt an der vorigen (RU70.1 an RU66 und RU67).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
