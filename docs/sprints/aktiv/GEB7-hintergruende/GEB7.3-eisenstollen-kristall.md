# GEB7.3 · Hintergründe Eisenstollen und Kristallhöhle

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb7/3-eisenstollen-kristall
- **Abhängig von:** GEB7.1
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

`hg-eisenstollen-fern|mitte|nah` und `hg-kristall-fern|mitte|nah` liegen als Daten-JSON mit PNGs (16 und 32 px je Kachel) vor; damit sind alle 15 Hintergrund-Schichten vollständig.

## Kontext

- **Spec:** GEB7 AC-01, AC-02, AC-04, › Anforderungen (Motive, Farbe und Lesbarkeit), › Offene Fragen 1, 2, 6; Regeldatei `docs/standards/gebaeude-grafik.md` (§ 1, § 3, § 5, § 10).
- **Vorbild:** `hg-wald-*.json` (GEB7.1) und `hg-hoehle-*`, `hg-mine-*` (GEB7.2). Himmelsfarben: `data/biomes/ironhold.json` (`#1f0f0a`, dunkles Rotbraun) und `crystal.json` (`#0d1626`, dunkles Blau) › `palette.sky`, nur lesen; beide Biome haben heute keine Grafik, nur Silhouetten.
- **Motive (Annahme Frage 6):** Eisenstollen: fern = dunkler Fels, ferne Gänge; mitte = Fels mit Eisenträgern und Streben (Eisen § 3: `heather`/`light-steel`/`dim-gray`), nah = Träger und Pfeiler mit **Lavaschein** (Glut § 3, `tahiti-gold`/`brown` sparsam in Spalten am Boden, nicht heller als die Gebäude). Kristallhöhle: fern = Fels in Blau-Violett, mitte = Kristallgruppen (Kristall § 3: `royal-blue`/`cornflower`/`deep-koamaru`, Glanz `white` nur fein), nah = große Kristalle am Boden und von der Decke, Kristalllicht in `viking` sparsam. Wiederholung mit festen Versätzen, keine Zufallsfunktion.
- **Farbe:** nur DB32; fern < mitte < nah, alle dunkler als die Gebäude (Mindeststufen aus § 10); Leuchtfarben (Glut, Kristalllicht) nur in kleinen Flächen, damit die Helligkeitsstufe hält. Neue Materialien zuerst in § 3.
- **Fallstricke:** wie GEB7.2 (Naht, transparente Oberkante, deckende Unterkante, ältere PNGs md5-gleich).
- **Team:** zwei Agenten parallel im Scratch-Ordner; `task gebaeude` zentral.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `hg-eisenstollen-*.json`, `hg-kristall-*.json` und PNGs, `index.json`)
- `tools/gebaeude/` (nur Fehlerkorrekturen mit Test)
- `docs/standards/gebaeude-grafik.md` (§ 3, § 4)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Spiel-Code, kein Register, keine Änderung an `data/biomes/*.json` (Paletten an DB32 angleichen wäre ein eigenes REG-Ticket), keine Animationen (Lava-Flackern, Kristallglimmen), keine neuen Bausteinarten im Generator.

## Schritte

1. `Status: in Arbeit`, Domänen-Sperre DEV prüfen, committen, pushen; md5 der vorhandenen PNGs sichern.
2. Die sechs JSONs nach Kontext; im Scratch-Ordner rendern und sichten (gestapelt auf der Himmelsfarbe), nachbessern.
3. `task gebaeude` zentral; zweiter Lauf ändert nichts; ältere PNGs md5-gleich; `go test ./tools/gebaeude` grün; alle **15** `hg-*.json` stehen im `index.json`.
4. Regeldatei § 3 und § 4 nachtragen.
5. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Hintergründe Eisenstollen und Kristallhöhle (GEB7.3)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-01: alle 15 Schichten (`hg-<biom>-fern|mitte|nah`, 24 × 15, 16 und 32 px) liegen vor; byte-gleich wiederholbar; ältere PNGs md5-gleich.
- [ ] AC-02: `TestKachelKlassen` grün für alle 15 Schichten.
- [ ] AC-04: nur DB32, Materialien in § 3, Helligkeitsstufen für alle fünf Biome im Test grün.
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
