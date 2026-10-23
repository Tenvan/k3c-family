# B-574 · Reiten hat eine Hufschlag-Schleife je Spieler

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** SO7
- **Projekt:** SND
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-558 (S5, KR4.1): Das Reiten bekommt einen Sound. Der Slot `sfx:ride` existiert, es gibt keinen Einbau; eine Schleife je Spieler braucht eine Erweiterung im Audio-Kern.

## Ziel

Reitet ein Spieler, läuft eine Hufschlag-Schleife für genau diesen Spieler.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Schleife je Spieler im Audio-Kern, abgeleitet aus `players` im Snapshot (Client rechnet nichts, `docs/decisions/001`).
- Funktioniert mit zwei Spielern gleichzeitig.
- Kandidat für `sfx:ride` im Asset-Studio wählen (Lizenz CC0 oder CC-BY).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ein reitender Spieler löst die Schleife aus, Absteigen beendet sie; zwei Reiter spielen je eine.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-558 (KR4.1). Spec noch Entwurf.
