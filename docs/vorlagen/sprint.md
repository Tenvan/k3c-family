# SP00 · DOM · Titel des Sprints

- **Status:** geplant | aktiv | erledigt
- **Domäne:** REG | SIM | SRV | CLI | PLAT | INF
- **Reife:** Entwurf | bereit
- **Einschiebbar:** nein | ja
- **Tickets:** B-000, B-000
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/main`)
- **Spec:** Entwurf | freigegeben | rückwirkend
- **Revision:** 1
- **Freigabe:** – (bei `freigegeben`: Datum und Quelle; umfasst die Ticket-Specs in ihrer aktuellen Revision)

## Ausgangslage

Stand vor dem Sprint, kurz. Details stehen in den Tickets.

## Ziel

Ein Satz: Was kann man nach diesem Sprint, was vorher nicht ging? Dazu „Am Ende sichtbar“: was man am TV,
in der CI oder im Terminal sieht.

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

- **AC-01** Prüfbar formuliert, mit Herkunft, falls aus einem Ticket (`B-009/AC-01`).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP00.1 | `SP00.1-kurzname.md` | Umsetzung | autonom | offen |
| SP00.2 | `SP00.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
