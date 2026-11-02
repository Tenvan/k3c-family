# GEB18.1 · Generator erzeugt Hintergründe in vierfacher Auflösung und mit Varianten

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb18/1-generator-dichte-varianten
- **Abhängig von:** –
- **Tickets:** B-618
- **Kriterien:** AC-01

## Ziel

`tools/buildings` erzeugt Hintergrund-Schichten in Dichte 128 (vierfache Auflösung) und kennt Varianten je Biom (Ablage `hg-<teil>-<schicht>-v<n>-128.png`).

## Kontext

Generator `tools/buildings` (`buildings.go`: `erlaubt(d)`, `render`, `renderDichten`; `background_test.go`), Vorlagen und Ausgaben unter `public/grafik/gebaeude/hg-*`; Client-Abbildung `src/scenes/render/worldSprites.ts` (`BACKGROUND_LAYERS`, Slots `bg:<biom>:<n>`); Spec B-618, GEB7-Regeln in `docs/rules`.

## Erlaubte Dateien

- `tools/buildings/**`
- `Taskfile`-Eintrag in `tools/` nur, falls ein neuer Generator-Aufruf nötig ist

## Nicht-Ziele

Neue Vorlagen-Inhalte (GEB18.2, GEB18.3), Client, Level-Generator.

## Schritte

1. Dichte 128 für Hintergrund-Klassen erlauben, Ausgabe-Namen mit Variante.
2. Vorlagen-Format für Varianten (eine Vorlage je Variante oder Varianten-Liste) festlegen und in `codemap.md` beschreiben.
3. Go-Tests: Größe ×4 gegenüber 32, Varianten-Namen, Prüfung „fünf Varianten je Biom“ als Hilfsfunktion für GEB18.3.

## Fertig, wenn

- [ ] AC-01: `go test ./tools/buildings/...` grün mit Tests für Dichte 128 und Varianten.

## Prüfen

`check_run go:test` mit Muster `buildings`, vor dem Commit `task check:diff`.

## Ergebnis

- AC-01 umgesetzt: Hintergrund-Vorlagen kennen das Feld `raster` (32 Standard, 128 nur Klasse `hintergrund`): Koordinaten in 1/128 Kachel, gerendert in Dichte 128 (jedes Raster-Pixel ein Bild-Pixel, echte vierfache Auflösung statt Vergrößerung) und 32 (Spiel-Dichte); Bausteine mit `detail` entfallen unter 128. Varianten über die ID `hg-<biom>-<schicht>-v<n>` (ohne Endung Variante 1), Prüfhilfe `fehlendeVarianten` für GEB18.3. Neu `tools/buildings/hires.go`, `masterPx`/`aufloesen`/`dichtenFuer` nehmen das Raster.
- AC-01 geprüft: `go test ./tools/buildings/` grün (neu `hires_test.go`: 1:1 in 128, Breite ×4, Detail fehlt in 32, Raster-Regeln, Wiederholung im feinen Raster, Varianten-IDs und -Zählung; `TestHintergrundStufen` prüft je Variante), `go:lint` grün; `task gen:gebaeude` erzeugt alle 56 bestehenden Gebilde byte-gleich (nur Zeilenende von `index.json`, zurückgesetzt). `task check:diff` nicht gelaufen (Ralf: in dieser Claude-Session gesperrt).
- Abweichung: Die Spec nannte Dateinamen `hg-<teil>-<schicht>-v<n>-128.png`; so entstehen sie (`<id>-<dichte>.png`). Hintergründe im feinen Raster liefern 32 und 128, nicht mehr 16 (die bestehenden im Master-Raster weiter 16 und 32).
