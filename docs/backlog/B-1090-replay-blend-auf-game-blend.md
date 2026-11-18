# B-1090 · replay/blend.rs auf game/blend.rs umstellen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU26
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU38.2 legte `rust/crates/k3c-game/src/replay/blend.rs` an: ein Port von `blend.go` für die Wiedergabe, der auf einer einzelnen Welt (`World`) und einer Kamera-Liste arbeitet. RU26.2 legt `game/blend.rs` an, der wie Go auf `Frame` und `Cell` arbeitet (mehrere Stufen, Rc<World>, Kamera je Zelle). `glide` und `JUMP_UNITS` sind geteilt (`pub use crate::replay::blend::{JUMP_UNITS, glide}`); die Zustandstypen `Blend` sind zwei verschiedene Kopien der Idee.

## Ziel

Eine einzige Implementierung der Zwischenbilder: die Wiedergabe (`replay/`) nutzt `game::blend::Blend`, `replay/blend.rs` entfällt oder schrumpft auf `glide` und `JUMP_UNITS`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `replay` ruft `Blend::capture/apply/restore` mit Frame und Zellen auf.
- Das Verhalten der Wiedergabe bleibt gleich (Tests `replay/blend_tests.rs` laufen weiter oder gehen in `game/blend_tests.rs` auf).

## Nicht-Ziele

Änderungen am Verhalten der Zwischenbilder.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `cargo test -p k3c-game` grün; `replay/blend.rs` enthält keinen zweiten `Blend` mehr.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Entstanden in RU26.2. `replay/` lag dort nicht im Bereich (läuft parallel in RU38).
