# GEB7.2 · Hintergründe Höhle und Mine

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb7/2-hoehle-mine
- **Abhängig von:** GEB7.1
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

`hg-hoehle-fern|mitte|nah` und `hg-mine-fern|mitte|nah` liegen als Daten-JSON mit PNGs (16 und 32 px je Kachel) vor, kachelbar, oben transparent, unten deckend, mit den Helligkeitsstufen der Regeldatei.

## Kontext

- **Spec:** GEB7 AC-01, AC-02, AC-04, › Anforderungen (Motive, Farbe und Lesbarkeit), › Offene Fragen 1, 2, 6; Regeldatei `docs/standards/gebaeude-grafik.md` (§ 1, § 3, § 5, § 10 aus GEB7.1).
- **Vorbild:** `hg-wald-*.json` aus GEB7.1 (Aufbau, Maße 24 × 15, `wiederholung`, Vorlage, Klasse `hintergrund`); der Generator und `TestKachelKlassen` stehen schon. Himmelsfarben als Stapel-Untergrund beim Sichten: `data/biomes/cave.json` und `mine.json` › `palette.sky` (nur lesen).
- **Motive (Annahme Frage 6):** Höhle: fern = ferne Hallen und Felswand im Dunkel (Töne `valhalla`/`opal`/`deep-koamaru`), mitte = Felswand mit Stalaktiten (Wiederholung mit Versatz, Stein § 3), nah = nahe Felsbögen und Stalagmiten. Mine: fern = Fels mit fernem Gerüst, mitte = Stützbalken und Querbalken vor Fels (Holz § 3), nah = Schienenbett und Pfosten (Holz, Eisen). Höhle bleibt bläulich-kühl (Himmel `cave`), Mine warm-braun.
- **Farbe:** nur DB32; fern < mitte < nah (Helligkeit), alle dunkler als die Gebäude (Mindeststufen aus § 10, Test `TestKachelKlassen`). Neue Materialien zuerst in § 3 (mit Zustimmung von 🧑 aus der Freigabe, Ralf prüft nach).
- **Fallstricke:** Nichts an Spalte 0 und der letzten Spalte außer durchgehenden Bändern (Naht); unterste Pixelzeile durchgehend deckend (Spalt über dem Boden und im Fluss); Stalaktiten hängen von oben, aber die Oberkante des Bilds bleibt transparent (Höhle beginnt mit Abstand zur Oberkante, der Himmel scheint oben durch). Ältere PNGs bleiben byte-gleich (md5 vor und nach `task gebaeude`).
- **Team:** Höhle und Mine sind unabhängig; je ein Agent rendert im Scratch-Ordner (`bin/gebaeude -root <scratch>`), kopiert die fertigen JSONs ins Repo; `task gebaeude` läuft danach einmal zentral.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `hg-hoehle-*.json`, `hg-mine-*.json` und PNGs, `index.json`)
- `tools/gebaeude/` (nur Fehlerkorrekturen mit Test, falls ein Befund aus GEB7.1 auftaucht)
- `docs/standards/gebaeude-grafik.md` (§ 3 Materialien, § 4 Bausteinarten)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Wald, Eisenstollen, Kristall (GEB7.1, GEB7.3), kein Spiel-Code und kein Register, keine neuen Bausteinarten im Generator, keine Änderung an vorhandenen PNGs.

## Schritte

1. `Status: in Arbeit`, Domänen-Sperre DEV prüfen (wie GEB7.1), committen, pushen; md5 der vorhandenen PNGs sichern.
2. `hg-hoehle-fern|mitte|nah.json` und `hg-mine-fern|mitte|nah.json` nach Kontext (Vorlage § 5, Klasse `hintergrund`, 24 × 15); im Scratch-Ordner rendern und sichten (`Read` auf `-32.png`, gestapelt auf der Himmelsfarbe des Bioms), nachbessern.
3. `task gebaeude` zentral; zweiter Lauf ändert nichts; ältere PNGs md5-gleich; `go test ./tools/gebaeude` grün (`TestKachelKlassen` greift die neuen Biome automatisch).
4. Regeldatei § 3 und § 4 nachtragen.
5. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Hintergründe Höhle und Mine (GEB7.2)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-01 (Teil Höhle, Mine): sechs JSONs mit Vorlage, je `-16.png` (384 × 240) und `-32.png` (768 × 480), kein 64/128; zweiter `task gebaeude`-Lauf ändert nichts; ältere PNGs md5-gleich.
- [ ] AC-02: `TestKachelKlassen` grün für Höhle und Mine (Naht, transparente Oberkante, deckende Unterkante).
- [ ] AC-04: nur DB32, Materialien in § 3; Helligkeitsstufen fern < mitte < nah im Test grün.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Mit laufender Workbench stattdessen `check_run` (mit `checkout`). Keine manuellen Prüfungen; die Sichtprüfung gehört 🧑 (GEB7.6, AC-08).

## Ergebnis

–
