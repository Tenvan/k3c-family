# B-1088 · Insel-Fixture schreibt null für leere Listen, der Takt-Vergleich erwartet leere Listen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU46
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`testdata/rust/insel/insel-1000.json` enthält `"defeatedBosses":null` und `"completed":null` (nil-Slices in Go, `engine/sim/rustref_island_test.go:63,66`). Rust baut leere `Vec` (`[]`) und vergleicht in `island/takt_tests.rs:27,34` mit `golden::diff` exakt; der Vergleich schlägt mit RU14 fehl, ohne echten Sim-Unterschied. Fund der Review RU10.6.

## Ziel

Der Takt-Vergleich unterscheidet nur echte Abweichungen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Entscheidung (Claude, autonom): Rust bildet leere Listen im Vergleich auf `null` ab, die Go-Fixture bleibt unverändert (kein Go-Lauf, keine Neuerzeugung, `testdata/` unberührt). Eine kleine Hilfsfunktion `null_if_empty(&[String]) -> Value` in `rust/crates/k3c-sim/src/island/takt_tests.rs` (`island_fields`) liefert `Value::Null` für leere `defeated_bosses` und `completed`, sonst das Array. Gilt nur, solange Go `nil`-Slices schreibt; ein Kommentar nennt `engine/sim/rustref_island_test.go:63,66`.
- Die Hilfsfunktion bekommt einen eigenen, nicht ignorierten Test (leer -> null, nicht leer -> Array). Der Test `insel_nach_1000_schritten_wie_go` bleibt `#[ignore = "wartet auf RU19 (decode_world_story)"]` (Aufbau der Insel braucht `decode_world_story`); im Ergebnis vermerken, dass die End-zu-Ende-Prüfung mit RU19 folgt.

## Nicht-Ziele

Änderungen an der Sim, am Go-Test oder an der Fixture, Freischalten des ignorierten Takt-Tests (RU19).

## Regeln und Einschränkungen

Fixture und Golden werden nicht neu geschrieben. Entscheidung 012.

- **Bereich:** `rust/crates/k3c-sim/src/island/takt_tests.rs`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Go `nil` → `null`, Rust `[]` → Diff; nach Fix gleich.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `island_fields` bildet leere `defeatedBosses` und `completed` auf `null` ab; Hilfsfunktion mit nicht ignoriertem Test (leer -> null, nicht leer -> Array).
- **AC-02** `cargo test --release -p k3c-sim island::takt` grün (der End-zu-Ende-Test bleibt ignoriert, RU19), `testdata/` unverändert (`git status --short testdata` leer), clippy ohne Warnungen.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
