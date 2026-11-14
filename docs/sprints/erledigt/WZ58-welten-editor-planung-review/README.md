# WZ58 · DEV · Welten-Editor zurück, Wellen-Regel, Größenvergleich im Review

- **Status:** erledigt
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
| WZ58.4 | `WZ58.4-review.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14 (WZ58.4, autonom Claude): AC-01 siehe WZ58.1 (B-879 AC-01/AC-02 getestet; **AC-03 Sichtprüfung durch Ralf offen**, B-879 bleibt bis dahin eingeplant), AC-02 siehe WZ58.2 (B-838 erledigt), AC-03 siehe WZ58.3 (B-853 erledigt), AC-04: Sprint-Review (k3c-reviewer) ohne schweren Befund, Akzent der geplanten Chunks dezenter (world.css); leichte Befunde bewusst gelassen (Log-Text bei unverändertem Override, Cache-Schlüssel nach Basis-Name, Wellen-Reihenfolge nur über „Abhängig von“).
Prüfung: `task check:diff` je Session und `task check:dev` grün. Kein Abnahme-Ticket für Sim, Balance und Gen: der Sprint ändert nur Workbench, `k3c-sim world` (ohne Simulation) und Doku, keine Grafik.
