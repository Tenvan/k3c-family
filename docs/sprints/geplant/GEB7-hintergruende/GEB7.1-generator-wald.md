# GEB7.1 · Generator für Hintergrund-Schichten und Wald

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb7/1-generator-wald
- **Abhängig von:** –
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

`tools/gebaeude` kennt `wiederholung` und die Klasse `hintergrund` (nur Dichten 16 und 32), ein Go-Test prüft Kachelbarkeit, transparente Oberkante, deckende Unterkante und die Helligkeitsstufen; der Wald (`hg-wald-fern|mitte|nah`) liegt als Daten-JSON mit PNGs vor und erscheint in der Vorschau (Streifen „Hintergründe“).

## Kontext

- **Spec:** GEB7 AC-01 bis AC-04, › Anforderungen (Schichten, Generator, Farbe und Lesbarkeit, Vorschau), › Offene Fragen 1, 2, 6, 7; Regeldatei `docs/standards/gebaeude-grafik.md` § 1, § 3, § 5, § 7; B-435 › Anforderungen (Daten und Skript im Repo).
- **Werkzeug heute:** `tools/gebaeude/gebaeude.go` (`baustein`, `render`, `check`, `build`, `renderDichten`: rendert immer `dichten` = 16/32/64/128), `stufen.go` (`renderStufen`), `kachel_test.go` (`TestHintergrundmauerKachelbar`: erste = letzte Pixelspalte nur für `hintergrundmauer`). Rechtecke in Kacheln, Vielfache von 1/32, Palette DB32 (`db32.go`).
- **Klasse:** Die Spec nennt die Schichten teils `kachel`, teils `hintergrund`. Festgelegt (Spec Revision 2): Klasse **`hintergrund`** = kachelbar wie `kachel` (gleiche Spalten-Prüfung, Vorschau) und zusätzlich nur Dichten 16 und 32, Oberkante transparent, Unterkante deckend. `kachel` (Hintergrundmauer) bleibt unverändert, alle vier Dichten.
- **`wiederholung`:** optionales Feld am Baustein: `{ "schritt": <Kacheln, Vielfaches von 1/32, > 0>, "anzahl": <ganze Zahl ≥ 2> }`. Das Werkzeug erzeugt `anzahl` Kopien (die erste ist das Original) um je `schritt` nach rechts; jede Kopie läuft durch dieselben Prüfungen wie ein einzelner Baustein (Raster, Maße). Fehler (Schritt ≤ 0, kein Vielfaches von 1/32, Anzahl < 2, Kopie außerhalb des Bilds) nennen Gebäude, Baustein und Feld. Auflösen vor dem Zeichnen, damit `check` und `render` unverändert bleiben; gilt auch für `zusaetze` einer Stufe.
- **Kachelbar mit festen Bausteinen:** Es gibt keine Bausteine über die Bildkante hinaus. Nahtlos heißt hier: nichts berührt die erste oder letzte Pixelspalte außer durchgehenden Bändern über die ganze Breite (Boden, Dunstband); alles andere steht mit Abstand zum Rand. Test: Spalte 0 = Spalte b−1 in jeder Dichte.
- **Maße:** 24 × 15 Kacheln = 768 × 480 px (32 px) und 384 × 240 px (16 px); 15 Kacheln = Höhe der heutigen Schichten. Bodenlinie = Unterkante des Bilds, oben transparent (Himmel bleibt `palette.sky` aus `data/biomes/forest.json`).
- **Farbe (AC-04):** Nur DB32. Fern < mitte < nah in der mittleren Helligkeit (Luma je deckendem Pixel, 0,299 R + 0,587 G + 0,114 B); alle Schichten dunkler als die Gebäude davor. Die Mindestabstände und die Obergrenze für „nah“ legt diese Session an `werkstatt-32.png`, `mauer-1-32.png` und den heutigen Wald-Schichten gemessen in der Regeldatei § 10 fest (Vorschlag: je Schicht mindestens 8 Luma-Punkte heller als die dahinter, „nah“ im Mittel unter dem Mittel der Werkstatt) und prüft sie im Test. Neue Materialien (Dunst, Fels dunkel) zuerst in § 3 eintragen, mit Zustimmung von 🧑: Die Zustimmung steht für diesen autonomen Lauf aus der Freigabe der Spec (Annahmen), Ralf prüft sie in der Vorschau nach.
- **Wald-Motiv (Annahme Frage 6):** fern = Hügelkämme und Stämme im Dunst (dunkles Grün/Opal, entsättigt); mitte = Laubbäume (Farbwelt der Bäume: Laub `christi`/`atlantis`/`dell`, abgedunkelt; Stämme `oiled-cedar`); nah = kräftige Stämme mit Laubansatz, Boden-Farn. Wiederholung mit festen, unregelmäßigen Versätzen (mehrere Gruppen mit verschiedenen Schritten), keine Zufallsfunktion. Das Wald-Gate (🧑 prüft den Wald vor GEB7.2/7.3) entfällt im autonomen Lauf: Annahme Frage 7 (nur Rechtecke) gilt, die Vorschau prüft 🧑 nach Abschluss.
- **Vorschau:** `src/tools/assetvorschau.ts` (`renderNeueGebaeude`, Streifen vor den Slots, `szene=gebaeude`), reine Logik in `assetvorschauLogic.ts` mit Test (`assetvorschauLogic.test.ts`). Neu: Streifen „Hintergründe“: je Biom ein Block mit den drei Schichten (fern zuerst) übereinandergelegt auf der Himmelsfarbe des Bioms, 32 px ×1 und 16 px ×2, zwei Kacheln nebeneinander (Naht sichtbar, waagerecht scrollbar). Zuordnung Biom → Schicht-Präfix: `forest` → `wald`, `cave` → `hoehle`, `mine` → `mine`, `ironhold` → `eisenstollen`, `crystal` → `kristall`; Himmelsfarbe `palette.sky` aus `data/biomes/<id>.json` (Import wie andere Tools). Biome ohne Schichten im `index.json` (hier Höhle bis Kristall) erscheinen nicht.
- **Fallstricke:** `index.json` führt `dateien` nur für 16/32 (Vorschau darf 64/128 nicht voraussetzen: `gebaeudeKarten` prüfen). Älteres bleibt byte-gleich: md5 aller vorhandenen PNGs vor und nach `task gebaeude`. Wegwerf-Skripte zum Erzeugen der JSON gehören in den Scratch-Ordner, nicht ins Repo; die JSON ist die Quelle.
- **Team:** Generator + Test + Vorschau und die Wald-JSONs sind unabhängig; Review durch getrennten Agenten.

## Erlaubte Dateien

- `tools/gebaeude/` (`gebaeude.go`, `stufen.go` nur falls nötig, neue Dateien für `wiederholung` und Tests)
- `public/grafik/gebaeude/` (neu: `hg-wald-fern|mitte|nah.json` und PNGs, `index.json`)
- `src/tools/assetvorschau.ts`, `src/tools/assetvorschauLogic.ts`, `src/tools/assetvorschauLogic.test.ts`, `src/tools/codemap.md`
- `docs/standards/gebaeude-grafik.md`, `docs/glossar.md` (nur falls ein neuer Begriff nötig wird)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine anderen Biome (GEB7.2, GEB7.3), kein Spiel-Code und kein Register (`data/assets.json`, GEB7.5), keine neuen Bausteinarten über Rechtecke hinaus (Dreiecke, Rauschen), keine Animation, keine Änderung an vorhandenen PNGs oder `data/`, keine neue Abhängigkeit.

## Schritte

1. `Status: in Arbeit`, `Start-Commit` steht schon; Domänen-Sperre DEV prüfen (`git ls-remote --heads origin 'sprint/*'`, Session-Dateien auf anderen Sprint-Branches mit Domäne DEV und `in Arbeit`: dann nichts tun und melden); committen, pushen.
2. md5 aller PNGs unter `public/grafik/gebaeude/` sichern (Scratch-Ordner).
3. Tests zuerst (rot): (a) `wiederholung`: Kopien an den erwarteten Pixeln, Schritt 0 / negativ / nicht im Raster / Anzahl 1 / Kopie außerhalb → Fehler mit Meldung; (b) Klasse `hintergrund` rendert nur `-16.png` und `-32.png`, andere Klassen weiter vier Dichten; (c) `TestKachelKlassen` über alle JSONs der Klassen `kachel` und `hintergrund` in `public/grafik/gebaeude/`: Spalte 0 = Spalte b−1 je Datei, bei `hintergrund` zusätzlich oberste Zeile transparent, unterste Zeile ohne Lücke, alle drei Schichten je Biom vorhanden und Helligkeit fern < mitte < nah um den Mindestabstand; `TestHintergrundmauerKachelbar` geht darin auf oder bleibt.
4. Generator: `wiederholung` auflösen (neue Datei, ≤ 60 Zeilen je Funktion), `dichtenFuer(klasse)`, `index.json` ohne 64/128 für `hintergrund`; Meldungen klar (Datei, Baustein, Feld).
5. Vorschau: Logik für den Streifen „Hintergründe“ (Biom, Himmelsfarbe, Schichten in Reihenfolge fern → nah) mit Test, Darstellung in `assetvorschau.ts` (Datei ≤ 400 Zeilen), `codemap.md` ergänzen.
6. Wald: `hg-wald-fern|mitte|nah.json` (Vorlage nach § 5: `motiv`, `teile`, `materialien`, `stimmung`, `fein`; Klasse `hintergrund`; `kacheln` 24 × 15). Rendern, PNGs sichten (`Read` auf `-32.png`, am besten auf Himmelsfarbe `#`-Wert aus `data/biomes/forest.json` gestapelt im Scratch-Ordner), nachbessern bis lesbar; Maße, Klasse, Dichten in § 1 und § 7, Materialien in § 3, neue Bausteinarten in § 4, Klasse und `wiederholung` in § 5, § 10 „Hintergrund-Schichten“ (Mindeststufen, Himmelsfarbe, Scroll-Faktoren als Hinweis) ergänzen. Zweiter `task gebaeude`-Lauf ändert nichts; ältere PNGs md5-gleich.
7. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Hintergrund-Schichten im Generator, Wald (GEB7.1)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-01 (Teil Wald): `hg-wald-fern|mitte|nah.json` mit Vorlage, `-16.png` (384 × 240) und `-32.png` (768 × 480); kein 64/128; zweiter `task gebaeude`-Lauf ändert nichts; ältere PNGs md5-gleich; Maße und Klasse in Regeldatei § 1.
- [ ] AC-02: `TestKachelKlassen` grün (Naht, Oberkante, Unterkante) für die Wald-Schichten; Generator-Tests für Raster, Maße und `wiederholung` grün.
- [ ] AC-03 (Wald): `index.json` führt die drei Schichten, die Vorschau zeigt den Streifen „Hintergründe“ mit dem Wald (Logik-Test; Sicht gehört 🧑).
- [ ] AC-04 (Wald): nur DB32; Helligkeitsstufen fern < mitte < nah im Test; Mindeststufen in § 10, neue Materialien in § 3.
- [ ] `task check` und `task check:go` grün (`check_run` mit `checkout`).

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Mit laufender Workbench stattdessen `check_run` (mit `checkout`). Keine manuellen Prüfungen; die Sichtprüfung gehört 🧑 (GEB7.6, AC-08).

## Ergebnis

- **AC-01 umgesetzt (Teil Wald):** `hg-wald-fern|mitte|nah.json` mit Vorlage, je `-16.png` (384 × 240) und `-32.png` (768 × 480), kein 64/128; Maße, Klasse und Dichten in Regeldatei § 1, § 5, § 7, § 10. Zweiter `task gebaeude`-Lauf byte-gleich; alle 156 älteren PNGs md5-gleich (vorher/nachher verglichen).
- **AC-02 geprüft:** `TestKachelKlassen` (Naht, oberste Zeile transparent, unterste deckend) und Generator-Tests (`wiederholung`: Kopien, Fehlerfälle, Bild; Klasse `hintergrund` nur 16/32) grün (`go test ./tools/gebaeude`).
- **AC-03 geprüft (Logik):** `hintergrundStreifen` mit Vitest; `assetvorschau.ts` zeichnet den Streifen „Hintergründe“ (Biom-Himmel, drei Schichten, zwei Kacheln, 32 px ×1 und 16 px ×2); `gebaeudeKarten` blendet `hintergrund` aus. Sicht im Browser: nicht geprüft, gehört 🧑.
- **AC-04 geprüft (Wald):** nur DB32; Luma fern 39,6 / mitte 52,8 / nah 71,3 (`TestHintergrundStufen`, Abstand ≥ 8, nah ≤ 75); neue Materialien `Hintergrund Wald|Holz|Fels|Eisen|Glut|Kristall` in § 3 (Zustimmung aus der Spec-Freigabe, Ralf prüft nach).
- `task check` grün (`check_run`, nach `npm ci --prefix tools/k3c-dev/frontend`: dem Worktree fehlten die Frontend-Pakete, kein Codefehler). Review durch getrennten Agenten: keine schweren Befunde.
- Abweichungen: Klasse `hintergrund` statt `kachel` (Spec Revision 2); Wald-Gate entfällt (Revision 2). Wald in Wegwerf-Skript im Scratch erzeugt, die JSON ist die Quelle. Neue Tickets: keine.
