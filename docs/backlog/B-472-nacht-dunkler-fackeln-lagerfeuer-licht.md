# B-472 · Nacht und Morgengrauen sind dunkler, Fackeln und Lagerfeuer machen Licht

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Client zeichnet die Dunkelheit der Nacht über `daylight()` in `src/scenes/viewRules.ts` (Morgengrauen = letzte 10 % der Nacht). Nacht und Morgengrauen sind nach Ralf (2026-10-17, Chat zu `B-460 › Offene Fragen` 13) zu hell: sie sollen deutlich dunkler werden, dafür sollen Fackeln und Lagerfeuer Licht abgeben. Berührt B-460 (Welt-Betrachter, Tageszeit-Auswahl) und B-470 (Morgengrauen als echte Phase). Die Details (Dunkelstufe, Lichtradius, Lichtquellen) sind offen.

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
