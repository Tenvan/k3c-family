# B-563 · Die Aktionsliste nennt den Bau-Auslöser des Endbosses, das Overlay zeigt ihn

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** KB3
- **Projekt:** KMP
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-132 nennt ein Aktionen-Overlay für den Bau-Auslöser des Endbosses (B-125, S3.3). Die Aktionsliste (`engine/net/actions.go`) nennt dafür keinen Eintrag; K5.1 hat das Overlay deshalb nicht gebaut (Überhang).

## Ziel

Spielende sehen am Bau des Endbosses, dass sie ihn auslösen können.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Server nennt einen Eintrag (z. B. `{ "action": "lair" }`) am Ort des Baus, berechnet von `engine/sim`.
- Client zeigt ihn im Aktionen-Overlay; Protokoll-Session für beide Enden.

## Nicht-Ziele

Grafik des Baus.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test: Spieler am Bau → Eintrag in der Aktionsliste; fern → fehlt.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
