# WZ57 · DEV · Workbench ohne Web-Client, doppelte Ticket-Nummern, Abnahme WZ43

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Stand vor dem Sprint, kurz. Details stehen in den Tickets.

## Ziel

Die Workbench läuft nur als EXE, die doppelten Ticket-Nummern sind aufgelöst, die Abnahme-Läufe WZ43 sind gefahren.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

- **Tickets:** B-872, B-861, B-852
- Reihenfolge: WZ57.1 und WZ57.2 parallel (andere Dateien), WZ57.3 danach, WZ57.4 Review zuletzt. B-872 erst nach Neubau der Workbench-EXE abschließen (Archivieren braucht sonst den Web-Client).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `tools/k3c-workbench/**`, `public/grafik/**`, `tools/paper/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Der Web-Client der Workbench ist entfernt, die Workbench läuft nur als EXE (`B-872/AC-01`–`AC-03`).
- **AC-02** Jede Ticket-Nummer kommt genau einmal vor (`B-861/AC-01`).
- **AC-03** Die Abnahme-Läufe WZ43 sind gefahren (`B-852/AC-01`–`AC-03`).
- **AC-04** Review des Sprints ohne offenen schweren Befund.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ57.1 | `WZ57.1-web-client-entfernen.md` | Umsetzung | autonom | fertig |
| WZ57.2 | `WZ57.2-doppelte-nummern.md` | Umsetzung | autonom | fertig |
| WZ57.3 | `WZ57.3-abnahme-wz43.md` | Umsetzung | autonom | fertig |
| WZ57.4 | `WZ57.4-review.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
