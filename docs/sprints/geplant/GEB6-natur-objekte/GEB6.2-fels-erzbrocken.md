# GEB6.2 · Fels und Erzbrocken mit Biom-Looks

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb6/2-fels-erzbrocken
- **Abhängig von:** GEB6.1
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

`felsen.json` (1 × 1, Looks Wald, Höhle, Mine) und `erzbrocken.json` (1 × 1, ein Look) liegen mit PNGs in `public/grafik/gebaeude/` und erscheinen mit ihren Looks in der Vorschau.

## Kontext

- **Spec:** GEB6 AC-01, AC-04, › Anforderungen (Fels, Erzbrocken, Biom-Look), Offene Fragen 1, 4, 6. Regeln: `docs/standards/gebaeude-grafik.md` (§ 1, § 3 Stein, Kupfer, § 5, § 6, § 7).
- **Heute im Spiel:** Slot `node:rock` zeigt Frame 1 und `node:copperOre` Frame 7 des Erz-Blatts (16 × 16 px, ×2). Fels steht in allen Biomen (Wald-Chunk „rocks“, Höhle, Mine, Eisenstollen, Kristallhöhle), Erzbrocken (`copperOre`) in Mine, Eisenstollen und Kristallhöhle (`data/biomes/*.json › resourcesPerChunk`).
- **Biom-Look:** Stufe = Look (GEB6.1, Regeldatei § 6): `felsen.json` Grundform = Stufe 1 Wald (Stein `dim-gray`, Licht `smokey-ash`, Fuge `valhalla`, etwas Moos in `christi`), `stufen` 2 = Höhle (kühler, bläulich, z. B. Stein → `deep-koamaru`/`venice-blue`, Moos → `smokey-ash`), 3 = Mine (dunkler und wärmer, z. B. Stein → `oiled-cedar`/`dim-gray`, Moos entfällt über Farbtausch auf Stein). `erzbrocken.json`: Fels in Stein mit Kupfer-Einsprengseln (Kupfer § 3: `tahiti-gold`, Licht `twine`, Fuge `rope`, Patina `verdigris`), ein Look (Mine); Eisenstollen und Kristallhöhle nutzen den Look Mine (Offene Frage 6).
- **Maße:** je 1 × 1 Kachel (32 × 32 px), niedrig und breit gelagert, Unterkante am Boden (§ 1). Der Fels ist ein **Abbau-Knoten** (`gatherables.rock`).
- **Materialien:** Fels = Zeile Stein; die Zeilen Fels, Gold, Zelttuch, Erde trägt GEB6.1 in § 3 ein (Offene Frage 4).
- Vorbild für Farbtausch und Vorschau-Stufenkarten: `mauer.json`, GEB6.1 (Bäume).

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `felsen.json`, `erzbrocken.json` und PNGs; `index.json`)
- `tools/gebaeude/` (nur nötige Erweiterungen, mit Test)
- `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel (GEB6.5), keine Änderung an vorhandenen JSONs und PNGs, keine Looks für Eisenstollen und Kristallhöhle, keine Adern (GEB6.3).

## Schritte

1. `sprint/geb6` auschecken, `Status: in Arbeit`, committen, pushen; md5 aller vorhandenen PNGs in `public/grafik/gebaeude/` sichern.
2. Vorlage zuerst (§ 5), dann Bausteine `felsen.json` (1 × 1, Grundform Wald, `stufen` 2 = Höhle, 3 = Mine mit `aenderung`) und `erzbrocken.json` (1 × 1, ohne Stufen).
3. `task gebaeude`; PNGs neben Mauer und Bäumen sichten (Stimmigkeit), nachbessern; md5 der vorhandenen PNGs gleich; zweiter Lauf ohne Änderung.
4. Regeldatei § 1 (Maße), § 4 (Bausteine), § 6 falls nötig; Vorschau zeigt die drei Fels-Looks (Stufenkarten).
5. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(dev): Fels und Erzbrocken (GEB6.2)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `felsen-1-32.png`, `felsen-2-32.png`, `felsen-3-32.png` (je 32 × 32 px) plus 16/64/128 und `erzbrocken-32.png` (32 × 32 px) plus 16/64/128, beide JSONs mit `vorlage`; Fels in drei erkennbar verschiedenen Looks.
- [ ] AC-04: md5 aller vorher vorhandenen PNGs gleich; ein zweiter `task gebaeude`-Lauf ändert nichts.
- [ ] AC-04: Regeldatei § 1 und § 4 nachgetragen; Vorschau zeigt die drei Fels-Looks und den Erzbrocken.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Mit laufender Workbench stattdessen `check_run` (mit `checkout`). Keine manuellen Prüfungen; die Sichtprüfung gehört GEB6.6.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
