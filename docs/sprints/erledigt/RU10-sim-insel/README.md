# RU10 · SIM · Sim: Insel mit Stufen, Takt und Material-Vorrat

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Die Gruppe `island*.go` ist mit 2107 Zeilen in 16 Dateien die größte der Sim.

## Ziel

Eine Insel mit mehreren Stufen rechnet in Rust im selben Takt wie in Go. Am Ende sichtbar: die portierten Insel-Tests sind grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 10 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`. Die Sessions laufen als Kette auf dem Branch `sprint/ru10` (eine nach der anderen, ein Commit je Session).

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/island/`; Dateinamen nach Leitfaden § 2. `island_endboss.go` (71, T) fehlte in der ersten Fassung und kommt in Session 5. Dazu Test-Dateien ohne eigene Quelldatei (Zuordnung in den Sessions): `island_start` (1), `island_save_boss`, `island_save_planned` (2), `island_save_v4`, `island_carry` (3), `island_depths`, `island_switch_etappe`, `island_switch_world` (4), `island_worlds_etappe`, `island_planned`, `island_planned_world` (5).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: island (231, T), island_data (90), island_defeat (40, T), island_nature (80, T) | 441 |
| 2 | `engine/sim/`: island_options (105, T), island_save (284, T) | 389 |
| 3 | `engine/sim/`: island_save_v3 (141, T), island_save_v5 (60), island_save_v8 (42, T), island_sizes (90, T), island_storage (109, T) | 442 |
| 4 | `engine/sim/`: island_switch (102, T), island_targets (125), island_travel (141, T), island_victory (77, T) | 445 |
| 5 | `engine/sim/`: island_worlds (400, T), island_endboss (71, T); Go-Fixture `rustref_island_test.go` | 471 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2188 Zeilen Go (die erste Fassung zählte 2107; `island_endboss` 71 und `island_data` 90, `island_worlds` 400 sind seither dazugekommen oder gewachsen). Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

### Hinweise für die Umsetzung

- Eine Insel rechnet alle Stufen im selben Takt (`engine/sim/island.go`); die Reihenfolge der Stufen ist Teil des Ergebnisses.
- Rust hat keinen Rückverweis `World → Island` (Zustandsmodell, Entscheidung 4): Die Insel reicht jeder Stufe vor ihrem Schritt einen `IslandRef` (RU11.1, `state/island_ref.rs`), der Vorrat wird per `std::mem::swap` getauscht (Entscheidung 5); RU10.1 legt `Island::stage_ref`/`put_stage_ref` an.
- Typen aus späteren Dateien dieses Sprints legt RU10.1 als Rumpf in deren Rust-Datei an (`IslandOptions`, `IslandTravel`, `WorldDef`), die spätere Session füllt sie.
- `CreateWorldWith`/`Step`/`CreateWorld` und die Welt-Werte kommen erst mit RU14; sie stehen bis dahin als Platzhalter in `rust/crates/k3c-sim/src/state/pending.rs`. Tests, die eine Insel bauen oder rechnen, tragen `#[ignore = "wartet auf RU14"]`; RU14 und RU22 lassen sie laufen, RU10.6 listet alle Marken.
- Datenlader für Welten, Inselgrößen, Schwierigkeit, Bosse liegen seit RU3 in `k3c-data` (`world`, `island_sizes`, `difficulty`, `bosses`); sie werden benutzt, nicht neu geschrieben.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: die neue Testdatei `engine/sim/rustref_island_test.go` (RU10.5; Referenz-Zustand der Insel aus Go, schreibt nur mit `K3C_RUSTREF=write`).

**Port-Orakel (Entscheidung 012):** Idiomatisches, performantes Rust ist erwünscht; Hinweise „wie Go“ und „Go-Reihenfolge“ in Sprint und Sessions gelten nicht für Rechenreihenfolge, `k3c-det`-Mathematik, lineare Suche und Go-Datenstrukturen. Verbindlich bleiben Reihenfolge der Ereignisse und der rng-Aufrufe, Ganzzahl-Werte und Determinismus innerhalb von Rust; Fließkomma gilt mit Toleranz 1e-4 relativ (`docs/standards/rust-port.md` § 1).

- **Bereich:** `rust/crates/k3c-sim/src/island/**`, `rust/crates/k3c-sim/src/state/pending.rs` (Platzhalter löschen und anlegen), `rust/crates/k3c-sim/src/state/island_ref.rs` (nur Aufbau aus der Insel), `rust/crates/k3c-sim/src/state/omit.rs` (nur neue Hilfen), `rust/crates/k3c-sim/src/lib.rs` (nur `pub mod island;`), `engine/sim/rustref_island_test.go`, `testdata/rust/insel/**`
- **Abhängig von:** RU6, RU8
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Aufbau und Takt einer Insel liefern nach 1000 Schritten denselben Zustand wie Go: Der Zustand aus Go (`testdata/rust/insel/insel-1000.json`) ist eingecheckt und der Rust-Vergleich ist geschrieben; er läuft ab RU14 (`step`), bis dahin `#[ignore = "wartet auf RU14"]`.
- **AC-02** Stufenwechsel und Material-Vorrat verhalten sich wie in `docs/rules/stufen.md`; die Go-Tests der Gruppe (samt der Test-Dateien ohne Quelldatei) sind portiert, die ohne `World`-Aufbau lauffähigen sind grün, jede Ignore-Marke nennt eine Sprint-Nummer.
- **AC-03** Der Insel-Spielstand (Version 9) wird gelesen und ergibt geschrieben denselben JSON-Baum (`testdata/saves/v9/familie.json`, Test `save_v9_rundlauf`); die Migrationen aus den Versionen 2 bis 8 ergeben dasselbe wie in Go.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **AC-01 (1000 Schritte gegen Go) braucht eine Referenz aus Go.** Kein Sim-Golden läuft über Inseln (die 13 `sim-*.json.gz` und beide `campaign-*.json.gz` rechnen Welten und Kampagnen). Default: Eine neue Go-Testdatei `engine/sim/rustref_island_test.go` schreibt den Zustand nach 1000 Schritten einmalig nach `testdata/rust/insel/insel-1000.json` (nur mit `K3C_RUSTREF=write`, sonst `t.Skip`); das ist ein einzelner Sim-Lauf als Test mit der Ausnahme aus B-1014 (RU10 steht dort namentlich) und die einzige Go-Änderung (RU13.6 geht denselben Weg mit `rustref_test.go`). Der Rust-Vergleich ist bis RU14 (`step`) ignoriert, AC-01 ist deshalb mit RU10 „Fixture eingecheckt, Test vorbereitet“ und wird mit RU14/RU22 grün.
- **Tests ohne `World`-Aufbau:** Alle Tests, die `CreateWorld`/`Step` brauchen, laufen mit `#[ignore = "wartet auf RU14"]`; AC-02 zählt sie als portiert.
- **Insel-Bezug der Stufen:** `IslandRef` als Parameter (RU11.1), Vorrat per `swap`; Spieler wechseln zwischen Stufen als Werte (`remove`/`insert`).
- **Test-Dateien ohne Quelldatei** (`island_start`, `island_save_*`, `island_carry`, `island_depths`, `island_switch_*`, `island_worlds_etappe`, `island_planned*`) kommen in die Session der getesteten Datei; `island_bench_test.go` (nur Benchmarks) wird nicht portiert.
- **Sprint-Schnitt:** `island_endboss.go` neu in Session 5; Session 5 umfasst dadurch 471 Zeilen Go (Plan: 450), liegt unter dem 1,5-Fachen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU10.1 | `RU10.1-island-data-defeat-nature.md` | Umsetzung | autonom | fertig |
| RU10.2 | `RU10.2-island-options-save.md` | Umsetzung | autonom | fertig |
| RU10.3 | `RU10.3-island-save-versionen-sizes-storage.md` | Umsetzung | autonom | fertig |
| RU10.4 | `RU10.4-island-switch-targets-travel-victory.md` | Umsetzung | autonom | fertig |
| RU10.5 | `RU10.5-island-worlds-endboss-takt.md` | Umsetzung | autonom | fertig |
| RU10.6 | `RU10.6-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18 (Review RU10.6, Details dort): AC-01 Fixture eingecheckt, Test ignoriert bis RU14; AC-02 alle 125 Go-Tests portiert, Ignore-Marken RU14/15/18/19; AC-03 Rundlauf v9 und Migrationen grün.
`task check` und `task check:go` grün. Reviewer-Befund HOCH (Vorrat je Stufe, `sync_stock`) behoben.
Neue Tickets: B-1087, B-1088. Abnahme-Ticket für Sim, Balance, Gen, Race: B-1086.
