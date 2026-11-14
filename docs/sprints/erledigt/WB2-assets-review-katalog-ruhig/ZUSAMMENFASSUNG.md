# WB2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Review und Katalog: feste Schieber, getrennte Scrollbereiche, Lade-Anzeige
- **Projekt:** PAP

## Ziel

Review und Katalog stehen ruhig: Schieber, Zähler und Knöpfe haben feste Breiten, Liste und Detail haben je einen eigenen Scrollbereich in voller Höhe, der Katalog zeigt beim Laden eine Lade-Anzeige.

## Ergebnis

2026-11-07, autonom (Claude). AC-01 und AC-02 WB2.1, AC-03 bis AC-05 WB2.2 (Tests und Browser-Messung im Worktree, Vite 5196), AC-06 WB2.3 (Review Opus ohne offene wichtige Befunde; Bausteine `FramePlayer`, `LoadingNote`, `ListDetail` über `@webkit`). Letztes `task check:diff` auf Wunsch von Ralf abgebrochen, gezielte Prüfungen grün (siehe WB2.3). Hinweis Ralf umgesetzt: Lade-Anzeige über der Grafik. Zurückgestellt: Review-Befunde 3, 5, 7–10 (klein). Neue Tickets: B-727, B-728, B-729. Sichtprüfung im echten Workbench-Fenster durch Ralf nach dem Merge.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Review: Frame-Schieber, Zähler und Abspiel-Knopf haben feste Breiten, der Zähler nutzt `ta … |
| AC-02 | geprüft | Review: Liste und Detail haben je einen eigenen Scrollbereich in voller Höhe; Scrollen der … |
| AC-03 | geprüft | Katalog: Lade-Anzeige beim Laden der Liste, bei der Wahl eines Assets (auch aus einer ande … |
| AC-04 | geprüft | Katalog: Liste und Detail getrennt scrollbar wie AC-02. |
| AC-05 | geprüft | Katalog: Schieber fest wie AC-01. |
| AC-06 | – | Bausteine über `@webkit` (Molekül, Template), `task check:diff` grün, Review (Opus) ohne o … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
