# tools/ruletable/

## Responsibility
CLI `regeltabelle` (WZ15.2, B-462): schreibt den Block „Hub-Stufe → Gebäude“ aus `data/hub.json` in `docs/rules/materialien-gebaeude.md` (§ 3, zwischen den Marken). Gleiche Daten ergeben byte-gleichen Text.

## Design
TypeScript unter bun, nur Node-APIs (RU71); Aufruf `bun tools/ruletable/main.ts [-root <ordner>]`.

- `main.ts` (Einstieg), `app.ts`: Flag `-root`; `run(root)` liest `data/hub.json` und das Regeldokument, ruft `block` und `replaceBlock` und schreibt nur bei einer Änderung, atomar über eine temporäre Datei im Zielordner. Exit 1 bei Fehler, 2 bei falschem Flag.
- `table.ts` (reine Logik ohne Dateizugriff): Marken `START`/`END`; `block(hubJson)` mit einer Zeile je Stufe (Material: das eine Material der Kosten samt Ausbauschritten, Gold zählt nicht, ohne Kosten Holz; Plätze aus `sites` und `islandSites` nach Art gruppiert; Mauer- und Turm-Zone; Händler nur Stufe 1); falsche Feldtypen sind ein Fehler. `replaceBlock` tauscht Start- bis End-Marke, der Rest bleibt byte-gleich, Zeilenenden folgen dem Dokument. `compareBlock` vergleicht ohne zu schreiben.
- Unterpaket `table/` (Go): dieselben Regeln für das Studio (siehe [`table/codemap.md`](table/codemap.md)).

## Flow
1. `data/hub.json` und Dokument lesen → Block erzeugen → Marken im Dokument ersetzen.
2. Gleicher Text: `✅ regeltabelle: Block ist aktuell`; sonst schreiben und `✅ regeltabelle: Block aktualisiert`.

## Integration
- Taskfile-Task `gen:rules` in `tools/Taskfile.yml`.
- Eingabe `data/hub.json`, Ausgabe `docs/rules/materialien-gebaeude.md`.
- Tests: `app.test.ts`, `table.test.ts` (Vitest der Wurzel).
- `main.go` und `main_test.go` nutzt der Task nicht mehr; Löschung in RU71.4.
