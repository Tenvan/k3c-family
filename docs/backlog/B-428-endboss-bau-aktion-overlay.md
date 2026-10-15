# B-428 · Aktionen-Overlay zeigt den Bau-Auslöser des Endbosses

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Überhang aus B-132 (K5.1): B-132 nennt ein Aktionen-Overlay für den Bau-Auslöser des Endbosses (B-125, S3.3). `actions` (`docs/protocol.md` › Skills und Aktionen, `PlayerAction` in `src/model/types.ts`) kennt dafür keinen Eintrag; der Endboss wacht über die Nähe auf (`lair.triggerUnits`). K5.1 hat das Overlay deshalb nicht gebaut.

## Ziel

Entscheiden, ob der Endboss-Bau einen Eintrag in `actions` bekommt (Server, SRV); wenn ja, zeigt das Aktionen-Overlay ihn an (CLI, Folge-Session). Sonst Ticket verwerfen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
