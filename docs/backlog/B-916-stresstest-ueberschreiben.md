# B-916 · Ein bestehendes Stresstest-Spiel lässt sich beim Start eines neuen einfach überschreiben

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** FB1
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Gibt es schon ein Stresstest-Spiel, lässt sich ein neues nicht ohne Umweg starten.

## Ziel

Der Start eines neuen Stresstests überschreibt den vorhandenen Stresstest-Spielstand ohne Hürde.

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

- **AC-01** Mit vorhandenem Stresstest-Spielstand startet ein neuer Stresstest direkt und ersetzt ihn (Test).
- **AC-02** Normale Spielstände bleiben unberührt.

## Offene Fragen

Rückfrage vor dem Überschreiben ja oder nein? Default: ohne Rückfrage (🧑).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
