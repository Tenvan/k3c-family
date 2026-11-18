# B-1091 · Importregel der Fachmodule hat fünf bekannte Ausnahmen

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

Der Test `game/imports_tests.rs` (Port der Importregel, B-792) fand in den bereits portierten Fachmodulen Verstöße, die als Liste `KNOWN` im Test stehen: `props` nutzt `crate::actors` (nature, paint, plan, testutil), `hud` nutzt `crate::props::simview` und `props::pending` (mod, hints, state, values, testutil, values_tests), `hud/draw_tests.rs` nutzt `crate::replay::trace`, `menu/image_tests.rs` nutzt `crate::still`, `actors/draw_tests.rs` nutzt `k3c_rig`. In Go gilt: Fachpakete importieren einander nie; der gemeinsame `SimView` liegt dort nicht in `props`.

## Ziel

Keine Ausnahmen mehr in `KNOWN`: gemeinsame Typen (`SimView`, Economy, Figuren-Maße) wandern nach `view` oder `art`, Tests nutzen `art` statt `k3c_rig`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `KNOWN` im Test ist leer und wird gelöscht.
- Das Verhalten bleibt gleich.

## Nicht-Ziele

Neue Regeln für die Fachmodule.

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

- **AC-01** `cargo test -p k3c-game game::imports` grün ohne `KNOWN`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Entstanden in RU26.2. Zusätzlich lässt der Test `k3c_det` (Mathematik, wie Gos `math`) und für `view` `k3c_platform` (Canvas) zu.
