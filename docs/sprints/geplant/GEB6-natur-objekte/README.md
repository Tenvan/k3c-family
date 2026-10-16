# GEB6 · DEV · Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-435, B-438
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bäume, Fels, Erz, Busch, Adern, Truhe, Portal, Ausgang und Rekrutierungslager stammen aus gemischten Packs. Drei Bäume sind als Beispiel in GEB1.

## Ziel

Alle Strukturen und Objekte haben einheitliche Grafik mit Varianten je Biom. Am Ende sichtbar: Vorschau je Biom, von 🧑 abgenommen.

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

- **AC-01** Bäume (weitere Varianten), Fels, Erzbrocken, Busch, vier Adern, Truhe, Portal, Ausgang und Rekrutierungslager liegen als Daten-JSON und PNG vor.
- **AC-02** Die Slots `node:*`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` sind zugeordnet.
- **AC-03** 🧑 hat die Objekte abgenommen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
