# MG7.2 · Hintergründe Mine, Eisenstollen und Kristall

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hintergruende
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg7/2-mine-eisen-kristall
- **Abhängig von:** MG7.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Hintergründe Mine (Schachtwand mit Gerüst), Eisenstollen (Schmiedehalle mit Glut) und Kristall (Kristallhalle) als Rig `hintergrund`, je fern, mitte, nah in v1 und v2, gerendert, Golden eingetragen.

## Kontext

Sprint-README und Ergebnis MG7.1 (Generator, Rahmen). Mine: `leder`, `stein`, `holz`, Laternen `gold`; Eisenstollen: `stein`, Glut `koenigsrot` und `gold`; Kristall: `koenigsblau`, `purpur`, Glanz `edelstein`.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `docs/sprints/*/MG7-papier-hintergruende/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine Änderung an Wald und Höhle außer Befund-Fixes.

## Schritte

1. Generator aus MG7.1 erweitern, Probe-Render.
2. `assets_create` und `assets_set` je Asset.
3. `assets_preview` je Asset, `task gen:paper`, fremde Gruppen zurücksetzen, `assets_review`.

## Fertig, wenn

- [ ] 18 Assets gültig und gerendert, jedes per Preview geprüft.
- [ ] Golden und Index der Gruppe aktuell.
- [ ] Ergebnis mit Nachweis je Kriterium.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- 18 Assets in `public/grafik/hintergruende/` (Rig `hintergrund`, 768 × 480, Animation `stand`):
  - `hg-mine-*`: Gesteinsschichten mit Stollenöffnungen und Gerüsten; Stützgestelle mit Laternen, Erzadern und Schiene; dicke Balken mit Brettern, Lore voll Erz, Kisten und Kohle.
  - `hg-eisenstollen-*`: Gewölbe aus vier Bögen mit Trägern und Glutpunkten; Stahlträger mit Flanschen, Streben, Ketten und Glutspalten; genietete Platten mit Rost, Glut am Fuß und Schlacke.
  - `hg-kristall-*`: Höhlenwand mit Kristall-Silhouetten; Kristallgruppen mit Licht- und Schattenfacette, Kristallzapfen und Leuchtadern; große Kristalle in `purpur` und `koenigsblau`, Felsblöcke, Glanz und Splitter.
- Generator aus MG7.1 um `motive2.py` erweitert (Scratchpad).
- **AC-01 geprüft:** `assets_list {gruppe: hintergruende}` 30/30 gültig.
- **AC-02 geprüft:** `task gen:paper` grün, `golden.json` mit 30 Einträgen; fremde Gruppen zurückgesetzt; `task test -- paper` 187/187 grün.
- **AC-03 geprüft:** `assets_preview` je Asset (18 Previews, korrigierte zusätzlich nach dem Fix), dazu ein Kontaktbogen mit allen drei Ebenen je Biom (kacheln ohne Sprung); `assets_review` 18× neu, kein roter Eintrag.
- Befund behoben: Streben und Pfosten lagen in einem Pfad mit gegenläufigem Umlaufsinn, die Füllregel stanzte die Überlappung aus (weiße Kerben in `hg-mine-mitte-*`, `hg-eisenstollen-mitte-*`, kleiner in `hg-mine-fern-*`); Balken haben jetzt denselben Umlaufsinn wie Rechtecke.
- Offen für PS1: Glut im Eisenstollen eher schwach (nur Punkte und kleine Spalten), Rost als ovale Flecken.
- `task check:diff` nach 10 min abgebrochen (Workbench-Go-Tests des feature/pap-Stands); `task test` einzeln 3173/3173 grün, der Diff von MG7 berührt nur `public/grafik/hintergruende/` und Doku.
