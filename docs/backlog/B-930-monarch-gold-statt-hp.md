# B-930 · Der Monarch hat keine HP: jeder Treffer kostet einen festen Goldbetrag, ohne Gold fällt die Krone

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** KS3
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Monarch: HP 100, Verteidigung 5 (`data/monarch.json:3`), `damagePlayer` mit Schild und Last Stand (`engine/sim/common.go:156-182`), Respawn und Wiederbeleben (`monarch_revive.go`). Geldsack-Größen aus B-922.

## Ziel

Keine HP mehr: jeder Treffer kostet einen festen Goldbetrag (Wert in `data/monarch.json`), Münzen fliegen sichtbar aus dem Sack. Ist kein Gold mehr da, fällt die Krone (B-931). Der Geldsack wird zum Puffer.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `engine/room/**`, `data/**`
- **Abhängig von:** B-926
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Treffer zieht den festen Betrag ab, Münzen fallen zu Boden (Test).
- **AC-02** Treffer ohne Gold lässt die Krone fallen (Test).
- **AC-03** HP, Respawn, Wiederbeleben und Last Stand sind entfernt oder umgedeutet; Skills mit HP-Bezug angepasst.

## Offene Fragen

Entschieden 🧑 2026-11-15: 1 Münze je Treffer, fällt aufsammelbar zu Boden; auch Gegner können dieses Gold rauben (wie der Dieb, ins Portal). Skills umdeuten: Schild fängt Treffer ohne Goldverlust ab, Heilung entfällt, Last Stand = letzte Münze hält einen Treffer länger.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
