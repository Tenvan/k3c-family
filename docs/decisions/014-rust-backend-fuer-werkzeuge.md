# 014 · Rust-Backend für Workbench und Studio: Tauri 2 statt Wails

- **Status:** angenommen (🧑, 2026-11-26, Option A; Entwurf Claude, RU65.2)
- **Datum:** 2026-11-25
- **Projekt:** RSU · Sprints: RU65 (Spike und Vorlage), Folge-Sprints RU66, RU67, RU69, RU70, RU71; Orakel RU68 (erledigt)
- **Vorgänger:** 004 (Studio getrennt, Schnittstelle Dateien), 008 (natives Spiel in Go, überholt), 012 (Port-Orakel), 013 (neue Mechaniken nur in Rust, Entwurf)

## Anlass

Nach RU40 und RU41 hängt das Spiel nicht mehr an Go. Übrig bleiben die Werkzeuge: Workbench rund 19.000, Studio rund
19.500, `tools/workkit` 2.900, `tools/assetkit` 4.000 Zeilen Go ohne Tests (dazu rund 26.000 Zeilen Tests), beide Apps
als Wails v2 (Go-Backend, React-Frontend, `tools/webkit`). Ralf hat am 2026-11-25 einen schrittweisen Umstieg auf Rust
geplant (RU65 bis RU71). Der Spike RU65.1 sollte zeigen, ob Tauri 2 das Backend in Rust übernehmen kann: Command und
Event, MCP-Server über ein Rust-SDK, Dev-Betrieb mit Hot Reload. Die Befunde stehen in `rust/tools/spike/ERGEBNIS.md` (Spike-Quellen daneben; gebaut wurde in `.work/spike-tauri/`).

## Optionen

| Option | Bewertung |
|---|---|
| **A · Tauri 2, Backend in Rust** | eine Sprache und eine Toolchain im ganzen Repo; Studio kann die Rust-Crates (`k3c-level`, `k3c-sim`, `k3c-data`) später direkt einbinden statt die Spiel-EXE aufzurufen; Kosten rund 33–43 Sessions ohne neuen Spielnutzen, längere Bauzeiten |
| **B · Wails und Go bleiben** | kein Aufwand, nichts geht kaputt, Go baut in Sekunden; dafür bleiben zwei Toolchains (Go samt golangci-lint und Wails-CLI neben Rust), doppelte Datenmodelle zwischen Go-Werkzeugen und Rust-Spiel, und die Werkzeuge erreichen die Spiellogik nur über die Spiel-EXE |
| **C · Tauri 2, aber das Go-Backend als Sidecar behalten** | verworfen: zwei Prozesse und eine zusätzliche Brücke ohne Gewinn gegenüber A oder B |

## Messwerte (Spike RU65.1, Entwicklungs-PC, je ein Lauf)

Alle drei Aufgaben laufen; es gibt keinen Ausschlussgrund.

| Aufgabe | Ergebnis |
|---|---|
| Command und Event | belegt durch Test (Tauri-Mock-Laufzeit) und im echten Fenster per DevTools-Protokoll; `info` im Release 0,96 ms je Aufruf, Command samt Event 1,5 ms |
| MCP über `rmcp` 3.11 (offizielles SDK, Streamable HTTP) | belegt durch Test, curl und Claude Code (`claude -p --mcp-config`): `tools/list`, `workbench_status`, `notify_ui` samt Toast in der echten Oberfläche; Host-Prüfung gegen DNS-Rebinding ab Werk |
| Hot Reload | belegt: Frontend per Vite-HMR ohne Neuladen; Rust-Änderung bis neues Fenster bereit 14,6 s |

| Messung | Tauri-Spike | Wails-Workbench (Go) |
|---|---|---|
| Bau kalt Debug / Release | 2 min 28 s / 2 min 17 s (458 Crates, Download 63 s, 674 MB) | nicht gemessen; Sekunden (vermutet) |
| Bau warm Debug / Release | 9,5 s / 67 s (thin LTO) | nicht gemessen |
| Größe EXE | 14,6 MB (Spike-Umfang) | 19,4 MB (voller Umfang) |
| Start bis Oberfläche bereit | 1,2–1,6 s, davon WebView2 rund 0,85 s | nicht gemessen |
| Speicher Backend-Prozess | 29 MB Working Set, 6,5 MB privat (Leerlauf) | 50 MB / 66 MB (laufende Instanz, nicht gleichwertig) |
| Speicher samt WebView2 | 416–444 MB Working Set | gleiche WebView2-Laufzeit, Anteil vermutet gleich |
| Platz `target/` | 6,4 GB | – |

## Entscheidung

1. **Option A: Workbench und Studio bekommen ein Rust-Backend auf Tauri 2** (stabile Linie 2.12, Version fest im Lockfile). Das React-Frontend und `tools/webkit` bleiben; jede App bekommt einen kleinen Tauri-Adapter in `src/api/` (im Spike 44 Zeilen, ohne npm-Paket über `withGlobalTauri`). Die Formulierung „Frontend unverändert“ in RU69 und RU70 wird zu „Frontend bis auf den Adapter unverändert“.
2. **MCP über `rmcp`** (offizielles Rust-SDK, Gegenstück zum heutigen Go-SDK), Streamable HTTP auf denselben Ports (5180, 5181), nur Loopback; Tool-Namen, Parameter und Antworten bleiben gleich (Orakel: Go-Fassung, für die Planung die Golden aus RU68).
3. **Eine Aufruf-Tabelle je App für Fenster und Browser:** Dieselben Backend-Funktionen bedienen Tauri-Commands im Fenster und die HTTP-Oberflächen-API (axum) für den Browser-Dev-Betrieb mit `httpBackend`, damit es nicht zwei Wege wie heute (Wails-Bindings und `uiapi.go`) gibt.
4. **`engine/devrun` wird nicht portiert:** Neubau und Neustart übernimmt `tauri dev` (im Spike belegt); der Browser-Dev-Betrieb (Vite + Backend ohne Fenster) bekommt einen Schalter `-headless` im Rust-Backend und `cargo watch` oder ein kleines Task-Skript. RU66 schrumpft um `devrun`.
5. **Go bleibt lauffähig, bis RU71 löscht;** die Rust-Apps laufen bis dahin auf eigenen Ports. RU71 löscht Go, sobald RU69 und RU70 ihre Kriterien ohne die PC-Abnahme erfüllen (🧑 2026-11-26: „Ja, direkt löschen“).
6. **Keine Haltepunkte** (🧑 2026-11-26): Der Second Worker läuft RU66 bis RU71 ohne Stopp durch; Ralf nimmt am PC am Ende in einem Abnahme-Ticket ab.
7. **Eigener Cargo-Workspace `rust/tools/`** für Werkzeug-Crates und Apps, damit das Spiel die Tauri-Crates nicht mitbaut. Einzelheiten und alle Vorentscheidungen: `docs/standards/rust-werkzeuge.md`; Spike-Quellen als Referenz unter `rust/tools/spike/`.

## Aufwand je Folge-Sprint

Schätzung in Agent-Sessions (vermutet, Grundlage: Zeilen Go ohne Tests, rund 1.000–1.500 portierte Zeilen samt Tests je Session, Erfahrung aus RU10 bis RU22).

| Sprint | Inhalt | Umfang Go | Sessions | Wer prüft |
|---|---|---|---|---|
| RU66 | workkit nach Rust: `mcpkit` auf `rmcp`, `applog`, `console`, `proc` (Job-Objekt, CTRL_BREAK, Pipes), `usage`, `winicon`, `conlog`; ohne `devrun` | 2.900 + 800 (devrun entfällt) | 4–5 | Review-Session; Ralf am Haltepunkt |
| RU67 | assetkit nach Rust | 4.000 + 2.000 Tests | 3–4 | Review-Session |
| RU70 | Studio-Backend: Gerüst, Welten/Hub, Assets, Sim/Balancing über die Spiel-EXE, MCP 5181 | 19.500 + 11.800 Tests | 12–15 | Ralf am PC |
| RU69 | Workbench-Backend: Gerüst, Dienste/Tasks, Logs, Git, Planung (Golden RU68), Rest, MCP 5180 | 19.000 + 12.000 Tests | 12–16 | Ralf am PC |
| RU71 | Go restlos entfernen, Taskfile, CI, Doku | – | 2–3 | Ralf (Ja vor dem Löschen) |
| **Summe** | | | **33–43** | |

## Reihenfolge

1. RU66 Teil 1 (Gerüst, `mcpkit`, `applog`) → **Haltepunkt Ralf**.
2. RU66 Rest und RU67 (beide Sperre `deps`, darum nacheinander).
3. **RU70 Studio vor RU69 Workbench** (Änderung gegenüber der Nummerierung): Die Workbench trägt Planung, Prüfläufe und Dienste jeder Session; fällt sie aus, steht die Arbeit. Das Studio ist weniger kritisch, und an ihm zeigt sich der Gewinn (später Rust-Crates im Prozess statt Spiel-EXE). RU40 ist erledigt, RU70 hat danach keine offene Abhängigkeit mehr.
4. RU69 Workbench, Planung gegen die Golden aus RU68.
5. RU71 nach Abnahme von RU69 und RU70 und nach RU41.

## Risiken

| Risiko | Bewertung | Gegenmittel |
|---|---|---|
| **Bauzeit und Platz:** Rust-Neustart 14,6 s statt Sekunden, Release warm 67 s, `target/` 6,4 GB, rund 400 Crates mehr | belegt; spürbar im Dev-Alltag | Tauri-Crates in eigenem Workspace oder eigener CI-Stufe, damit `cargo build` des Spiels nicht mitbaut; Release mit `lto = false` für Werkzeuge (Einfluss ungeprüft) |
| **WebView2** | gering: Wails nutzt dieselbe Laufzeit (158.0); Datenordner muss gesetzt werden (Vorgabe `%LOCALAPPDATA%`) | `data_directory` wie im Spike |
| **Windows-Fenster:** Icon, Größe und Position merken, DPI, Single-Instance | ungeprüft im Spike | Tauri-Plugins `window-state`, `single-instance` (offiziell); Abnahme durch Ralf |
| **Prozesse, Job-Objekte, CTRL_BREAK, Pipes** (Dienste, Tasks, Prüfläufe) | ungeprüft; heute in `workkit/proc` und `devrun` (rund 300 Zeilen Windows-Code) | Crate `windows` liegt über Tauri schon im Baum; `tokio::process` für Pipes; RU66 Teil 2 mit Tests gegen Go als Orakel |
| **MCP-SDK `rmcp`:** schnelle Major-Versionen (3.x), Protokoll 2026-07-28 ohne Sessions | mittel | Version fest; die Eigenheiten aus `mcpkit` (Kopf `X-K3C-Root`/Argument `checkout`, Schreibschutz, Fortschritt, Statistik) über axum-Middleware nachbauen, Zugriff auf HTTP-Teile im Tool ungeprüft |
| **MCP nach Neustart:** neue Prozesse kennen alte Sessions nicht | wie heute mit `devrun`; Verhalten von Claude Code ungeprüft | prüfen in RU66 Teil 1 |
| **Tests unter Windows** brauchen ein Common-Controls-Manifest | gelöst im Spike (`build.rs`) | Muster übernehmen |
| **Tauri 3** (Alpha) | spätere Migration nötig, Tauri 2 bleibt gepflegt (vermutet) | bei 2.x bleiben, bis 3 stabil ist |
| **Kein Spielnutzen** während 33–43 Sessions; Haltung „ohne Spielnutzen keine neue Komplexität“ | belegt durch Umfang | Haltepunkt nach RU66 Teil 1; Go bleibt bis zur Abnahme |
| **Ereignis vor Listener** beim Start | wie bei Wails; Oberfläche lädt den Stand per `info` | beibehalten |

## Gründe für A

- Der Spike erfüllt alle drei Pflichtaufgaben ohne Umweg; das MCP-SDK deckt Streamable HTTP samt Sessions und Host-Schutz ab, Claude Code verbindet sich ohne Anpassung.
- Nach RU41 wäre Go nur noch für Werkzeuge im Repo: zweite Toolchain, golangci-lint, Wails-CLI, zweites Datenmodell neben `k3c-data`. Mit A wird das Repo einsprachig (Rust plus TypeScript für Oberflächen und `tools/paper`).
- Das Studio kann nach dem Port Welt-Prüfung und Level-Generator direkt aus den Rust-Crates rechnen statt über die Spiel-EXE (eigenes Folge-Ticket, kein Teil des Ports ohne Verhaltensänderung).
- Backend-Prozess schlanker (29 MB statt 50 MB Working Set; ungleiche Last, darum nur Richtwert), EXE kleiner.

## Gründe für B (Gegenstimme)

- Null Aufwand und null Risiko für die Werkzeuge, auf die jede Session angewiesen ist; Go baut und startet schneller.
- Der Nutzen von A liegt in Wartung und Einheitlichkeit, nicht im Spiel. Wer die 33–43 Sessions lieber in Spielfeatures steckt, wählt B; dann werden RU66, RU67, RU69, RU70 und RU71 verworfen.

## Folgen bei Annahme

- RU66 verliert `devrun` (Punkt 4), einen Haltepunkt gibt es nicht (Punkt 6); RU69 und RU70 erlauben den Tauri-Adapter im Frontend (Punkt 1) und die gemeinsame Aufruf-Tabelle (Punkt 3); RU70 rückt vor RU69.
- Projekt RSU nennt unter Nicht-Ziele noch „Workbench, `tools/assetkit`, `tools/paper` und die übrigen Werkzeuge nach Rust (bleiben Go und TypeScript)“; das wird für Workbench, Studio, workkit und assetkit gestrichen (`tools/paper` bleibt TypeScript).
- Neue Crates gelten mit der Annahme als freigegeben, Liste im Leitfaden `docs/standards/rust-werkzeuge.md` (RU65 › Regeln).
- Hinweis für RU71: `CLAUDE.md`, `codemap.md`, `requirements.md` und `docs/arbeitsweise.md` beschreiben danach Tauri statt Wails.

## Folgen bei Ablehnung

- RU66, RU67, RU69, RU70 und RU71 werden verworfen; RU41 entfernt nur die Go-Engine, Workbench und Studio bleiben Wails/Go.

## Annahme

Angenommen von Ralf am 2026-11-26: Option A; Haltepunkte entfallen; RU71 löscht Go ohne weiteres Ja; Regel B-925 gestrichen.
