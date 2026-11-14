# WZ39 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Studio-Nachträge: Downloads, Overrides-Editor, Inspektor, Testlücken, Quellen
- **Projekt:** WZG

## Ziel

Die offenen Befunde aus Studio und Workbench sind behoben oder bewusst dokumentiert. Am Ende sichtbar: neue Go- und Vitests grün, `task check:studio` grün.

## Ergebnis

2026-11-02 (Claude, ein Lauf auf `develop`): AC-01 `TestDownloadHitWeiterleitungAufFremdenHostAbgelehnt`, `TestDownloadHitFolgtWeiterleitung`; AC-02 `TestPlanWorldCheckKindSchonUngueltigWarnt`, `TestPlanWorldKindOhneParseWarnt`, `TestPlanWorldEnkelInChildrenUndPruefung`, `TestPatchOverridesZeilenenden`, `TestFlattenJSONListen`, Vitest „markiert ungültige Nachfolger“; AC-03 Messung in B-689 (+0 px bei 24 Chips); AC-04 `stageLogic.test.ts`, `LivePreviewWorld.test.tsx`, `StageToolbar.test.tsx`, `GitPage.test.tsx`; AC-05 Diff `arbeitsweise.md`, `world/codemap.md`; AC-06 B-630 nicht reproduzierbar …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Downloads prüfen jede Weiterleitung gegen die Host-Erlaubnis (`B-688/AC-01`, `AC-02`). |
| AC-02 | – | Die sieben Punkte des Overrides-Editors sind behoben oder dokumentiert (`B-693/AC-01`, `AC … |
| AC-03 | – | Inspektor scrollt in sich, Messung festgehalten (`B-689/AC-01`, `AC-02`). |
| AC-04 | – | Testlücken der Studio-Welt geschlossen (`B-690/AC-01` bis `AC-04`). |
| AC-05 | – | Doku stimmt (`B-691/AC-01`, `AC-02`). |
| AC-06 | – | Ursache von B-630 geklärt und behoben oder als nicht reproduzierbar begründet (`B-630`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-688 | erledigt | AC-01 – | Downloads folgen Weiterleitungen nur zu erlaubten Hosts |
| B-693 | erledigt | AC-02 – | Overrides-Editor: Nachträge aus dem Review WZ37.1 |
| B-689 | erledigt | AC-03 – | Welten-Editor: Inspektor scrollt in sich, Höhe mit 24 Chips messen |
| B-690 | erledigt | AC-04 – | Studio-Welt: Testlücken Klick-Schwelle, Bühnen-Verdrahtung, Werkzeugleiste, Merg … |
| B-691 | erledigt | AC-05 – | Doku: Norton-Ausnahme in arbeitsweise.md, veraltete world/codemap.md |
