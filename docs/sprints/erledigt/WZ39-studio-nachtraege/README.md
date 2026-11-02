# WZ39 · DEV · Studio-Nachträge: Downloads, Overrides-Editor, Inspektor, Testlücken, Quellen

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-688, B-693, B-689, B-690, B-691
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-02

## Ausgangslage

Aus WZ37.1, WZ38.1 und dem Team-Lauf WZG offen: Downloads folgen Weiterleitungen auf beliebige Hosts (B-688), Nachträge im Overrides-Editor (B-693), Inspektor ohne eigenes Scrollen (B-689), Testlücken in der Studio-Welt (B-690), veraltete Doku (B-691), Fehlerbericht „Quellen werden nicht übernommen“ (B-630).

## Ziel

Die offenen Befunde aus Studio und Workbench sind behoben oder bewusst dokumentiert. Am Ende sichtbar: neue Go- und Vitests grün, `task check:studio` grün.

## Beteiligte und Zielgruppen

Wer Welten und Assets im Studio pflegt; Agent DEV; 🧑 nimmt die Messung im Browser ab.

## Anforderungen

`B-688 › Anforderungen`, `B-693 › Ausgangslage`, `B-689 › Ziel`, `B-690 › Akzeptanzkriterien`, `B-691 › Akzeptanzkriterien`, `B-630 › Ausgangslage`.

## Nicht-Ziele

Regeln und Simulation des Wertesatzes (BAL10, B-692); neue Funktionen.

## Regeln und Einschränkungen

Oberfläche nur über `@webkit`; Studio-Tests auf Port 5196; keine neue Host-Liste für Downloads.

## Beispiele

Weiterleitung von erlaubtem Host auf `127.0.0.1` → Download bricht ab, keine Datei.

## Ausnahme- und Fehlerfälle

Kind-Welt schon vorher ungültig → Speichern der Eltern-Welt warnt, blockiert nicht.

## Akzeptanzkriterien

- **AC-01** Downloads prüfen jede Weiterleitung gegen die Host-Erlaubnis (`B-688/AC-01`, `AC-02`).
- **AC-02** Die sieben Punkte des Overrides-Editors sind behoben oder dokumentiert (`B-693/AC-01`, `AC-02`).
- **AC-03** Inspektor scrollt in sich, Messung festgehalten (`B-689/AC-01`, `AC-02`).
- **AC-04** Testlücken der Studio-Welt geschlossen (`B-690/AC-01` bis `AC-04`).
- **AC-05** Doku stimmt (`B-691/AC-01`, `AC-02`).
- **AC-06** Ursache von B-630 geklärt und behoben oder als nicht reproduzierbar begründet (`B-630`).

## Offene Fragen

keine. Entschieden (autonom, Claude, 2026-11-02): Ein schon vorher ungültiges Kind blockiert das Speichern der Eltern-Welt nicht mehr, sondern warnt (B-693 Punkt 1).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Entwurf, abgearbeitet am 2026-11-02 in einem Lauf auf `develop` (Auftrag 🧑: „offene Tickets abarbeiten“).

- WZ39.1 (DEV) Downloads und Overrides-Editor (AC-01, AC-02).
- WZ39.2 (DEV) Inspektor, Testlücken, Doku (AC-03, AC-04, AC-05).
- WZ39.3 (DEV) Quellen-Fehler (AC-06).
- WZ39.4 Review, alle Kriterien.

## Abnahme

2026-11-02 (Claude, ein Lauf auf `develop`): AC-01 `TestDownloadHitWeiterleitungAufFremdenHostAbgelehnt`, `TestDownloadHitFolgtWeiterleitung`; AC-02 `TestPlanWorldCheckKindSchonUngueltigWarnt`, `TestPlanWorldKindOhneParseWarnt`, `TestPlanWorldEnkelInChildrenUndPruefung`, `TestPatchOverridesZeilenenden`, `TestFlattenJSONListen`, Vitest „markiert ungültige Nachfolger“; AC-03 Messung in B-689 (+0 px bei 24 Chips); AC-04 `stageLogic.test.ts`, `LivePreviewWorld.test.tsx`, `StageToolbar.test.tsx`, `GitPage.test.tsx`; AC-05 Diff `arbeitsweise.md`, `world/codemap.md`; AC-06 B-630 nicht reproduzierbar (Seite seit B-677 ersetzt), Prüfung durch 🧑 offen, Ticket aus dem Sprint gelöst. Review (Sonnet): keine schweren Befunde, niedrige in B-694. `task check:diff` grün (simrun-TempDir-Flake einzeln grün).
