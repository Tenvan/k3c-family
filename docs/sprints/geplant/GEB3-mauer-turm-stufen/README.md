# GEB3 · DEV · Mauer und Turm, Stufen 1 bis 5

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

Mauer und Turm gibt es in fünf Stufen (Holz, Stein, Kupfer, Eisen, Kristall) aus gemischten Packs; `wall:1`, `tower:1` und `tower:5` (Zaubertum) sind Lücken.

## Ziel

Mauer und Turm haben fünf zusammenpassende Stufen aus einer Geometrie mit Material-Paletten. Am Ende sichtbar: Stufenreihe in der Vorschau, von 🧑 abgenommen.

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

- **AC-01** Mauer und Turm liegen in den Stufen 1–5 als Daten-JSON und PNG vor; alle Stufen teilen Geometrie und Maße und unterscheiden sich über Palette und Zusätze.
- **AC-02** Die Slots `wall:1`–`wall:5` und `tower:1`–`tower:5` sind zugeordnet, der Zaubertum (Stufe 5) ist erkennbar.
- **AC-03** 🧑 hat die Stufenreihe abgenommen.

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
