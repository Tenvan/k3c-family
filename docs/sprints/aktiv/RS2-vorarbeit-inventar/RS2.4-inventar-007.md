# RS2.4 · Inventar der Engine und Entscheidung 007 (Rust-Server, Entwurf)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/rs2
- **Abhängig von:** RS2.3
- **Tickets:** B-736
- **Kriterien:** AC-04

## Ziel

Ein Inventar der Go-Engine (Zustand, Protokoll, Spielstand, Daten-Schemas, Nebenläufigkeit, Abhängigkeiten) und der Entwurf der Entscheidung 007 (Rust-Server: Architektur und Bibliotheken) liegen vor. Entscheidung über Abhängigkeiten trifft Ralf.

## Kontext

B-736 (Etappe 2), Entscheidung 006 › Varianten und › Gemeinsamer Code. Bestand Go: `engine/sim` ~25 650 Zeilen, `room` ~7 600, `net` ~7 400, `store` ~1 400, `watch` 560, `conlog` 220, `devrun` 1 130, `cmd/k3c-server` ~1 050; externe Pakete `coder/websocket`, `fsnotify`. Rust: `rust/k3c-core` (rng, level, coins; C-ABI), Brücke `engine/core`. Protokoll: `docs/protocol.md`, `engine/net/protocol.go`, `src/online/clientProtocol.ts`. Nach RS2.3 ist der Zustandsaufbau handgeschrieben und damit die beste Feldliste. Dokumentablage: keine weiteren Prozess-Dokumente (`CLAUDE.md`); das Inventar gehört in `docs/decisions/007-rust-server.md` (Anhang) oder `docs/messung/`, nicht in ein neues Prozessdokument.

## Erlaubte Dateien

- `docs/decisions/007-rust-server.md` (neu, Entwurf)
- `docs/decisions/006-rust-engine.md` (Verweis)
- `docs/glossar.md`
- lesend: `engine/**`, `cmd/**`, `docs/protocol.md`, `src/online/**`

## Nicht-Ziele

Kein Code, kein Rust-Server-Skelett (RS3), keine neue Abhängigkeit hinzufügen, keine Freigabe von 007 (Ralf).

## Schritte

1. Inventar als Tabellen in 007 (Anhang): (a) `sim.World` und Teilstrukturen mit Feldanzahl, JSON-Tag, Herkunft; (b) Protokoll-Nachrichten (Client→Server, Server→Client) mit Versionsfeld; (c) Spielstand-Format (`store`, `saves/`); (d) `data/*.json`-Schemas; (e) Goroutinen, Locks, Kanäle in `room`/`net` (was läuft nebenläufig, was ist Raum-lokal); (f) Dateisystem-Interaktion (`watch`, `store`, Supervisor Exit 75, `K3C_*`-Umgebung); (g) alle externen Pakete mit Rust-Gegenstück.
2. Entscheidung 007 (Entwurf): Laufzeit (tokio oder Threads), HTTP/WebSocket-Bibliothek, serde/serde_json, Datei-Watch (`notify`), Logging (`tracing` oder eigenes mit Emojis wie `engine/conlog`), Fehlerbehandlung, Verzeichnisstruktur (`rust/k3c-server`, Workspace mit `k3c-core`), Build für Windows/arm64, Docker (musl vs. `distroless/cc`), Lints für Komplexitäts-Budget in Rust.
3. Je Bibliothek: Grund, Größe (Abhängigkeitsbaum, Binärgröße), Pi-Tauglichkeit (1 GB RAM); Alternativen genannt.
4. Portreihenfolge und Orakel je Etappe (Golden, Trace) skizzieren; Schätzung gegen 006 prüfen (Sessions je Sprint RS3–RS12 aus B-736).
5. Neue Begriffe (z. B. Orakel, Trace-Golden, Rust-Server) in `docs/glossar.md` zuerst eintragen.
6. Offene Entscheidungen für Ralf als Liste am Ende von 007.

## Fertig, wenn

- [x] AC-04: 007 (Entwurf) enthält alle sieben Inventar-Tabellen und je Rust-Abhängigkeit Grund und Pi-Tauglichkeit.
- [x] AC-04: Offene Entscheidungen für Ralf stehen als Liste in 007 und in B-736 › Offene Fragen (abgeglichen).
- [x] CI-Voraussetzung: `task check:diff` grün (`tests/planning.test.ts`, Glossar).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).

## Ergebnis

2026-11-08, autonom (Claude), nur Doku. `docs/decisions/007-rust-server.md` angelegt (Status **Entwurf**, nicht freigegeben), Verweis in 006 › Folgen, Glossar-Eintrag „Go-JSON-Schreiber“ (Orakel, Trace-Golden, Rust-Server, Gate standen schon drin).

- **AC-04 geprüft (Inventar):** Anhang A–G in 007: (A) `sim.World` (50 Felder, 33 JSON) und 20 Teilstrukturen mit Feldanzahl, JSON-Tags, Herkunft; (B) Protokoll v11, 12 Nachrichten Gerät → Server, 8 Server → Gerät, 19 HTTP-Routen; (C) Spielstand v9 und Ablage `store`; (D) `data/*.json` mit Go-Typ und Leser; (E) Goroutinen, Sperren, Kanäle mit Rust-Gegenstück; (F) Dateisystem, Neustart Exit 75, 14 `K3C_*`-Variablen; (G) externe Pakete mit Rust-Gegenstück und alle Abhängigen der Engine-Pakete.
- **AC-04 geprüft (Bibliotheken):** Tabelle je Crate mit Zweck, Grund, Größe (ungeprüft, Messung RS3), Pi-Tauglichkeit, Alternativen; Liste der bewusst nicht vorgeschlagenen Crates.
- **AC-04 geprüft (offene Entscheidungen):** 10 Punkte am Ende von 007, abgeglichen mit `B-736 › Offene Fragen` (dort 13 Punkte mit Verweis auf die 007-Nummer).
- **CI-Voraussetzung:** `check_run task:test` (Muster `planning`) und `task:check:diff` grün (siehe Commit).

Abweichungen und Befunde für Ralf:

- **Umfang von B-736 zu klein:** `engine/balance` + `cmd/k3c-sim` (~8 300 Zeilen) und `studioview` der Workbench hängen an `engine/sim`/`net`/`room`; ohne Port ist AC-08 (Go-Engine löschen) nicht erreichbar. `devrun` und `conlog` nutzt die Workbench, sie bleiben Go. Schätzung 66–86 statt 57–72 Sessions (ungeprüft).
- **Byte-Gleichheit braucht einen eigenen Go-JSON-Schreiber** (sortierte Map-Schlüssel, Go-Zahlenformat, HTML-Escapes) und eine Portierung von `math.Pow`/`math.Exp` (libm der Plattform rechnet anders).
- `rust/Cargo.toml` hat `panic = "abort"`; für den Server muss das auf `unwind` (Raum-Absturz schließt nur den Raum).
- `docs/protocol.md` nennt in der Nachrichten-Tabelle noch `v` = 9, Code und Kopf sagen 11 (nicht geändert, Datei außerhalb der erlaubten Dateien).
- Keine neuen Tickets; die Befunde stehen als offene Fragen in B-736 und 007.
