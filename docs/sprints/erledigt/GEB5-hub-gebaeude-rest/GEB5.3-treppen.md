# GEB5.3 · Treppe hoch und Treppe runter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb5/3-treppen
- **Abhängig von:** GEB5.2
- **Tickets:** B-435
- **Kriterien:** AC-01, AC-04

## Ziel

`treppe-hoch.json` (4 × 4 Kacheln, ein Bild) und `treppe-runter.json` (4 × 3 Kacheln) liegen mit Vorlage, ohne `stufen`, als PNGs in `public/grafik/gebaeude/` und erscheinen in der Vorschau.

## Kontext

- **Spec:** GEB5 AC-01, AC-04, › Anforderungen (Treppen), › Offene Frage 6; Regeldatei § 1, § 3, § 5.
- **Heute:** Treppe hoch = fünf 16 × 32-Kacheln aus `gothicvania-town` (`STAIRS_UP_TILES` in `src/scenes/buildingSprites.ts`, zusammengesetzt in `siteView.stairsUp`), Treppe runter = Ausschnitt `entrance` aus `phantasy-dungeon-entrance/ebenen/dungeon-door.png`. Beide sind Hub-Plätze ab Hub-Stufe 2 und kosten Stein (`materialien-gebaeude.md` § 3.2).
- **Motive:** Treppe hoch = steinerne Außentreppe, steigt von links unten nach rechts oben, mit Geländer und Absatz oben (Richtung „hinauf“ lesbar); Treppe runter = Steinbogen über Stufen, die in einen dunklen Abgang führen (Richtung „hinab“ lesbar). Stein-Farben wie Mauer-Stufe 2 (dim-gray/smokey-ash/valhalla/black).
- **Plätze:** Treppe runter +8, Treppe hoch +16, Burg reicht bis +4: 4 Kacheln Breite passen ohne Überlappung.
- **Fallstricke:** md5 aller vorhandenen PNGs vor `task gebaeude` sichern und danach vergleichen.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (neu: `treppe-hoch.json`, `treppe-runter.json` und PNGs, `index.json`)
- `tools/gebaeude/`, `src/tools/assetvorschau*.ts` (nur nötige Erweiterungen mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Darstellung im Spiel und kein Entfernen von `STAIRS_UP_TILES` (GEB5.4), keine Stufen, keine Änderung an vorhandenen PNGs, an `data/` oder am Stufenwechsel.

## Schritte

1. `sprint/geb5` auschecken, `Status: in Arbeit`, committen, pushen.
2. md5 der vorhandenen PNGs sichern.
3. `treppe-hoch.json` und `treppe-runter.json` nach Kontext; `task gebaeude`; PNGs sichten (32 und 16 px, neben `burg-2-32.png`), nachbessern; md5 der übrigen PNGs gleich.
4. Regeldatei § 1 (Maße Treppen).
5. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-01/AC-04: `treppe-hoch-32.png` 128 × 128 px und `treppe-runter-32.png` 128 × 96 px plus 16/64/128; zweiter `task gebaeude`-Lauf ändert nichts; ältere PNGs md5-gleich; beide in der Vorschau.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task gebaeude
task check
task check:go
```

Keine manuellen Prüfungen; die Sichtprüfung gehört GEB5.5.

## Ergebnis

- **AC-01 geprüft:** `treppe-hoch.json` (4 × 4, ein Bild: acht Stein-Stufen, Holzgeländer, Laterne am Absatz) und `treppe-runter.json` (4 × 3, Steinbogen mit Zinnen, drei Stufen in den dunklen Abgang) mit Vorlage, ohne `stufen`; `treppe-hoch-32.png` 128 × 128, `treppe-runter-32.png` 128 × 96 plus 16/64/128; beide stehen in `index.json` und damit in der Vorschau (`assetvorschau.ts` liest sie, keine Code-Änderung).
- **AC-04 geprüft:** `task gebaeude` zweimal (zweiter Lauf mit `--force`): Treppen-PNGs byte-gleich; md5 aller 148 vorhandenen PNGs vor und nach dem Lauf gleich; Maße in Regeldatei § 1, neue Bausteinarten in § 4.
- **Prüfen:** `check_run task:check` und `task:check:go` grün (Worktree-Checkout).
- **Review:** getrennter Reviewer (Raster 1/32, `fein`-Regel, DB32-Namen, Rechtecke im Bild, Dateiumfang, Regeldatei): keine schweren Befunde.
- Manuell geprüft hat niemand; die Sichtprüfung gehört GEB5.5. Claude hat die PNGs nur als Bild gesichtet und die Stufen des Bogens einmal nachgebessert (heller Auftritt vorn).
- Abweichungen vom Plan: keine. Neue Tickets: keine.
