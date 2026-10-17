# WZ8 · SIM · Chunks je Biom planen: Seed oder feste Gebäude und Objekte

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-461
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-461 › Ausgangslage: Der Generator reiht Chunks zufällig nach `chunkWeights`; feste Gegenden lassen sich nicht planen. Voraussetzung: Welt-Betrachter aus WZ7.

## Ziel

Chunks eines Bioms lassen sich im Welt-Betrachter planen (Seed oder feste Gebäude/Objekte); der Generator setzt sie deterministisch ein. Am Ende von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `B-461/AC-01` Datenformat für geplante Chunks als Entscheidung 🧑, Glossar.
- **AC-02** `B-461/AC-02` Generator übernimmt geplante Chunks deterministisch (Golden-Test).
- **AC-03** `B-461/AC-03` Bearbeiten im Welt-Betrachter mit sofortiger Vorschau.
- **AC-04** `B-461/AC-04` Ungültige Planung abgelehnt und benannt.
- **AC-05** 🧑 nimmt eine geplante Sondergegend in Vorschau und Spiel ab (Review-Session).

## Offene Fragen

B-461 › Offene Fragen 1–3 (Ort der Planung, feste Gebäude vs. Objekte, Domänen-Schnitt SIM → DEV). Entscheidet 🧑 im Workshop WZ8.1.

## Sessions

- WZ8.1 (SIM) Workshop: Datenformat, Ort der Planung, feste Gebäude oder Objekte (AC-01).
- WZ8.2 (SIM) Generator in `engine/level/` übernimmt geplante Chunks, Validierung, Golden-Test (AC-02, AC-04).
- WZ8.3 (DEV) Editor für geplante Chunks im Welt-Betrachter (AC-03).
- WZ8.4 (DEV) Review und Abnahme durch 🧑 (AC-05, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
