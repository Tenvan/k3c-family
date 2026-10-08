# PJ2 · SRV · k3c-dev plant mit Projekten: plan-Tools und Planungsseite

- **Status:** geplant
- **Projekt:** –
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-357, B-358
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach PJ1 stehen Projekt-Ebene, Rang, Domäne je Session und `verworfen` in Regeln, Vorlagen und Planungstest. Die `plan_*`-Tools und die Planungsseite der Workbench (`tools/k3c-dev/`) kennen sie noch nicht. Details: B-357, B-358, B-338.

## Ziel

Projekte werden über MCP angelegt, zugeordnet und nach Rang sortiert, und die Planungsseite zeigt die Arbeit nach Projekten mit verschiebbarem Rang.

**Am Ende sichtbar:** Workbench, Reiter Planung: Projekte nach Rang mit Sprints, Fortschritt und nächster Session; `plan_list kind=projekt` antwortet.

## Beteiligte und Zielgruppen

🧑 priorisiert auf der Planungsseite; Agenten planen über die MCP-Tools; PJ3 nutzt die Tools für den Umzug.

## Anforderungen

- `B-357 › Anforderungen` (Projekte in den `plan_*`-Tools, Rang, Zuordnung, Domäne je Session, keine Sprint-Prio).
- `B-338 › Anforderungen`, SRV-Teil (`plan_set` setzt `verworfen`).
- `B-358 › Anforderungen` (Planungsseite nach Projekten).

## Nicht-Ziele

Regeltexte und Planungstest (PJ1), Umzug der Daten (PJ3), Drag & Drop, Projekte auf der Seite anlegen.

## Regeln und Einschränkungen

- Domäne SRV: nur `tools/k3c-dev/`. Dateiformate kommen aus den Vorlagen von PJ1, das Tool erfindet keine eigenen.
- Hängt von PJ1 ab (auf `develop`).
- Tests offline: Go-Tests auf einer Kopie von `docs/`, Frontend mit Mock-Daten; `dev:test` grün.
- Komplexitäts-Budget: Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Siehe `B-357 › Beispiele` und `B-358 › Beispiele`.

## Ausnahme- und Fehlerfälle

Siehe `B-357 › Ausnahme- und Fehlerfälle` und `B-358 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** B-357/AC-01 (Projekte in `plan_create`, `plan_get`, `plan_list`, `plan_section`).
- **AC-02** B-357/AC-02 (Ränge lückenlos).
- **AC-03** B-357/AC-03 (Zuordnung pflegt Sprint-Tabelle und Übersicht, Planungstest grün).
- **AC-04** B-357/AC-04 (Domäne je Session, Sprint-Domänen abgeleitet, keine Sprint-Prio).
- **AC-05** B-338/AC-02 (`plan_set` setzt `verworfen`).
- **AC-06** B-358/AC-01 (Seite nach Rang mit Sprints und nächster Session).
- **AC-07** B-358/AC-02 (Rang hoch/runter).
- **AC-08** B-358/AC-03 (ruhend/erledigt eingeklappt, ABN, Ohne Projekt).
- **AC-09** B-358/AC-04 (Prompt-Vorschläge nach Rang).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PJ2.1 Projekte in den `plan_*`-Tools: anlegen, lesen, listen, Rang, Zuordnung von Sprints und Tickets (AC-01, AC-02, AC-03).
- PJ2.2 Sessions mit Domäne und `verworfen`, Sprint-Domänen ableiten, Sprint-Prio und `Einschiebbar` entfernen (AC-04, AC-05).
- PJ2.3 Planungsseite nach Projekten, Rang per Knopf, Prompts nach Rang (AC-06, AC-07, AC-08, AC-09).
- PJ2.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
