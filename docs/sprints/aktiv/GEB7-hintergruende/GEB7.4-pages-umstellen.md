# GEB7.4 · Pages-Seite auf neue Hintergründe umstellen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** geb7/4-pages-umstellen
- **Abhängig von:** GEB7.3
- **Tickets:** B-435
- **Kriterien:** AC-07

## Ziel

`site/index.html` (GitHub-Pages-Seite) lädt für Hero-Parallax und die drei Tiefen-Kacheln nur noch die neuen Hintergrund-Schichten aus `app/grafik/gebaeude/`; kein `url()` zeigt mehr auf `forest-background`, `blue-cave-background` oder `warped-caves-pixel-art-pack`.

## Kontext

- **Spec:** GEB7 AC-07, › Regeln (Domänen nacheinander: PLAT vor dem Löschen), › Offene Fragen 5. **Annahme:** Frage 5 wird mit „umstellen“ beantwortet (Empfehlung der Spec, Freigabe-Quelle: Chat 2026-10-16, Ralf: Umsetzung auf Annahmen). Bei anderer Antwort von 🧑 („unverändert lassen“) entfällt diese Session (`verworfen`), GEB7.5 löscht dann nur `warped-super-grotto-escape-pack` und die ungenutzten `ebenen/*` von `sunnyland-tall-forest-environment` und lässt die drei Packs mit Vermerk `verschoben` stehen.
- **Heute (`site/index.html`):** Hero: vier Ebenen `.l-back`, `.l-lights`, `.l-mid`, `.l-front` mit `url(app/grafik/forest-background/ebenen/parallax-forest-*.png)`, `background-size: 1156px 680px` (Pack 272 × 160 mal 4,25), Scroll-Animation `@keyframes scroll` bis `-1156px`, `.l-lights` mit `opacity: 0.8`; Tiefen-Kacheln `.d0` (Wald, `parallax-forest-middle-trees.png`), `.d1` (Höhle, `startcavebg.png`), `.d2` (Mine, `warped-caves-pixel-art-pack/ebenen/background.png`) als `--pic` mit `background: var(--pic) center / cover`. Die HTML-Zeilen 151–154 legen die vier Hero-Ebenen als `<div class="layer l-… px">` an.
- **Neu:** Pfade `app/grafik/gebaeude/hg-<biom>-<schicht>-32.png` (`task pages` baut `dist` unter `app/`, `public/grafik/gebaeude/` liegt darin). Hero: drei Ebenen `hg-wald-fern`, `hg-wald-mitte`, `hg-wald-nah` (Dauer 90 s / 50 s / 30 s wie die vorhandenen Ebenen; `.l-lights` entfällt samt HTML-Zeile), Größe 768 × 480 skaliert auf Höhe 680 → `background-size: 1088px 680px`, `@keyframes scroll` bis `-1088px`; `image-rendering` bleibt wie die Klasse `px` es setzt. Tiefen-Kacheln: `.d0` = `hg-wald-mitte-32.png`, `.d1` = `hg-hoehle-mitte-32.png`, `.d2` = `hg-mine-mitte-32.png`. Die Texte der Seite ändern sich nur, wenn sie Pack-Namen nennen (`git grep` auf „Forest Background“, „Warped“ in `site/`).
- **Zusammenspiel:** `site/` ist PLAT; PG1 (Präsentation Bilder) ändert dieselbe Datei, ist aber nicht aktiv; trifft die Session auf dessen Stand, ersetzt sie nur die `url()`-Zeilen. Pack-Dateien löscht erst GEB7.5.
- **Prüfung ohne Browser:** `task pages` baut `_site/`; ein Skript (Scratch) oder `git grep` listet alle `url(app/...)`-Pfade aus `site/index.html` und prüft, dass jede Datei unter `public/` existiert. Die Sicht (Hero und Kacheln) prüft 🧑.

## Erlaubte Dateien

- `site/` (`index.html`, ggf. weitere `site/*`, die dieselben Pack-Pfade nennen)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Löschen von Grafik (GEB7.5), keine neuen Abschnitte oder Texte der Seite, keine Änderung an Figuren, Münzen oder dem Rest der Seite, keine Änderung an Spiel-Code.

## Schritte

1. `Status: in Arbeit`, committen, pushen (Domänen-Sperre PLAT prüfen: keine andere PLAT-Session `in Arbeit`).
2. `git grep -n -E "forest-background|blue-cave|warped" -- site` belegt die Fundstellen vorher.
3. Hero-Ebenen und Tiefen-Kacheln auf die neuen Dateien, Größen und Animation anpassen (Kontext).
4. `git grep` ohne Treffer in `site/`; `task pages` baut ohne Fehler, alle `url(app/…)` zeigen auf vorhandene Dateien.
5. Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(plat): Pages-Seite mit neuen Hintergründen (GEB7.4)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-07: `git grep -n -E "forest-background|blue-cave-background|warped-caves-pixel-art-pack" -- site` ohne Treffer; `task pages` ohne Fehler; jeder `url(app/…)`-Pfad in `site/index.html` existiert unter `public/` oder `dist/`.
- [ ] `task check` grün.

## Prüfen

```bash
task pages
task check
```

Mit laufender Workbench `check_run` (mit `checkout`). Keine manuellen Prüfungen; die Sicht auf Hero und Kacheln gehört 🧑 (GEB7.6, AC-08).

## Ergebnis

- **AC-07 umgesetzt und geprüft:** `site/index.html` lädt Hero-Parallax (drei Ebenen `hg-wald-fern|mitte|nah`, `background-size` 1088 × 680, `.l-lights` entfällt) und die Tiefen-Kacheln (`hg-wald|hoehle|mine-mitte-32.png`) aus `app/grafik/gebaeude/`; der Credit-Satz zu Warped Caves entfällt mit dem Pack. `git grep -E "forest-background|blue-cave|warped|l-lights" -- site` ohne Treffer; alle `url(app/…)`-Pfade existieren unter `public/`; `task pages` baut, `_site/app/grafik/gebaeude/hg-wald-fern-32.png` liegt vor.
- Sichtprüfung (Hero-Anmutung, Kachel-Ausschnitt `cover`) nicht geprüft, gehört 🧑 (AC-08). Annahme Frage 5 „umstellen“ (bei „unverändert“ wäre diese Session verworfen worden).
- Lint-Korrektur aus GEB7.1 (`errcheck` in `hintergrund_test.go`, `check:go` rot) mitcommittet. Neue Tickets: keine.
