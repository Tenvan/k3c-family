# GEB2 · DEV · Gebäude ohne Grafik: Taverne, Heilplatz, Schmiede, Rüstkammer

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-435
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Taverne, Heilplatz, Schmiede und Rüstkammer haben keine Grafik (Slots `tavern`, `healer`, `smithy`, `armory` sind Lücken). Grundregeln und Werkstatt-Vorlage kommen aus GEB1.

## Ziel

Die vier Lücken-Gebäude haben eigene Grafik nach den Grundregeln. Am Ende sichtbar: Vorschau im Asset-Manager im Spielmaßstab, von 🧑 abgenommen.

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

- **AC-01** Taverne, Heilplatz, Schmiede und Rüstkammer liegen als Daten-JSON und PNG unter `public/grafik/gebaeude/` und entsprechen den Regeln aus GEB1.
- **AC-02** Die Slots `tavern`, `healer`, `smithy`, `armory` sind im Register zugeordnet.
- **AC-03** 🧑 hat die vier Gebäude am Bildschirm abgenommen.

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
