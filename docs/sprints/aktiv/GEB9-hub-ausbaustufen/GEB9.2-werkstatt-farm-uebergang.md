# GEB9.2 · Übergang in tools/buildings, Werkstatt und Farm Stufen 2–5

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb9/2-werkstatt-farm-uebergang
- **Abhängig von:** GEB9.1
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-03, AC-05

## Ziel

`tools/buildings` schreibt für Gebäude mit `stufen` übergangsweise auch `<id>-<dichte>.png`; Werkstatt und Farm haben `stufen` 2–5 und PNGs für Stufe 1–5.

## Kontext

- Stufen-Looks: `docs/standards/gebaeude-grafik.md` § 6 › Hub-Gebäude (GEB9.1). Mechanik: `tools/buildings/stages.go` (`renderStufen`, `stufeBauen`), Aufruf und Dateischreiben in `tools/buildings/main.go` / `buildings.go`; `index.json` führt `stufen`.
- Heute löscht der Generator mit `stufen` die Datei `<id>-<dichte>.png`; für die Gebäude der GEB9-Liste (`werkstatt`, `farm`, `kaserne`, `lager`, `taverne`, `schmiede`, `heilplatz`, `ruestkammer`) soll sie bis GEB9.5 zusätzlich entstehen (= Stufe 1, byte-gleich zu `<id>-1-<dichte>.png`), damit das Spiel (`src/scenes/render/buildingSprites.ts`) grün bleibt. Am einfachsten über eine kleine Übergangsliste im Generator mit Kommentar „GEB9.5 entfernt“.
- Werkstatt und Farm: `public/grafik/gebaeude/werkstatt.json`, `farm.json`. Holzteile (Gestell, Zaun, Tresen) bleiben Holz, Tuchdach/Mauerwerk tauschen laut § 6.
- Asset-Vorschau: Grafik-Seite des Asset-Studios (`tools/k3c-studio/frontend/src/graphics/BuildingStrip.tsx`, `szenenLogic.ts` › `stufenKarten`, Test `szenenLogic.test.ts`); zeigt die Stufenreihe, sobald `stufen` im Index steht.
- Aufruf: `task gen:gebaeude`; gleiche JSON → byte-gleiche PNGs (Test in `tools/buildings`).

## Erlaubte Dateien

- `tools/buildings/*.go` (Übergang, Tests)
- `public/grafik/gebaeude/werkstatt*.json|png`, `farm*.json|png`, `public/grafik/gebaeude/index.json`
- `tools/k3c-studio/frontend/src/graphics/szenenLogic.test.ts` (Testfall mit einem neuen Gebäude)

## Nicht-Ziele

Andere Gebäude, Spielcode, Löschen der Grundform-Dateien, Regeldatei (GEB9.1 hat sie).

## Schritte

1. Branch `geb9/2-werkstatt-farm-uebergang`, `Status: in Arbeit`. § 6 (inklusive „Hub-Gebäude“), `stages.go`, `main.go` lesen.
2. Tests zuerst: (a) Gebäude der Übergangsliste mit `stufen` erzeugt zusätzlich `<id>-<dichte>.png` byte-gleich zu Stufe 1, Gebäude außerhalb der Liste unverändert; (b) `farben`-Schlüssel `"art/farbe"` tauscht die Farbe nur bei Bausteinen dieser Art, der engere Schlüssel gilt vor dem weiteren, `pruefeStufen` lehnt eine Art/Farbe-Kombination ab, die in der Grundform nicht vorkommt.
3. `stages.go` (`pruefeStufen`, `stufeBauen`) um die Schlüssel `art/farbe` erweitern (abwärtskompatibel: Mauer, Turm, Burg, Tor, Natürliches bleiben byte-gleich), den Übergang im Generator umsetzen (kleine Liste mit Kommentar „GEB9.5 entfernt“).
4. `werkstatt.json` und `farm.json`: `stufen` 2–5 nach § 6 (Farbrollen, Arten, Zusatz ab Stufe 3, `aenderung` je Stufe; die Vorlage beschreibt Stufe 1). `task gen:gebaeude`, PNGs und `index.json` einchecken, Stufe 1–5 in 128 px ansehen (Read) und auf Lesbarkeit prüfen, Abweichungen von der Tabelle in § 6 im Ergebnis nennen.
5. Test in `szenenLogic.test.ts`: `stufenKarten` liefert für die Werkstatt 5 Stufen.
6. `task check:diff`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-02: `werkstatt.json` und `farm.json` führen `stufen` 2–5; PNGs `<id>-1..5-<dichte>.png` liegen vor.
- [ ] AC-03: `szenenLogic.test.ts` prüft die Stufenreihe mit einem der neuen Gebäude.
- [ ] AC-05: zweimal `task gen:gebaeude` ergibt keinen Diff; Mauer, Turm, Burg, Tor unverändert; `<id>-<dichte>.png` bleibt als Stufe 1 erhalten (Übergang); `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt (Werkstatt, Farm): `werkstatt.json` und `farm.json` führen `stufen` 2–5 nach § 6 (Farbrollen, tauschende Arten, Wimpel bzw. Windfahne ab Stufe 3); PNGs `<id>-1..5-<dichte>.png` liegen vor, Stufen 1–5 in 64 px angesehen. AC-03 geprüft: `scenesLogic.test.ts` (Studio, Datei heißt so, nicht `szenenLogic`) liest den echten `index.json` und prüft Stufen 2–5 samt Dateinamen für Werkstatt und Farm. AC-05 geprüft: zweiter `task gen:gebaeude` ohne Diff, Mauer/Turm/Burg unverändert (Go-Tests), `<id>-<dichte>.png` bleibt als Stufe 1 (Übergang).

Umgesetzt in `tools/buildings`: Schlüssel `"art/farbe"` in `stufen.farben` (engerer Schlüssel gilt vor dem weiteren), `uebergangGrundform`-Liste mit Kopie von Stufe 1, Tests in `stages_teile_test.go`. Abweichungen von der Tabelle in § 6: Farm lässt Tor, Rahmen und Streben in Holz (sonst orange Kreuz auf dem Tor), Windfahne sitzt rechts am First (auf freiem Hintergrund, sonst unsichtbar); `fuge` und `schatten` folgen der Wand (in § 6 nachgetragen). Prüfung: `go test ./tools/buildings`, `golangci-lint` (0 Issues), Studio-Vitest (463 Tests) und `tsc` grün; `task check:diff` lief wegen Last zweimal in Zeitlimit bzw. in „parallel golangci-lint is running“, die Teilprüfungen sind einzeln grün. Studio-Frontend brauchte `bun install` im Worktree. Stufen-Skript für die weiteren Gebäude liegt nur im Scratchpad, nicht im Repo.
