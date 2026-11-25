# B-1270 · Go-Test TestHubFixtureDesMocksIstDieAntwortDerEngine liest den alten Workbench-Pfad

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU33
- **Projekt:** RSU
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`engine/simcli/hub_test.go` (`TestHubFixtureDesMocksIstDieAntwortDerEngine`) liest `tools/k3c-workbench/frontend/src/api/hubFixture.json`; die Datei liegt seit WZ63 unter `tools/k3c-studio/frontend/src/api/hubFixture.json`. Der Test ist rot (aufgefallen in RU33.2; `task check:go` fasst ihn nicht an). Der Rust-Port liest schon den Studio-Pfad.

## Ziel

Der Go-Test liest den Studio-Pfad und ist grün.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Pfad im Test auf `tools/k3c-studio/frontend/src/api/hubFixture.json` ändern.

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

- **AC-01** `go test ./engine/simcli/ -run TestHubFixture` ist grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
