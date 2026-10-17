# WZ10 · DEV · Monarch-, Gegner- und Boss-Vorschau im Asset-Werkzeug

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-463, B-464
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-463 und B-464 › Ausgangslage: Monarch, 17 Gegnertypen und 5 Bosse sind nur statisch in `figuren.html`/`aufstellung.html` zu sehen; Animationen (Reiten, Schlagen, Skills, Angriff, Tod, Boss-Effekte) nur im Spiel. Voraussetzung: Asset-Werkzeug aus WZ6.

## Ziel

Das Asset-Werkzeug hat eine Monarch-Seite (Reiten, Schlagen, Skills, weitere Zustände) und eine Gegner-Seite (Gegnertypen nach Biom, Bosse), beide mit abspielbaren Animationen im Spielmaßstab und gemeinsamen Abspiel-Bausteinen. Am Ende von 🧑 abgenommen.

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

- **AC-01** `B-463/AC-01`–`AC-04` Monarch-Seite: alle Animationen, Reiten je Reittier, Schlagen und Skills, Spielerfarben, Spielmaßstab.
- **AC-02** `B-464/AC-01`–`AC-04` Gegner-Seite: alle Gegnertypen nach Biom, Bosse mit Effekten, Maßstab, gleicher Aufbau.
- **AC-03** 🧑 nimmt beide Seiten ab (Review-Session).

## Offene Fragen

B-463 › Offene Fragen 1–3 (weitere Zustände, Technik der Abspielung, Ersatz für `figuren.html`/`aufstellung.html`); B-464 › Offene Fragen 1–2 (Boss-Effekte, Gruppierung). Entscheidet 🧑 vor der Freigabe.

## Sessions

- WZ10.1 (DEV) Gemeinsame Abspiel-Bausteine (Animation, Maßstab, Spielerfarbe) (AC-01, AC-02).
- WZ10.2 (DEV) Monarch-Seite: Animationen, Reiten, Schlagen, Skills (AC-01).
- WZ10.3 (DEV) Gegner-Seite: Gegnertypen nach Biom (AC-02).
- WZ10.4 (DEV) Bosse mit Effekten (AC-02).
- WZ10.5 (DEV) Review und Abnahme durch 🧑 (AC-03, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
