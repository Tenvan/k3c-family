# RU49 · SRV · Smoke-Raum: Welt anlegen, Inselwechsel und Pause im Raum

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

`k3c-room` (RU21) hat Manager, Raum-Takt, Aktionen und Speicher. Drei Teile fehlen für ein erstes Spiel und stehen als Platzhalter in `rust/crates/k3c-room/src/room/pending.rs`: `Manager::create_in` und `load_in` (Go `worlds.go`), `Room::switch_island` (Go `island_switch.go`, `Room::tick` ruft es jeden Takt) und die Pause (Go `pause.go`). Der Rest von RU24 (Stufen je Gerät, Speichern, Vorschau, Metriken, Beobachter) ist für die Smoke-Version nicht nötig.

## Ziel

Ein Manager legt eine neue Insel in der Standard-Welt an und rechnet sie; Inselwechsel und Pause im Raum funktionieren wie in Go. Am Ende sichtbar: `cargo test -p k3c-room` grün, `Manager::create` läuft ohne `unimplemented!`.

## Beteiligte und Zielgruppen

Agenten setzen um (Second Worker, Lane `zweit`), die Review-Session prüft. Teil des kürzesten Wegs zur Smoke-Version (`docs/plan-weiterentwicklung.md` › Weg zur Smoke-Version).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler). Port-Leitfaden `docs/standards/rust-port.md`. Dieser Sprint zieht nur die drei genannten Teile aus RU24 vor; RU24 liefert den Rest und streicht diese Teile aus seinen Sessions (Hinweis in RU24.1 und RU24.4).

## Nicht-Ziele

Stufen je Gerät (`stages.go`), Spielstand speichern (`fullsave`, `saver`), Vorschau, Metriken, Beobachter, Welten-Liste (`WorldList`), Zeitraffer-Dev-Aktionen: bleiben in RU24. Änderungen an Go-Code, Spielstand-Format, `k3c-sim`.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Keine neuen Crates. Sim-Sperre vom 2026-11-11 gilt (nur Tests, keine Sim-Läufe).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:
- **Bereich:** `rust/crates/k3c-room/**`
- **Abhängig von:** RU21
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`create_in("seed", 1, opts)` ohne `opts.world` → Insel der Standard-Welt, `Room` läuft Takte ohne Absturz. Pause eines Geräts → Takt dieses Geräts steht, die anderen laufen weiter (wie `pause_test.go`).

## Ausnahme- und Fehlerfälle

Unbekannte Welt → `Fehler` mit derselben Meldung wie `worldErrorf` in Go. Insel-Wechsel ohne bereites Ziel → `switch_island` liefert `false` und ändert nichts.

## Akzeptanzkriterien

- **AC-01** `Manager::create_in` und `load_in` sind portiert; `worlds_test.go` und `worlds_values_test.go` laufen als Rust-Tests (Tests, die `k3c-sim` nicht tragen, sind mit Grund `ignore`).
- **AC-02** `Room::switch_island` und `Room::pause` samt `couch_paused`, `device_paused`, `pause_tick` sind portiert, `island_switch_test.go` und `pause_test.go` laufen als Rust-Tests.
- **AC-03** Kein `unimplemented!` aus `worlds`, `island_switch` oder `pause` bleibt in `pending.rs`.

## Offene Fragen

keine

Mit Default entschieden (autonom, Claude, 2026-11-18; 🧑 kann widersprechen): Die Welten-Liste (`WorldList`) bleibt in RU24.4, weil das Hauptmenü der Smoke-Version die Standard-Welt startet.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU49.1 | `RU49.1-room-worlds-switch-pause.md` | Umsetzung | autonom | offen |
| RU49.2 | `RU49.2-review.md` | Umsetzung | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
