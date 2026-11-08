# CU1.1 · Gerüste und Plantage als Papier-Assets

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** mg
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** PS1
- **Tickets:** B-700
- **Kriterien:** AC-04

## Ziel

Gerüste und Plantage haben Papier-Assets: `geruest-1`, `geruest-2`, `geruest-3`, `geruest-4`, `geruest-6` und `hub-geruest` in `public/grafik/gebaeude-paper/`, `pappel-niedrig` in `public/grafik/objekte/` (D-US2-02). Gerendert, Review-Status gesetzt, `golden.json` der Gruppen aktuell.

## Kontext

- D-US2-02 (B-701): Diese Grafiken sind heute noch alte PNGs aus `tools/buildings` (`public/grafik/gebaeude/geruest-<klasse>-32.png`, `hub-geruest-32.png`, `setzling-32.png`, `pappel-niedrig-32.png`, Maße in den gleichnamigen `.json`). Sie laden über die Gebäude-Zuordnung (`src/scenes/render/buildingSprites.ts` Zeilen 39–57) als Einzelbild.
- Größenklassen: `SCAFFOLD_CLASSES = [1, 2, 3, 4, 6]` (`src/scenes/render/scaffoldView.ts`) = Breite des Gebäudes in Kacheln (32 px); Regeln in `docs/standards/gebaeude-grafik.md` § 1. Das Gerüst der Hub-Burg heißt `hub-geruest` (`hubGrowth.ts` › `HUB_SCAFFOLD`).
- `public/grafik/objekte/setzling.paper.json` existiert schon (MG8) und deckt `setzling` ab; `pappel.paper.json` ist die ausgewachsene Pappel, `pappel-niedrig` fehlt (halbe Höhe, gleicher Stil).
- Rig für Gerüste: `gebaeude` wie die übrigen Assets in `gebaeude-paper` (Vorbild `burg.paper.json`); Material Holz, Seile, Brüstung; Rahmenbreite = Klasse × 32 px.
- Assets nur über MCP `assets_*` mit `checkout`; seit B-707 zieht `assets_set` `golden.json` und Index der Gruppe nach (Workbench-EXE muss B-707 enthalten, sonst `task gen:paper` und fremde Gruppen zurücksetzen). Kein `assets_style_set`.

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/geruest-*.paper.json`, `hub-geruest.paper.json` und ihre Ausgabeordner
- `public/grafik/objekte/pappel-niedrig.paper.json` und Ausgabeordner
- `public/grafik/gebaeude-paper/index.paper.json`, `golden.json`, `public/grafik/objekte/index.paper.json`, `golden.json` (nur über die Tools)
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-Zeilen)

## Nicht-Ziele

Client-Code (CU1.2); alte PNGs löschen (CU1.5); andere Gebäude ändern.

## Schritte

1. `assets_guide` und `assets_style_get` lesen; `burg.paper.json` als Vorbild mit `assets_get` ansehen.
2. `geruest-1` anlegen (`assets_create`), mit `assets_preview` prüfen; daraus `geruest-2/3/4/6` mit passender Breite ableiten.
3. `hub-geruest` in Breite der Hub-Burg (Rahmen von `burg` übernehmen).
4. `pappel-niedrig` aus `pappel` ableiten (halbe Höhe).
5. Je Asset `assets_review` mit Status und kurzer Notiz; offene Formfragen als D-CU1-Zeile in B-701.
6. `check_run {target: task:test, pattern: public}` mit `checkout`.

## Fertig, wenn

- [ ] AC-04: `assets_list {gruppe: gebaeude-paper}` nennt `geruest-1`, `-2`, `-3`, `-4`, `-6`, `hub-geruest`; `assets_list {gruppe: objekte}` nennt `setzling` und `pappel-niedrig`; alle gerendert.
- [ ] `task test -- public` grün (golden.json passt).

## Prüfen

```bash
task check:diff
```

Kein Review-Agent (nur Asset-Specs).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
