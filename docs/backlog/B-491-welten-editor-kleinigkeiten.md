# B-491 · Welten Editor: kleine Nachträge zu Schieber, Klick-Sprung und Planungsliste

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Niedrige Befunde der Reviews zu WZ17.4 (2026-10-18), nicht umgesetzt: (1) `pointercancel` beim Tippen auf die Bühne fehlt. (2) Ein Hin-und-zurück-Ziehen unter 5 px zählt als Klick und springt. (3) Die Einträge der Planungsliste sind nicht per Tastatur anspringbar. (4) Der aktive Listeneintrag der Auswahlliste wird bei langen Listen nicht ins Bild gescrollt. (5) Nach einer Änderung der Fensterbreite wird die Bildmitte erst bei der nächsten Bewegung neu begrenzt (die Anzeige stimmt). (6) Liegen zwei Hinweise (`chunkMarks`) am selben Abschnitt, gewinnt der letzte; bei einer zweiten Quelle fehlt eine Rangfolge. (7) Alt + Mausrad zoomt jetzt auch im Gebäude-Planer (`wheelAction` wird geteilt), ungetestet.

## Ziel

Die genannten Kleinigkeiten sind behoben oder bewusst verworfen, je Punkt mit Test.

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
