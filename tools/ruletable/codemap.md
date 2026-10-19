# tools/ruletable/

## Responsibility
CLI `regeltabelle` (WZ15.2, B-462): schreibt den Block „Hub-Stufe → Gebäude“ aus `data/hub.json` in `docs/rules/materialien-gebaeude.md` (§ 3, zwischen den Marken). Gleiche Daten ergeben byte-gleichen Text.

## Design
- `main.go`: Flag `-root`; `run(root)` liest `data/hub.json` und das Regeldokument, ruft `table.Block` und `table.Ersetzen` und schreibt nur bei einer Änderung, atomar über eine temporäre Datei im Zielordner und `os.Rename`. Rückgabe: geändert oder nicht. Exit 1 bei Fehler.
- Unterpaket `table/`: die Erzeugung selbst (siehe [`table/codemap.md`](table/codemap.md)).

## Flow
1. `data/hub.json` und Dokument lesen → Block erzeugen → Marken im Dokument ersetzen.
2. Gleicher Text: `✅ regeltabelle: Block ist aktuell`; sonst schreiben und `✅ regeltabelle: Block aktualisiert`.

## Integration
- Taskfile-Task `rules:update` (baut `bin/regeltabelle`, startet es).
- Eingabe `data/hub.json`, Ausgabe `docs/rules/materialien-gebaeude.md`.
- Test: `main_test.go`.
