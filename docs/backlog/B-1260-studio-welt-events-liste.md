# B-1260 · Das Studio setzt die Welt-Events über world_override_set (welt-events)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE8
- **Projekt:** WZG
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`world_override_set` kennt nur die Go-Dateiliste (buildings, economy, enemies, events, goals, hub, monarch, ship, troops, waves). Die Events stehen in `data/welt-events.json` und sind so nicht setzbar. WE8.5 (Sengoku, Nil, Britannien) bleibt ohne Events, bis das gelöst ist.

## Ziel

Das Studio setzt die Welt-Events Überfall, Sandsturm und Nebel über `world_override_set` mit der Datei `welt-events`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

1. `welt-events` ist in der Studio-Liste der überschreibbaren Dateien; Fehlermeldung nennt sie.
2. Setzen von `events › events › raid` (bzw. sandstorm, fog) über das Studio funktioniert und wird in `world_check` geprüft.
3. Test: Override eines Welt-Events wird gespeichert und gelesen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Befund: Die Dateiliste steht in `tools/k3c-studio/internal/mcpsrv/tools_world.go` (`overrideFilesText`) und wird gegen die Go-Engine geprüft (`TestOverrideFilesTextMatchesEngine`). `welt-events` gibt es nur in der Rust-Liste (`override_files`). Die Go-Engine ist eingefroren (Entscheidung 013), darum darf die Liste nicht einseitig um `welt-events` wachsen. Default (Entscheidung ohne Rückfrage): Die Studio-Liste bleibt, bis Entscheidung 013 zur Liste eindeutig ist. Umsetzung erst danach.
