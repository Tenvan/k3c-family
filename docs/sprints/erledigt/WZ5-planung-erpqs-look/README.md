# WZ5 · DEV · Planungsseite im Look der erp-qs-Workbench

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-439
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Siehe `B-439 › Ausgangslage`: Die Planungsseite von k3c-dev ist älter als die der erp-qs-Workbench.

## Ziel

Die Planungsseite zeigt und bedient die k3c-Planung im Look der erp-qs-Workbench. Am Ende sichtbar: im Mock (`k3c-dev-ui`, Port 5191) Filterleiste mit Checklisten, Splitter, Projektkarten mit Pills, Detail nur bei Auswahl und Lesemodus, hell und dunkel.

## Beteiligte und Zielgruppen

🧑 nutzt die Seite und gibt die Spec frei; ein Agent setzt um, ein zweiter prüft.

## Anforderungen

`B-439 › Anforderungen`. Sprint-eigen: ein Lese-Binding `PlanningGet(id)` (wie `plan_get`) liefert den Volltext für den Lesemodus.

## Nicht-Ziele

`B-439 › Nicht-Ziele`.

## Regeln und Einschränkungen

`B-439 › Regeln und Einschränkungen`; nur `tools/k3c-dev/` und die Planungsdateien.

## Beispiele

`B-439 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-439 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `B-439/AC-01` Filterleiste.
- **AC-02** `B-439/AC-02` Splitter und Detail.
- **AC-03** `B-439/AC-03` Karten, Panels, Zeilen.
- **AC-04** `B-439/AC-04` Backlog.
- **AC-05** `B-439/AC-05` Lesemodus.
- **AC-06** `B-439/AC-06` Bestehendes bleibt, Prüfungen grün.

## Offene Fragen

`B-439 › Offene Fragen` (nicht blockierend).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ5.1 | `WZ5.1-umsetzung.md` | Umsetzung | autonom | fertig |
| WZ5.2 | `WZ5.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-17, Chat (Ralf): WZ5.1 ist umgesetzt (#306, B-439), WZ5.2 damit abgenommen. Kriterien AC-01–AC-06 laut Ralf erfüllt; keine neuen Tickets.
