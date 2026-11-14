# WZ58 · DEV · Welten-Editor zurück, Wellen-Regel, Größenvergleich im Review

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

## Ausgangslage

Stand vor dem Sprint, kurz. Details stehen in den Tickets.

## Ziel

Die Workbench hat den Welten-Editor wieder (B-879), Wellen planen Sessions eines Sprints nie parallel (B-838), das Review zeigt den Größenvergleich auch vorher (B-853).

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

- **Tickets:** B-879, B-838, B-853

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `tools/k3c-workbench/**`, `docs/arbeitsweise.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Welten-Editor in der Workbench (`B-879/AC-01`, `AC-02`; Sichtprüfung `AC-03` durch 🧑).
- **AC-02** Wellen-Regel in Code und Doku gleich (`B-838`).
- **AC-03** Größenvergleich für das Vorher-Bild (`B-853`).
- **AC-04** Review ohne offenen schweren Befund.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ58.1 | `WZ58.1-welten-editor.md` | Umsetzung | autonom | fertig |
| WZ58.2 | `WZ58.2-wellen-regel.md` | Umsetzung | autonom | fertig |
| WZ58.3 | `WZ58.3-groessenvergleich-vorher.md` | Umsetzung | autonom | fertig |
| WZ58.4 | `WZ58.4-review.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
