# B-1088 · Insel-Fixture schreibt null für leere Listen, der Takt-Vergleich erwartet leere Listen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU10
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`testdata/rust/insel/insel-1000.json` enthält `"defeatedBosses":null` und `"completed":null` (nil-Slices in Go, `engine/sim/rustref_island_test.go:63,66`). Rust baut leere `Vec` (`[]`) und vergleicht in `island/takt_tests.rs:27,34` mit `golden::diff` exakt; der Vergleich schlägt mit RU14 fehl, ohne echten Sim-Unterschied. Fund der Review RU10.6.

## Ziel

Der Takt-Vergleich unterscheidet nur echte Abweichungen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Entweder Go-Test schreibt leere Listen als `[]` und die Fixture wird einmal neu erzeugt, oder Rust bildet leer auf `null` ab (`island_fields`).

## Nicht-Ziele

Änderungen an der Sim.

## Regeln und Einschränkungen

Einzige erlaubte Go-Änderung bleibt `rustref_island_test.go`.

- **Bereich:** `engine/sim/rustref_island_test.go`, `testdata/rust/insel/**`, `rust/crates/k3c-sim/src/island/takt_tests.rs`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Go `nil` → `null`, Rust `[]` → Diff; nach Fix gleich.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `insel_nach_1000_schritten_wie_go` zeigt bei Freischaltung (RU14) keinen Diff in `defeatedBosses` und `completed`.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
