# B-1054 · Jede Go-Testdatei aus engine/sim ist in Rust portiert oder begründet ausgelassen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU22
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Session-Tabellen der Sim-Port-Sprints (RU10, RU11, RU14, RU15, RU18, RU19) sind nach Quelldateien geschnitten. Mehrere `_test.go` in `engine/sim/` haben keine gleichnamige Quelldatei; die Planer haben die meisten zugeordnet, ungeklärt blieben u. a. `cause_test.go`, `events_moon_stages_test.go`, `aggression_veins*`, `waves_split`, `pool_lawine`, `merchant_raid_target`, `hub_*`, `site_*`, `monarch_*` (nur teils zugeordnet).

## Ziel

Vor Gate G3 ist jede Go-Testdatei aus `engine/sim/` in Rust portiert oder mit Grund ausgelassen.

## Beteiligte und Zielgruppen

Agenten von RU22 setzen um; Review RU22 prüft.

## Anforderungen

- Liste aller `engine/sim/*_test.go` gegen die Ergebnisse der Sessions RU10–RU19 abgleichen (Ergebnis-Abschnitte).
- Fehlende Tests nach `rust/crates/k3c-sim/tests/` oder ins passende Modul portieren.
- Ausgelassene (Benchmarks, Mess-Tests) mit Grund in `docs/messung/rust-sim-abgleich.md`.

## Nicht-Ziele

Regeländerungen, Golden neu schreiben.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-sim/**`, `docs/messung/rust-sim-abgleich.md`
- **Abhängig von:** RU18, RU19
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant

## Ausnahme- und Fehlerfälle

Ein Test, der auf Go-Interna (Reflection, `testing.AllocsPerRun`) baut, wird ausgelassen und begründet.

## Akzeptanzkriterien

- **AC-01** `docs/messung/rust-sim-abgleich.md` führt jede `engine/sim/*_test.go` als portiert (Rust-Pfad) oder ausgelassen (Grund); keine Zeile fehlt.

## Offene Fragen

keine

## Notizen

Aus der Planung RSU 2026-11-17 (Rückmeldung des Planers zu RU6/RU10/RU11/RU15). Umsetzung in RU22, sinnvoll in RU22.1 neben dem Abgleich-Werkzeug oder im Review.
