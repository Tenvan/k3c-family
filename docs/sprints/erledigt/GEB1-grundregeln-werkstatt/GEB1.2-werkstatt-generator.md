# GEB1.2 · Werkstatt: Regeldatei, Generator, PNG, Vorschau

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb1/2-werkstatt
- **Abhängig von:** GEB1.1
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-02, AC-03, AC-06

## Ziel

Die Regeldatei für Gebäude-Grafik steht, das Go-Werkzeug `tools/gebaeude` rendert die Werkstatt aus Daten in 16/32/64/128 px je Kachel, und die Asset-Vorschau zeigt die Werkstatt in 32 px und 16 px ×2 nebeneinander.

## Kontext

- **Spec:** Sprint-README GEB1 (Anforderungen, Dichte-Regel, Ausnahme- und Fehlerfälle, AC-01/02/03/06) und B-435 (Anforderungen, Beispiele, Notizen › Slot-Register). Entscheidungen aus GEB1.1 stehen unter Offene Fragen.
- **Werkstatt (Stilvorlage, 🧑 2026-10-15):** Handelsstand in Vorderansicht, 6×4 Kacheln (192×128 px bei 32 px), Pfosten aus Holz, Stoffdach (Tuch), Tresen, Gestell rechts für abholbereite Hämmer und Bögen; Palette DB32, Licht von links oben. Die damalige Skript-Quelle liegt nicht im Repo: Die Werkstatt wird aus dieser Beschreibung neu aufgebaut, 🧑 vergleicht in GEB1.4.
- **Vorbild Generator:** `tools/atlas` (`main.go` mit Flags, Logik in `atlas.go`, `atlas_test.go`, Taskfile-Ziel `atlas` mit `sources`/`generates`, feste EXE `bin/atlas{{exeExt}}`). Neues Ziel `gebaeude` genauso: `go build -o bin/gebaeude{{exeExt}} ./tools/gebaeude` und Aufruf.
- **Datenformat (Vorschlag, im Werkzeug festlegen und in der Regeldatei dokumentieren):** `{ "id", "kacheln": {"b": 6, "h": 4}, "bausteine": [ { "art": "pfosten|tuch|tresen|gestell|…", "rechteck": [x, y, b, h] in Kachel-Einheiten, "farbe": "<DB32-Name>", "fein": true? } ] }`. Koordinaten sind Vielfache von 1/32 Kachel (1 px der Master-Dichte), sonst Fehler; dadurch sind 64 und 128 px exakte Vergrößerungen (AC-06). Bei 16 px wird auf ganze Pixel gerundet (`floor`), die Mindestbreite ist 1 px der Zieldichte; `fein`-Bausteine entfallen bei 16 px.
- **DB32:** die 32 Farben der DawnBringer-32-Palette als feste Tabelle im Werkzeug; unbekannte Namen → Fehler.
- **Vorschau:** `assetvorschau.html?szene=gebaeude` (`src/tools/assetvorschau.ts`, Logik in `assetvorschauLogic.ts`, Test `assetvorschauLogic.test.ts`). Die Slots kommen aus `data/assets.json`; diese Datei bleibt unberührt. Stattdessen schreibt das Werkzeug `public/grafik/gebaeude/index.json` (Liste der Gebäude mit PNG je Dichte), und die Szene `gebaeude` zeigt darüber einen eigenen Streifen „Neue Gebäude (GEB)“: je Gebäude 32 px ×1 und 16 px ×2 nebeneinander, `image-rendering: pixelated`, keine CSS-Skalierung außer ganzzahlig.
- **Fallstricke:** stdlib `png.Encode` schreibt keine Zeitstempel und ist deterministisch; kein `math/rand`; `image.Paletted` mit DB32 hält die Farben fest.

## Erlaubte Dateien

- `tools/gebaeude/` (neu: `main.go`, Logik, `*_test.go`, `codemap.md`)
- `public/grafik/gebaeude/` (neu: `werkstatt.json`, `werkstatt-16.png`, `werkstatt-32.png`, `werkstatt-64.png`, `werkstatt-128.png`, `index.json`)
- `docs/standards/gebaeude-grafik.md` (neu)
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`
- `Taskfile.yml` (nur neues Ziel `gebaeude`)
- `docs/sprints/` (Session, Sprint-README), `docs/backlog/` (neue Tickets)

## Nicht-Ziele

Keine Bäume (GEB1.3), keine Ausbaustufen über Stufe 1 hinaus (nur Mechanik in der Regeldatei beschreiben), keine Zuordnung ins Spiel (`data/assets.json`, `src/scenes/`), keine Prompt-Erzeugung im Asset-Manager, keine Animation, keine neue Abhängigkeit.

## Schritte

1. `Status: in Arbeit`, Sprint-Branch `sprint/geb1` nach `docs/arbeitsweise.md` › Autonomer Ablauf.
2. Test zuerst (`tools/gebaeude/*_test.go`): Dichte 64 → nach Nearest-Downsampling (2×2) gleich Dichte 32; Dichte 48 → Fehler mit erlaubten Werten; fremde Farbe → Fehler; Form außerhalb der Maße → Fehler; zweimal rendern → gleiche Bytes; Dichte 16 ohne `fein`-Bausteine.
3. Werkzeug bauen: JSON lesen, je Dichte als `image.Paletted` (DB32) rendern, PNG und `index.json` schreiben. Flags wie `tools/atlas` (`-root`, `-out`).
4. `werkstatt.json` nach der Beschreibung im Kontext anlegen (Pfosten, Tuch, Tresen, Gestell, Werkzeuge am Gestell als `fein`, Licht links oben über hellere Kanten links/oben).
5. Taskfile-Ziel `gebaeude` (`sources`: `tools/gebaeude/*.go`, `public/grafik/gebaeude/*.json` ohne `index.json`; `generates`: PNGs, `index.json`), `task gebaeude` laufen lassen, PNGs committen.
6. Vorschau: Streifen „Neue Gebäude (GEB)“ in der Szene `gebaeude` aus `index.json`; reine Logik (Liste → Karten) in `assetvorschauLogic.ts` mit Test.
7. `docs/standards/gebaeude-grafik.md` schreiben: Raster (Kachel = `UNIT_PX` = 32 px), Kachelmaße je Gebäudeklasse (Stand + Ausblick aus B-435 › Notizen), Palette DB32, Licht links oben, Materialsprache Holz/Stoff, Baustein-Liste, Ausbaustufen über Paletten und Zusätze, Datenformat, Dichte-Regel, Skalierungsregel (ganzzahlig für alles, Faktor aus GR7.4 offen), Ablauf `task gebaeude`.
8. Ergebnis je Kriterium ausfüllen, `Status: fertig`.

## Fertig, wenn

- [ ] AC-01: `docs/standards/gebaeude-grafik.md` enthält alle in AC-01 genannten Punkte.
- [ ] AC-02: `task gebaeude` erzeugt `werkstatt-32.png` mit 192×128 px; zweiter Lauf → `git status` ohne Änderung an PNGs.
- [ ] AC-06: `go test ./tools/gebaeude/...` grün mit den Tests aus Schritt 2.
- [ ] AC-03: `assetvorschauLogic.test.ts` prüft die Karten aus `index.json` (32 px ×1, 16 px ×2); Browser-Sicht erst in GEB1.4.
- [ ] `task check` und `task check:go` grün; keine Datei über 400, keine Funktion über 60 Zeilen.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen in dieser Session; die Sichtprüfung gehört GEB1.4.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-01 umgesetzt: `docs/standards/gebaeude-grafik.md` (Raster, Maße, DB32, Licht, Material, Bausteine, Datenformat, Stufen, Dichte, Skalierung; Faktor offen bis GR7.4, Spiel-Dichte offen bis GEB1.4).
- AC-02 geprüft: `task gebaeude` → `werkstatt-32.png` 192×128 px, dazu 16/64/128; PNGs gelöscht und neu gebaut → md5 gleich.
- AC-06 geprüft: `go test ./tools/gebaeude` (64→32 pixelgleich, Dichte 48 abgelehnt, 16 ohne `fein`, Fehlerfälle, gleiche Bytes).
- AC-03 umgesetzt: `gebaeudeKarten` mit Test in `assetvorschauLogic.test.ts`; Streifen in `assetvorschau.html?szene=gebaeude`; Sichtprüfung in GEB1.4.
- Checks: `task check` grün (2342 Tests), `golangci-lint ./tools/gebaeude` 0 Befunde. Review (Sonnet-Agent): keine schweren Befunde; verwaiste PNGs behoben (Werkzeug löscht alte PNGs vor dem Lauf), Katalog-Befund → B-437.
- Abweichungen: außerhalb der erlaubten Dateien `src/tools/texts.de.ts`/`texts.en.ts` (Text `av.geb`, Textregel PL2) sowie `credits.test.ts`/`grafikPacks.test.ts` (Ausnahme für eigene Grafik `gebaeude/`, Freigabe Ralf im Chat). Werkstatt aus der Beschreibung neu aufgebaut (Entwurfs-Quelle nicht im Repo). k3c-dev fiel während der Session aus; Planung danach von Hand, Prüfungen in der Shell.
