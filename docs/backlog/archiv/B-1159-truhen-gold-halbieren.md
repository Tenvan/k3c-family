# B-1159 · Truhen geben nur halb so viel Gold wie nach B-953 (statt zehnmal fünfmal)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU36
- **Projekt:** RSU
- **Erstellt:** 2026-11-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU36.2 (🧑 2026-11-20, G-5): Die Truhen mit Fontaene passen, geben aber zu viel Gold: noch einmal auf die Hälfte reduzieren (zehnmal wird fünfmal). Wert in `data/`; Balancing und Sim-Läufe sind gesperrt, daher nur den Wert ändern.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

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

RU36.3 (🧑 „mit neuen Golden machen“): `chestGold` in `data/economy.json` von [30, 60] auf [15, 30]. Golden neu geschrieben mit `task go:golden` (testdata/golden: campaign-abstieg, campaign-ausbau, sim-* ; Begründung: Truhen-Gold halbiert, B-1159). Tests angepasst: `coop/chest_tests.rs` (15..=30, doppelt 45..=90), `unit/hammer_tests.rs`, Go `unit_hammer_test.go`. Geprüft: `go test ./engine/sim ./engine/level ./engine/room` grün, Rust k3c-sim/room/game/devwin grün bis auf die lokale Altdatei `data/worlds/rom.json` (nicht dieser Diff). Kein Balancing- oder Sim-Lauf. Nachprüfung 🧑: Truhen im Spiel (Station G-5) geben 15 bis 30 Gold, Fontäne unverändert.
