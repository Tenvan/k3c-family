# tools/gebaeude/

## Responsibility
CLI `gebaeude` (GEB1, B-435): rendert Gebäude aus Daten-JSON (Rechtecke in Kachel-Einheiten, DB32-Farbnamen) zu PNGs in den Dichten 16, 32, 64 und 128 px je Kachel und schreibt `index.json` für die Asset-Vorschau. Gleiche Daten ergeben byte-gleiche PNGs. Regeln: `docs/standards/gebaeude-grafik.md`.

## Design
- `main.go`: Flags `-root`, `-dir` (Standard `public/grafik/gebaeude`); der Pfad-Präfix in `index.json` ist `-dir` ohne `public/`. Exit 1 bei Fehler.
- `gebaeude.go`: Typen `building` (`id`, `klasse`, `vorlage`, `kacheln`, `bausteine`, `stufen`), `baustein` (`art`, `rechteck [x,y,b,h]`, `farbe`, `fein`, `wiederholung`), `vorlage`, `eintrag`. `masterPx` rechnet Kacheln in Master-Pixel (`master` = 32, nur Vielfache von 1/32), `render` zeichnet ohne Glättung in eine `image.Paletted`, `check` prüft Farbe, Raster, Maße und Mindestbreite je Dichte (`fein`-Bausteine entfallen unter der Master-Dichte), `encode` schreibt PNG ohne Zeitstempel. `run` rendert erst alles, löscht dann alte `*.png` und schreibt PNGs plus `index.json`; scheitert ein Gebäude, bleibt der Ordner unverändert. `build` liest mit `DisallowUnknownFields`, prüft die Vorlage (`pruefeVorlage`: Motiv und mindestens ein Teil) und verlangt `id` = Dateiname.
- `db32.go`: DB32-Palette mit Namen; Index 0 der Bildpalette ist transparent.
- `stufen.go`: Ausbaustufen ab 2 (`stufe`: Änderungstext, Farbtausch alt → neu, Zusätze). `pruefeStufen` (lückenlos, Farben in DB32 und in der Grundform), `stufeBauen`, `renderStufen` (Dateien `<id>-<stufe>-<dichte>.png`), `renderDichten`.
- `wiederholung.go`: Klasse `hintergrund` (nur Dichten 16 und 32, `dichtenFuer`); `aufloesen` ersetzt Bausteine mit `wiederholung` (`schritt`, `anzahl`) durch Kopien entlang x, auch in den Zusätzen der Stufen.

## Flow
1. `run` globbt `*.json` (ohne `index.json`) sortiert → `build` je Datei: lesen, Vorlage prüfen, Wiederholungen auflösen, ID prüfen.
2. Ohne Stufen `renderDichten`, sonst `renderStufen`; der Karten-Eintrag zeigt die Grundform (Stufe 1).
3. Alte PNGs entfernen, neue schreiben, `index.json` schreiben; Ausgabe `✅ gebaeude: n Gebäude gerendert`.

## Integration
- Taskfile-Task `gebaeude` (baut `bin/gebaeude`, Quellen `public/grafik/gebaeude/*.json`, Ausgaben PNGs und `index.json`).
- Eingabe und Ausgabe unter `public/grafik/gebaeude/`; die Asset-Vorschau liest `index.json`.
- Tests: `gebaeude_test.go`, `stufen_test.go`, `vorlage_test.go`, `kachel_test.go`, `hintergrund_test.go`, `wiederholung_test.go`.
