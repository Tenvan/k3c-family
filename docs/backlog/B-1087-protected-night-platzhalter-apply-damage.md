# B-1087 · protected_night in state/common.rs ruft den Platzhalter mit unimplemented

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

`rust/crates/k3c-sim/src/state/common.rs:182` (`hit_site`) ruft `protected_night(w)` aus `state/pending.rs:72`, das `unimplemented!("RU18: protected_night")` ist. Das echte `protected_night(w, isl)` steht seit RU11.1 in `state/island_ref.rs:82`. Sobald ein Platz auf 0 HP fällt, panict Rust. Fund der Review RU10.6.

## Ziel

`hit_site` nutzt den echten Insel-Bezug; der Platzhalter in `pending.rs` entfällt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `apply_damage_by` und `hit_site` bekommen den `IslandRef` (Option), `protected_night(w, isl)` wird gerufen.
- Kein `unimplemented!` auf dem Pfad.

## Nicht-Ziele

Andere Platzhalter in `pending.rs`.

## Regeln und Einschränkungen

Port-Leitfaden `docs/standards/rust-port.md` § 11.

- **Bereich:** `rust/crates/k3c-sim/src/state/common.rs`, `state/pending.rs`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Platz mit 0 HP in geschützter Nacht: HP bleibt 1.

## Ausnahme- und Fehlerfälle

Ohne Insel (None): Platz wird zerstört.

## Akzeptanzkriterien

- **AC-01** Test: Platz fällt in geschützter Nacht auf HP 1, sonst wird er leer; kein Panic.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
