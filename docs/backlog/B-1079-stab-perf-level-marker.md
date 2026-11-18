# B-1079 · Level-Generator: STAB- und PERF-Markierungen in k3c-level abbauen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU43
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c-level` (RU6) trägt zwei bewusste Schwachstellen: `STAB(B-1079)` in `level_entities.rs` (Go bricht bei leerer Liste der Verstecke mit `panic` ab, Rust meldet einen Fehler mit neuem Text) und `PERF(B-1079)` in `nature.rs` (`go_float` baut je Entität einen Text für den Variantenstrom; einmal je Level). Der Variantenstrom `<biom>:<seed>:variant:<gruppe>:<go_float(x)>` hängt an bitgleichen Entity-x (Review RU6.6).

## Ziel

Beide Markierungen sind nach Messung entfernt oder begründet unkritisch; die Bitgleichheits-Abhängigkeit der Varianten steht in `docs/standards/rust-port.md`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Erst messen (Allokationen je Level), dann ändern.
- Fehlerweg des leeren Versteck-Pools nach Port-Orakel (Entscheidung 012) festlegen.
- Leitfaden: Ausnahme der Toleranz für Variantenströme nennen.

## Nicht-Ziele

Neue Level-Regeln, Änderung der rng-Reihenfolge.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-level/**`
- **Abhängig von:** RU6
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Kein `STAB(B-1079)` und kein `PERF(B-1079)` mehr im Code, `task rust:lint` und `cargo test -p k3c-level` grün, Golden unverändert.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
