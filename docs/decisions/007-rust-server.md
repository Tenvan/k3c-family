# 007 · Rust-Server: Architektur, Bibliotheken und Portreihenfolge (Etappe 2)

- **Status:** Entwurf (RS2.4, 2026-11-08, autonom, Claude). Freigabe nur durch Ralf; jede Crate gilt erst mit dieser Freigabe.
- **Datum:** 2026-11-08
- **Projekt:** RST · Ticket: B-736 (Sprint RS2, Session RS2.4)
- **Vorgänger:** 001 (Engine auf Go-Server), 004 (Datei-Watch, Neustart), 006 (Rust-Engine, Variante 2 = Etappe 2)

## Anlass

Ralf hat am 2026-11-07 Etappe 2 gewählt (B-736): Der Server läuft komplett in Rust, Go-Server und Brücke `engine/core`
entfallen. Diese Entscheidung legt fest, **womit** der Rust-Server gebaut wird und **in welcher Reihenfolge** portiert wird.
Grundlage ist das Inventar im Anhang (Stand `sprint/rs2`, 9640d992, nach B-731 Stufe 1). Messwerte: 006.

## Kernaussagen (Entwurf)

1. **Laufzeit hybrid:** Räume laufen auf eigenen OS-Threads (ein Thread je Raum, 30 Hz, wie heute eine Goroutine je Raum,
   `engine/room/run.go:60`); Netz und HTTP laufen auf **tokio** (Multi-Thread, 2 Worker). Die Sim kennt kein `async`.
2. **HTTP und WebSocket: axum** (Feature `ws`, darunter hyper und tokio-tungstenite). Rückfall, falls G2 an Speicher oder
   Binärgröße scheitert: blockierend mit `tiny_http` + `tungstenite` (Thread je Verbindung, höchstens 16 Geräte).
3. **Wire-Format byte-gleich zu Go über einen eigenen Go-JSON-Schreiber** (Glossar): `serde`/`serde_json` (schon im
   `Cargo.lock` von `k3c-core`) mit eigenem `Formatter` für Zahlen und Escapes, Zustandsfelder alphabetisch. Kein
   `map[string]any`-Äquivalent; Delta vergleicht typisierte Felder.
4. **Keine Crate für Logging, Fehler, Zeit, Zufall, Statik:** eigenes Modul `conlog` (Emojis, JSON-Zeilen mit denselben
   Feldern wie `slog`, damit `logs_query` weiterliest), eigene Fehler-Enums, eigenes RFC-3339-Format, Raum-Codes aus `rng.rs`
   mit Zeit-Seed, statische Dateien selbst ausgeliefert (Cache-Regeln aus `engine/net/static.go`).
5. **Datei-Watch: `notify`** mit dem Abfrage-Rückfall wie heute (`engine/watch/notify.go`), Entprellen selbst.
6. **Transzendente Mathematik selbst portiert:** `math.Pow` (7×) und `math.Exp` (1×) aus `engine/sim` werden als Rust-Port der
   Go-Algorithmen geschrieben, nicht `f64::powf`/`exp` (die rufen die libm der Plattform: MSVC, glibc, musl rechnen
   verschieden). Golden bitgleich auf amd64 und arm64 hängt daran.
7. **Verzeichnis:** Cargo-Workspace `rust/` mit `k3c-core` (Sim, Daten, Spielstand; rein, ohne IO und ohne tokio) und
   `k3c-server` (Binary: Raum, Netz, Store, Watch, Supervisor, Log). `panic = "unwind"` im Workspace statt `abort` (Raum-Absturz
   schließt nur den Raum, wie `safeTick` heute); `extern "C"` bricht seit Rust 1.81 beim Unwind selbst ab, die C-ABI bleibt sicher.
8. **Pi-Build:** `aarch64-unknown-linux-musl`, statisch, Image bleibt `distroless/static` (B-736 Frage 5, Default musl). Misst
   RS3 einen spürbaren Nachteil des musl-Allokators, Wechsel auf `-gnu` + `distroless/cc`.
9. **HTTPS hinter einem Cargo-Feature `tls`, Standard aus** (rustls zieht `ring` oder `aws-lc-rs` mit C-Code; lokal fehlt der
   C-Compiler im PATH, ungeprüft ob `ring` baut). Ralf entscheidet, ob HTTPS überhaupt noch gebraucht wird.
10. **Komplexitäts-Budget in Rust:** `clippy.toml` mit `too-many-lines-threshold = 60`, `cognitive-complexity-threshold = 15`,
    `excessive-nesting-threshold = 4` (Lints `too_many_lines`, `cognitive_complexity`, `excessive_nesting` auf `deny`), dazu
    Dateilänge ≤ 400 im bestehenden Regeltest (`tests/projectRules.test.ts`) auf `rust/**/*.rs` erweitert.
11. **Umfang korrigiert:** `engine/devrun` und `engine/conlog` bleiben Go (Workbench nutzt sie). Neu im Umfang:
    `engine/balance` + `cmd/k3c-sim` (~8 300 Zeilen, hängen an `engine/sim`) und `studioview` der Workbench (bindet
    `engine/net`, `room`, `store` ein). Ohne sie lässt sich `engine/sim` in RS12 nicht löschen (Anhang G).
12. **Spielstände:** Rust liest und schreibt nur Version 9 (`IslandSaveVersion`); die Migrationen v1–v8 werden nicht portiert
    (Ralf: keine Rücksicht auf alte Stände). Format-Test gegen `testdata/saves/v9` je Seite, solange beide Server laufen.

## Bibliotheken (Kandidaten, je Crate Zustimmung durch Ralf)

Größen sind Erfahrungswerte, **ungeprüft**; RS3 misst Abhängigkeitsbaum (`cargo tree`), Binärgröße (Release, `lto`, `strip`) und
RSS im Leerlauf auf PC und Pi. Heute: Go-Server 15,8 MB EXE, leer 22 MB RSS (006).

| Crate | Zweck | Grund | Größe (ungeprüft) | Pi 3 (1 GB, 4× A53) | Alternativen |
|---|---|---|---|---|---|
| `serde`, `serde_json` | Daten, Spielstände, Protokoll | schon Abhängigkeit von `k3c-core` (Cargo.lock, 16 Pakete); eigener `Formatter` deckt Go-Zahlenformat und Escapes | ~10 Crates, ~300 KB | ja, keine Laufzeit | handgeschriebener Parser (zu teuer), `simd-json` (unnötig) |
| `tokio` (rt-multi-thread, net, time, signal, sync, macros) | Netz-Laufzeit, Signale (Strg+Pause, SIGTERM) | Grundlage von axum; Signale ohne weitere Crate | ~25 Crates, ~600 KB | ja, 2 Worker, wenige MB RSS | Threads + blockierendes IO (Kernaussage 2 Rückfall) |
| `axum` (ws, http1, tokio; ohne Standard-Features) | ~20 Routen `/api/*`, `/ws`, Statik, Shutdown | Routing, WebSocket-Upgrade, Ping/Close-Codes, Graceful Shutdown fertig; spart ~1 500–2 000 Zeilen eigenen Code (3–4 Sessions, ungeprüft) | ~60–80 Crates gesamt mit hyper/tower, ~1,5–2,5 MB | ja (vermutet), RS3 misst | `tiny_http` + `tungstenite` (~15 Crates, < 1 MB, alles Weitere selbst), `hyper` direkt |
| `notify` (Standard-Features ohne `crossbeam`) | Datei-Watch `data/`, `public/` | inotify/ReadDirectoryChangesW wie fsnotify; Abfrage-Rückfall bleibt eigener Code | ~10 Crates, ~200 KB | ja (inotify) | nur Abfragen alle 30 s (006: Abfragen kostete 150 ms je Lauf mit Map-Neubau; mit `stat` ohne Neubau vermutlich billig) |
| `windows-sys` (Threading) | Priorität „über normal“, EcoQoS aus (`cmd/k3c-server/priority_windows.go`) | ohne sie fehlt der Takt-Fix aus RS1.3 (p99 75 → 36 ms); kommt über tokio/notify ohnehin mit (ungeprüft) | nur Bindings, ~0 KB Code | entfällt (nur Windows) | `windows` (größer) |
| `rustls` + `tokio-rustls` (nur Feature `tls`) | HTTPS-Port, falls die Xbox ihn braucht | Go-Server bietet HTTPS optional (`main.go:207`) | ~20 Crates, ~2 MB, C-Code in `ring` | ja | HTTPS weglassen (Empfehlung, wenn ungenutzt), Reverse-Proxy vor dem Container |

Ausdrücklich **nicht** vorgeschlagen: `tracing`/`tracing-subscriber` (eigenes `conlog`, ~200 Zeilen, Format muss zu
`k3c-server.jsonl` passen), `anyhow`/`thiserror` (eigene Enums; `thiserror` wäre billig, da `syn` schon im Baum ist),
`chrono`/`time`/`jiff` (RFC 3339 + Sicherungs-Stempel selbst), `rand` (Regel: nur `rng.rs` würfelt), `tower-http`
(Statik selbst), `mimalloc` (erst, wenn RS3 den musl-Allokator als Engpass misst), `schemars` (kein Schema, B-736 Frage 6).

## Verzeichnis und Build

```
rust/
  Cargo.toml          Workspace: k3c-core, k3c-server; profile.release: lto, codegen-units 1, panic "unwind"
  clippy.toml         Komplexitäts-Budget (Kernaussage 10)
  k3c-core/           lib (bis RS12 auch cdylib für engine/core): rng, level, coins + data, sim, save, gomath, gojson
  k3c-server/         bin k3c-server: room, net, store, watch, supervise, conlog, static, priority
```

- `task rust:server:build` → `bin/k3c-server-rs.exe` (fester Pfad, Firewall); bis RS12 parallel zu `bin/k3c-server.exe`,
  eigener Port (Vorschlag 8070), damit Go-Orakel und Rust-Server gleichzeitig laufen.
- Linux/arm64 lokal nur `cargo check`; Linken in der CI (Job `rust`) und im Docker-Build. Docker: Build-Stufe `rust:1.99-alpine`
  mit `--target aarch64-unknown-linux-musl` (Cross-Linker in buildx, ungeprüft; Alternative QEMU, langsam). Der Go-Build bleibt
  für `k3c-tui` im Image.
- Flags wie heute: `-health` (HEALTHCHECK im distroless-Image), `-check-data` (Neustart-Prüfung), `-dev` entfällt (Dev über
  `engine/devrun`, das `cargo build` als Kommando bekommt, ungeprüft ob devrun das schon kann).

## Portreihenfolge und Orakel

Jede Etappe endet mit grünem Orakel; kein Golden-Neuschrieb zugunsten von Rust (B-736). Sessions aus B-736, mit Korrektur.

| Sprint | Inhalt | Orakel | Sessions B-736 | Einschätzung RS2.4 |
|---|---|---|---|---|
| RS3 | Trace-Golden aus dem Go-Server (`testdata/trace/`), Go-JSON-Schreiber, Skelett: HTTP, `/ws`, `hello`/`welcome`/`rooms` gegen den TS-Client; Spike Tick + Snapshot aus Rust-Typen (Zustand aus einem Go-Snapshot geladen) auf PC und Pi (G2) | 43 Beispiele `testdata/protocol/`, Trace-Handschlag | 5–6 | 5–6 |
| RS4 | Daten (`data/*.json`, 18 Dateien + Biome + Welten) typisiert, `gomath`, Welt aus Level, Tag/Nacht | `rng.json`, `level-*.json` (schon grün), `sim-forest-tag`, `sim-forest-ohne-spieler` | 20–26 (RS4–RS7) | 20–26 |
| RS5 | Monarch, Bewegung, Münzen, Skills | `sim-forest-monarch` | | |
| RS6 | Gegner, Kampf, Wellen, Bosse, Truppen | `sim-*-welle`, `-nacht`, `-sturm`, `-raub`, `-belagerung`, `-aggression` | | |
| RS7 | Wirtschaft, Bau, Bürger, Stufen, Insel, Spielstand v9 | `sim-*-aufbau`, `campaign-*`, `testdata/saves/v9` | | |
| RS7b (neu) | `engine/balance` + `cmd/k3c-sim` nach Rust (Bots, Bericht, Varianten) | Bericht gleich zum Go-`k3c-sim` (feste Seeds), `testdata/balance/` | – | 10–14 (ungeprüft) |
| RS8 | Raum: Takt, Geräte, Pause, Stufen- und Inselwechsel, Speichern außerhalb der Sperre | Trace-Golden ohne Netz (Raum-API) | 6–8 | 6–8 |
| RS9 | Netz: WebSocket, Warteschlange je Gerät, Snapshot/Delta je Gerät, Status-API | Trace-Golden über `/ws` byte-gleich | 8–10 | 8–10 |
| RS10 | Store (atomar, Sicherungen, Berichte, `restart-rooms.state`), Watch, Supervisor (Exit 75) | Format-Tests, Neustart-Tests wie `cmd/k3c-server/*_test.go` | 5–7 | 4–6 (devrun bleibt Go) |
| RS11 | Server-`cmd`, Lasttest gegen beide Server, Parallelbetrieb; `studioview` startet den Rust-Server als Kind; `k3c-load` ohne Import von `engine/net` | `k3c-load`-Bericht beider Server | 4–5 | 5–6 |
| RS12 | Cutover, Docker, CI, Pi-Abnahme (Mensch) | Pi-Tick p99 (G4) | 3–4 | 3–4 |
| **Summe** | | | **57–72** | **66–86** (±30 %, ungeprüft) |

Prüfung der Schätzung gegen 006: `engine/sim` hat 10 375 Zeilen Code (ohne Tests); 20–26 Sessions sind 400–520 Zeilen je
Session. RS1.5 portierte `level` + `rng` (~1 800 Zeilen Go) in **einer** Session mit Golden-Orakel; reine Funktionen gehen also
3–4× schneller als angesetzt. Die Sim ist aber zustandsreich (50 Felder in `World`, globale Daten, `Event` als `map[string]any`),
deshalb bleibt die Schätzung stehen. Die Lücke liegt nicht bei der Sim, sondern im fehlenden Umfang: Balancing und
`studioview` (+11–15 Sessions). G3 (RS4–RS7 > 36 Sessions) bleibt unverändert; RS7b bekommt ein eigenes Budget.

## Risiken

| Risiko | Beleg | Umgang |
|---|---|---|
| Wire-Bytes: Go schreibt `map[string]any` mit sortierten Schlüsseln, Zahlen als `1` statt `1.0`, ab 1e21 und unter 1e-6 mit Exponent (`1e-7`, `1e+21`), escaped `<`, `>`, `&`, U+2028/2029 | `engine/net/state.go` baut `map[string]any`, kein `SetEscapeHTML(false)` im Code | Go-JSON-Schreiber mit Tests gegen `encoding/json` (Tabellentest Zahlen und Texte, RS3) |
| Fließkomma über Plattformen: `f64::powf`/`exp` hängen an der libm | `math.Pow` 7×, `math.Exp` 1× in `engine/sim`; Go rechnet sie in Go | `gomath` portiert Go-Algorithmen, Golden auf amd64 und arm64 |
| `math.Max`/`Min` mit NaN, Float→Int außerhalb des Bereichs | Go: NaN gewinnt, Umwandlung undefiniert; Rust: NaN verliert, Umwandlung sättigt | Hilfsfunktionen `go_max`/`go_min`; Golden fängt Abweichungen |
| Daten-Structs ohne JSON-Tags: Go ordnet Schlüssel ohne Groß-/Kleinschreibung zu | `EnemyData`, `BuildingData`, `HubData` … ohne Tags (`engine/sim/data.go`) | serde mit `rename_all = "camelCase"` je Struct, Test: jede Datei aus `data/` lädt ohne unbekannte Felder |
| Unstabile Sortierung | `slices.SortFunc` 5× in `room`/`net` (nicht in `sim`) | bei Gleichstand Schlüssel ergänzen, sonst Trace-Abweichung |
| Panic beendet den ganzen Server | `profile.release` hat `panic = "abort"` (`rust/Cargo.toml`) | auf `unwind`, `catch_unwind` je Raum-Tick und je Schreib-Task |
| Parallelphase: Regeländerungen in Go müssen nachgezogen werden | B-736 Frage 4 | jede SIM-Session nennt die Rust-Nachzieh-Session; Treiber für G3 |
| Lokale Werkzeuge: kein C-Compiler im PATH | `cl.exe` nicht gefunden, Host `x86_64-pc-windows-msvc` | keine Crate mit C-Code außer hinter `tls`; arm64 nur CI |

## Offene Entscheidungen für Ralf

1. **Laufzeit und HTTP:** tokio + axum (Empfehlung) oder blockierende Threads mit `tiny_http` + `tungstenite`? RS3 misst beides
   nur, wenn Ralf das will; sonst nur axum mit Abbruch an G2. *(B-736 Frage 3)*
2. **Crate-Liste freigeben:** `tokio`, `axum`, `notify`, `windows-sys` neu; `serde`, `serde_json` schon da. Einzeln
   zustimmen oder streichen.
3. **HTTPS:** Wird der HTTPS-Port (Zertifikate in `K3C_CERTS_DIR`) noch gebraucht? Default: nein, Feature `tls` bleibt aus.
4. **Docker:** musl statisch + `distroless/static` (Default) oder `-gnu` + `distroless/cc`? Entscheid nach RS3-Messung. *(B-736 Frage 5)*
5. **Balancing in Rust (RS7b, 10–14 Sessions):** portieren, oder `k3c-sim` bleibt Go mit eigener Go-Sim (dann bleibt `engine/sim`
   und jede Regel wird doppelt gepflegt)? Default: portieren, sonst ist AC-08 (Go-Sim löschen) nicht erreichbar.
6. **Umfang von B-736 anpassen:** `devrun`, `conlog` bleiben Go; `balance`, `k3c-sim`, `studioview` kommen dazu. Summe 66–86
   Sessions statt 57–72.
7. **Spielstand nur v9:** Migrationen v1–v8 nicht portieren? Default: ja (keine Rücksicht auf alte Stände).
8. **Logging ohne `tracing`:** eigenes `conlog` mit identischem JSON-Format (Default) oder `tracing` mit eigenem Formatter?
9. **Schema:** weiter kein JSON-Schema, Trace-Golden ist das Orakel. *(B-736 Frage 6, Default bestätigt)*
10. **Rust-Lints:** clippy-Schwellen aus Kernaussage 10 plus Dateilängen-Test. *(B-736 Frage 7, Default konkretisiert)*

---

## Anhang · Inventar des Go-Servers (Stand 9640d992)

Zeilen ohne Tests (Code / Tests): `engine/sim` 10 375 / 15 279 (83 Dateien), `room` 3 231 / 4 348, `net` 2 711 / 5 444,
`store` 689 / 694, `watch` 217 / 344, `conlog` 187, `devrun` 644, `level` 1 738, `rng` 85, `core` 431, `balance` 7 239 / 3 441,
`cmd/k3c-server` 1 091 / 702, `cmd/k3c-sim` 1 028 / 409. Rust heute: `k3c-core` 2 580 Zeilen (rng, level, coins, FFI).
(Die Zahlen im Kontext der Session, z. B. „room ~7 600“, enthalten Tests.)

### A · Zustand `sim.World` und Teilstrukturen (`engine/sim/types.go`, sofern nicht anders genannt)

Felder gezählt aus den Struct-Definitionen; „JSON“ = Felder mit `json`-Tag außer `"-"`. Der Zustand im Protokoll entsteht seit
RS2.3 von Hand in `engine/net/state.go` (`worldState`, `playerState` …), die Felder dort sind die Soll-Liste für den Port.

| Struct | Felder | JSON | `-` / ohne Tag | Herkunft, Bemerkung |
|---|---:|---:|---:|---|
| `World` | 50 | 33 | 4 / 13 | eine Stufe; ohne Tag u. a. `rng`, `planned`, `hubSite`, `vals`, `island`, `guardsCleared` (Map), `lastWaveAt` |
| `Island` (`island.go`) | 26 | 0 | 0 / 26 | Insel mit `Stages []*World`, Vorrat, Optionen; nur Laufzeit, Spielstand über `IslandSave` |
| `Player` | 28 | 24 | 1 / 3 | `Protected` nur Laufzeit; Skills, Slots, Cooldowns |
| `Enemy` | 25 | 25 | 0 | Bosse, Status-Effekte (`stun`, `slow`, `taunt…`) |
| `Site` | 26 | 21 | 0 / 5 | Bauplätze, Angebote |
| `Troop` | 16 | 13 | 0 / 3 | mit `Job` (7, 6 JSON) |
| `ResourceNode` | 9 | 8 | 0 / 1 | Bäume, Felsen, Adern |
| `Projectile` | 8 | 7 | 1 | |
| `Hazard`, `Storm` | 6 / 5 | 4 / 5 | 1 / 0 | Boss-Flächen, Gewitter |
| `Coin`, `Castle`, `Travel`, `Stock`, `Pickup`, `Camp`, `QueuedSpawn` | 4, 4, 4, 5, 3, 2, 4 | 4, 4, 4, 5, 3, 2, 3 | 0 | |
| `Merchant` (`merchant.go`), `Drop` (`unit_warrior.go`) | 6 / 4 | 6 / 4 | 0 | |
| `CycleInfo` (`world_cycle.go`) | 4 | 4 | 0 | Tag, Phase |
| `ActiveEvent`, `SwitchState`, `WarnZone` (`world_state_mirror.go`) | 2 / 3 / 3 | alle | 0 | Spiegel für das Protokoll |
| `Economy`, `HubUpgrade` (`economy_view.go`) | 8 / 5 | alle | 0 | `sim.EconomyOf`, in den Zustand gemischt |
| `Event` | `map[string]any` | – | – | Ereignisse je Tick; in Rust ein Enum mit eigener Serialisierung |
| `WorldValues`, `economyData`, `Options` | 9 / 12 / 6 | 0 | – | Werte der Welt, nur Laufzeit |

Globale Daten: `engine/sim/data.go:141–149` lädt Paket-Variablen beim Start (`buildings`, `troops`, `economy`, `hub`, `monarch`,
`enemyData`, `waves`, `biomes`, `goals`); `UseData` ersetzt sie nur im Balancing (`engine/balance/vary.go:148`). Rust: `Arc<Data>`
statt Globale.

### B · Protokoll-Nachrichten (v11, `engine/net/protocol.go:14`, `docs/protocol.md` › Nachrichten)

Version nur im Handschlag (`hello.v`, `welcome.v`); falsche Version → `error` `version`, Verbindung zu (`ws.go:282`).
Beispiele: 43 Dateien in `testdata/protocol/`. Befund: `docs/protocol.md` nennt in der Tabelle noch `v` = 9, Code und Kopf sagen 11.

| Richtung | Nachrichten (`t`) | Felder (Auszug) |
|---|---|---|
| Gerät → Server | `hello`, `create`, `join`, `addSlot`, `removeSlot`, `input`, `learn`, `respec`, `leave`, `pause`, `resume`, `dev` (Aktionen `gold`, `material`, `timescale`, `pause`, `castleFall`, `boss`, `bossPhase`, `event`, `night`, `grade`) | `input`: `seq`, `p[]` (`slot`, `moveX`, `sprint`, `pay`, `attack`, `skill`); Grenze 16 KiB je Nachricht (`ws.go:28`) |
| Server → Gerät | `welcome`, `rooms`, `joined`, `level`, `snap`, `delta`, `seats`, `error` (13 Codes, `protocol.md` › Fehler-Codes) | `snap`/`delta`: `stage`, `tick`, `ack`, `s`; Delta mit `set`/`del` für 9 Listen mit `id` und `unset` (`delta.go:15`) |
| HTTP | `/api/health`, `save`, `saves`, `worlds`, `save/backups`, `save/restore`, `status`, `status/log`, `status/disconnect`, `status/save`, `metrics`, `report`, `level`, `clientlog`, `dev`, `dev/fullsave`, `room/close`, `/ws`, Statik `/` | `handler.go:66–94`; Origin-Prüfung für schreibende `/api/` (`origin.go`); `/api/status` mit `K3C_STATUS_TOKEN` |

### C · Spielstand-Format (`engine/sim/island_save.go`, `save.go`, `engine/store`)

| Teil | Inhalt | Port |
|---|---|---|
| `IslandSave` v9 | 19 Felder: `version`, `campaignId`, `world`, `islandId`, `savedAt`, `day`, `phase`, `seed`, `time`, `stock`, `options`, `stages[]` (`HubSave`, 18 Felder), `skillPool`, `players[]` (`IslandPlayerSave`, 6), `defeatedBosses`, `goldCollected`, `won`, `merchantVisits`, `merchant` | 1:1, Feldreihenfolge = Struct-Reihenfolge |
| Migrationen | v1 (Campaign, `SaveGame`), v2–v8 (`island_save_v3.go`, `_v5`, `_v8`, ~250 Zeilen) | nicht portieren (Entscheidung 7) |
| Ablage | `saves/<slot>.json` (Slot `^[a-z0-9-]{1,32}$`), Sicherung `<slot>-<savedAt>.json` bei anderem Spiel, rotierende `saves/backups/<slot>/<stempel>.json`, `restart-rooms.state` | atomar: Temp-Datei + Rename mit Wiederholung unter Windows (`atomic.go:24–93`) |
| Berichte | `reports/gamepad-*.json`, `session-*.json`, nie überschreiben, begrenzt (`reports.go`) | 1:1 |
| Fixtures | `testdata/saves/v1`–`v9` | Rust: nur `v9` |

### D · Daten-Schemas (`data/`, eingebettet per `go:embed *.json biomes/*.json worlds/standard.json`, `data/embed.go:15`)

Kein JSON-Schema vorhanden (nur `tools/paper/schema/` für Grafik). Typen sind die Go-Structs.

| Datei | Go-Typ / Leser | Bemerkung |
|---|---|---|
| `buildings.json` + `ship.json` | `map[string]BuildingData` (14 Felder) + `shipFile`, `data.go:141` | ohne Tags |
| `troops.json` | `map[string]TroopData` (5) | ohne Tags |
| `economy.json` | `economyData` (12, `world_values.go`) | |
| `hub.json` | `HubData` (9), dazu `level` (Plätze, Linien) | Kandidaten-Hub gleiche Form |
| `monarch.json` | `monarchData` (10), `SkillData`, `skillEffect` | |
| `enemies.json`, `bosses.json` | `EnemyData` (10), `BossData` (9) | Pool-Prüfung gegen Biome (`data.go:229`) |
| `waves.json`, `goals.json`, `difficulty.json`, `island-sizes.json`, `events.json`, `light.json` | `wavesData`, `Goals`, `Grade`, `IslandSize`, `NightEvent`, Fackeln | Prüfungen mit deutschem Fehlertext |
| `guards.json`, `ruins.json`, `biomes/*.json` | `engine/level` | in Rust schon portiert (`k3c-core/src/level`) |
| `worlds/*.json` | `rawWorld` (6), `DisallowUnknownFields` (`island_worlds.go:257`) | einzige strenge Datei; `K3C_WORLDS_DIR` im Dev-Mode |
| `paper.json` | `cmd/k3c-server/paper.go` | Zuordnung Grafik, nur durchgereicht |
| nicht vom Server gelesen | `assets*.json`, `asset-prompts.json`, `balance-*.json`, `sprites.json` | Workbench, Balancing |

### E · Nebenläufigkeit (`engine/room`, `engine/net`, `cmd/k3c-server`)

| Was | Wo | Rust-Gegenstück |
|---|---|---|
| Supervisor-Prozess startet Kind (`K3C_CHILD=1`), Signale gepuffert | `main.go:99–101`, `supervise.go:45–190` | `std::process::Command`, tokio-Signal |
| Manager-Schleife: Fristen 1 s, Statistik, Messreihen | `manager.go:60` (`mu`: Raum-Map), `run.go:23–47` | eigener Thread oder tokio-Task |
| Eine Goroutine je Raum, Ticker 30 Hz, Panic schließt nur den Raum | `run.go:60–80`, `room.go:105` (`mu` schützt Insel, Geräte, Monarchen; Tick und Zustandsaufbau unter der Sperre) | OS-Thread je Raum, `Mutex<Room>`, `catch_unwind` |
| Speichern außerhalb der Raum-Sperre | `saver.go:17–112` (`mu`, `WaitGroup`, `wg.Go(writeSaves)`) | Schreib-Thread je Raum, Join beim Flush |
| Je WebSocket: Lese-Handler, Schreib-Goroutine (geweckt über `wake`, Kanal Länge 1), Ping jede Sekunde | `ws.go:52–64` (`qmu` Warteschlange, `mu` Raum, `seq` atomar), `ws.go:244–245`, `ping.go:16` | tokio-Tasks, `Notify` + `Mutex<VecDeque>` |
| Handler: offene Verbindungen, Warte-Gruppe beim Herunterfahren | `handler.go:40–53`, `main.go:221` | `JoinSet`, Frist 3 s |
| Datei-Watch: fsnotify-Goroutine → Kanal Länge 1, Abfrage-Takt | `notify.go:16–32`, `restart.go:105–117` | `notify`-Callback → Kanal |
| Globale Caches | `state.go:355` (`words`, RWMutex), `fullsave.go:29`, `level.go:23` (`sync.Once`), `cpu.go:25`, `monitor.go:134`, `accesslog.go:76`, `conlog.go:45`, `store/saves.go:35` | `OnceLock`, `Mutex`; `words` entfällt mit typisiertem Zustand |

Raum-lokal ist alles unter `Room.mu` (Insel, Stufen, Geräte, Tick-Zähler); über Räume hinweg geteilt sind nur Manager, Monitor,
Store und die Daten (nach dem Laden nur lesend). Mutex-Profil RS1.1: keine Wartezeit an Raum-Sperren (006).

### F · Dateisystem und Umgebung

| Thema | Verhalten heute | Beleg |
|---|---|---|
| Daten | `K3C_DATA_DIR` (Ordner vorhanden → von der Platte, sonst eingebettet) | `data/embed.go` |
| Watch | beobachtet `data/` und `public/`, ignoriert `*.paper.json`, entprellt, Scan bei Ereignis, Frist oder alle 30 s | `engine/watch`, 006 › RS1.3 |
| Neustart | geänderter Stand → Kind `-check-data` → `restarting` an alle Geräte → nach 3 s `room_closed` → Räume merken (`restart-rooms.state`) → Exit 75 → Supervisor startet neu; ungültiger Stand: alter läuft weiter | `cmd/k3c-server/restart.go:29`, `store/restart.go` |
| Umgebung | `K3C_CHILD`, `K3C_DATA_DIR`, `K3C_DIST`, `K3C_PUBLIC_DIR`, `K3C_SAVES_DIR`, `K3C_REPORTS_DIR`, `K3C_WORLDS_DIR`, `K3C_CERTS_DIR`, `K3C_HTTP_PORT`, `K3C_HTTPS_PORT`, `K3C_DEV`, `K3C_STATUS_TOKEN`, `K3C_LOG_DIR`, `K3C_LOG_LEVEL`; nur Go: `K3C_CORE_LIB`, `K3C_DEV_CHILD`, `K3C_DEV_REBUILD`, `K3C_TAKT_TUNE` (Test) | `cmd/k3c-server/main.go:5–83` |
| Logs | Konsole mit Emojis, `k3c-server.jsonl` und `k3c-client.jsonl` unter `K3C_LOG_DIR` | `logging.go` |
| Statik | `dist/` von der Platte, `index.html` `no-cache`, gehashte Assets `immutable` | `static.go:73–109` |

### G · Externe Pakete und Abhängige des Go-Servers

| Go-Paket | Nutzung | Rust-Gegenstück |
|---|---|---|
| `github.com/coder/websocket` v1.8.15 | `/ws`, `Accept` ohne Optionen, Lese-Grenze, Close-Codes | axum `ws` (tokio-tungstenite) |
| `github.com/fsnotify/fsnotify` v1.10.1 | Datei-Ereignisse | `notify` |
| `golang.org/x/sys/windows` | Priorität, EcoQoS | `windows-sys` |
| `github.com/ebitengine/purego` | nur Brücke `engine/core` (Linux) | entfällt mit RS12 |
| Standardbibliothek: `net/http` (+ TLS), `encoding/json`, `log/slog`, `math/rand/v2` (Raum-Codes), `os/exec`, `time` | | axum/rustls, serde + Go-JSON-Schreiber, `conlog`, `rng.rs`, `std::process`, eigenes Zeitformat |

Wer heute an den Engine-Paketen hängt (`go list -deps`): `cmd/k3c-sim` → `engine/balance`, `enginetools`, `sim`;
`cmd/k3c-load` → `engine/net` (nur `ProtocolVersion`, `bots.go:101`), damit auch `room`, `sim`, `store`; Workbench →
`engine/net`, `room`, `store`, `sim` (über `internal/studioview`, „Im Spiel ansehen“), `devrun`, `conlog`. `k3c-tui` hängt an nichts.
