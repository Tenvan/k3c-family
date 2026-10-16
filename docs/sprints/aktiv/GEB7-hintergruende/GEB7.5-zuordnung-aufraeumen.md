# GEB7.5 · Hintergründe im Spiel, alte Grafik gelöscht

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb7/5-zuordnung-aufraeumen
- **Abhängig von:** GEB7.4, GEB6 (auf develop gemergt, Domänen-Sperre CLI)
- **Tickets:** B-435
- **Kriterien:** AC-05, AC-06

## Ziel

Alle fünf Biome zeigen im Spiel ihre drei neuen Hintergrund-Schichten in Maßstab 1; Register, Zuordnungs-Doku und Tests stimmen überein; die ersetzte Pack-Grafik ist samt Verweisen gelöscht.

## Kontext

- **Spec:** GEB7 AC-05, AC-06, › Anforderungen (Spiel, Aufräumen), › Regeln (Merge-Reihenfolge GEB5 → GEB6 → GEB7, Aufräum-Regel), › Ausnahme- und Fehlerfälle, › Offene Fragen 1, 4, 8.
- **Heute:** `src/scenes/worldSprites.ts`: `WORLD_TEXTURES` mit `bg:forest-back|-lights|-middle|-front`, `bg:cave`, `bg:mine-back|-far|-middle` (Dateien unter `forest-background/…`, `blue-cave-background/…`, `warped-super-grotto-escape-pack/…`); `BACKGROUNDS` (Wald Zoom 3 mit vier Schichten und Scroll 0,15/0,25/0,35/0,5, Höhle Zoom 2 mit einer Schicht 0,2, Mine Zoom 2 mit drei Schichten 0,1/0,25/0,45; `biomeBackground(biome)`). `objectView.backgroundLayers` legt je Schicht einen `TileSprite` (Unterkante `GROUND_Y`, Skalierung = Zoom, Scroll-Faktor); `stageView` zeichnet ohne Hintergrund zwei Silhouetten und spiegelt die Schichten im Fluss. Register `data/assets.json` › `bg:forest`, `bg:cave`, `bg:mine`, `bg:ironhold`, `bg:crystal` (`wahl`: forest nur Schicht 1, mine nur `back`, ironhold und crystal leer). Test `worldSprites.test.ts` prüft `forest`, `cave`, `mine` und `biomeBackground('sumpf')` = `null`; `zuordnung.test.ts` verlangt `bg:${id}` je Biom in `docs/assets/zuordnung-welt.md`.
- **Neu:** Texturen `bg:<biom>:<n>` (n = 1 hinten … 3 vorn) aus `gebaeude/hg-<präfix>-<fern|mitte|nah>-32.png`, Zoom 1 für alle fünf Biome; Biom → Präfix: `forest` → `wald`, `cave` → `hoehle`, `mine` → `mine`, `ironhold` → `eisenstollen`, `crystal` → `kristall`. Scroll-Faktoren (hinten < vorn): Wald 0,15 / 0,35 / 0,5, Höhle 0,1 / 0,2 / 0,4, Mine 0,1 / 0,25 / 0,45, Eisenstollen und Kristallhöhle wie Mine. Register: je Schicht ein Slot `bg:<biom>:<n>` (Kategorie `umgebung`, `wahl` = die PNG, Pack `grafik/gebaeude`, Lizenz „eigen“) ersetzt `bg:<biom>`; `zuordnung.test.ts` erwartet dann `bg:${id}:${n}` für n = 1–3; `zuordnung-welt.md` führt je Schicht eine Zeile (Muster der Zeilen `hub:<n>`). Schlüssel und Dateien per Schleife erzeugen (Muster `stufen()` in `buildingSprites.ts`), nicht von Hand auflisten. `biomeBackground` bleibt `null` für unbekannte Biome.
- **GEB6 zuerst:** GEB6.5 ändert in `worldSprites.ts` `OBJECTS`/`WORLD_TEXTURES` (Objekte) und löscht Packs; GEB7.5 wartet, bis GEB6 auf `develop` steht (`git merge origin/develop` in `sprint/geb7`), sonst nichts tun und melden. Domänen-Sperre CLI: keine andere CLI-Session `in Arbeit` (`git ls-remote --heads origin 'sprint/*'`, Session-Dateien lesen).
- **Aufräumen (AC-06), Kandidaten (bei der Planung belegt, vor dem Löschen erneut mit `git grep` prüfen):**
  - `warped-super-grotto-escape-pack` ganz (heute nur `bg:mine-*`).
  - `forest-background`, `blue-cave-background`, `warped-caves-pixel-art-pack` ganz, **sobald `site/` sie nicht mehr lädt** (GEB7.4; sonst `verschoben`).
  - `sunnyland-tall-forest-environment/ebenen/*` (nur als Alternative dokumentiert); `props/Plant.png` bleibt, falls GEB6 es nicht schon ersetzt hat; hat GEB6 den Pack ganz entfernt, entfällt der Punkt.
  - Mitziehen: `public/grafik/index.json`, `public/grafik/CREDITS.md` (Tabellenzeilen **und** Fließtext zu Warped Caves und Super Grotto), `src/tools/grafikPacks.ts` (inkl. Beschreibung von `sunnyland-tall-forest-environment`: „Hintergrund Wald“, „Höhe der Hintergründe“), Lizenztext `lic.warped` in `src/tools/texts.de.ts` und `texts.en.ts` samt Lizenzseite `lizenzen.html` (Zeile mit `data-t-html="lic.warped"`, nur wenn der Pack wegfällt), `docs/assets/zuordnung.md` (Pack-Tabelle, Ergebnis-Zeile) und `zuordnung-welt.md`, `src/tools/credits.test.ts` (ohne `warped-caves-pixel-art-pack`), `src/scenes/worldSprites.test.ts`, `src/tools/zuordnung.test.ts`, `src/scenes/codemap.md`, `tools/k3c-dev`-Fixtures und Mocks (Suche).
  - **Nicht löschen:** alles, was ein Treffer in `src/`, `data/`, `tests/`, `tools/` oder `site/` noch nennt; historische Belege (`docs/funde/`, erledigte Sprints, Archiv, andere Tickets).
- **Fallstricke:** Der Fluss spiegelt die Schichten über `frame.height` und `tilePositionY`: die Unterkante der Textur muss an der Bodenlinie liegen (AC-02 sichert das). Splitscreen: jede Kamera verschiebt die Schichten mit dem eigenen Scroll-Faktor, an der Logik ändert sich nichts. Zoom 1 statt ×2/×3: `TileSprite`-Breite und Höhe aus der Textur, nichts hartcodieren.
- **Team:** Verwendungssuche je Kandidat parallel (Explorer), Umsetzung durch einen Agenten, Review getrennt.

## Erlaubte Dateien

- `src/scenes/` (`worldSprites.ts`, `objectView.ts`, `stageView.ts` nur falls nötig, Tests, `codemap.md`)
- `data/assets.json` (nur `bg:*`)
- AC-06: die genannten Packs und Dateien unter `public/grafik/`, `public/grafik/index.json`, `public/grafik/CREDITS.md`, `src/tools/grafikPacks.ts`, `src/tools/texts.de.ts`, `src/tools/texts.en.ts`, `src/tools/credits.test.ts`, `src/tools/zuordnung.test.ts`, `lizenzen.html`, `docs/assets/zuordnung*.md`, Tests und Mocks, die genau diese Dateien nennen
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Änderung an den Hintergrund-PNGs und JSONs (GEB7.1–GEB7.3), an Scroll-Logik, Fluss-Spiegelung, Kamera oder `data/biomes/*.json`, keine Objekte und Gebäude (GEB5, GEB6), keine Sim-, Protokoll- oder Spielstand-Änderung, kein Löschen, das ein Treffer außerhalb historischer Doku noch nennt, keine Animationen.

## Schritte

1. Prüfen, dass GEB6 auf `develop` steht und keine CLI-Session auf einem anderen Sprint-Branch `in Arbeit` ist; `git merge origin/develop` in `sprint/geb7`; sonst nichts tun und melden. `Status: in Arbeit`, committen, pushen.
2. Aufräum-Kandidaten mit `git grep` vor dem Löschen belegen (Treffer je Kandidat im Ergebnis festhalten).
3. Tests zuerst: `worldSprites.test.ts` für alle fünf Biome (drei Schichten, Zoom 1, Scroll hinten < vorn, Dateien existieren, `sumpf` → `null`); `zuordnung.test.ts` auf `bg:${id}:${n}`.
4. Zuordnung: `WORLD_TEXTURES` und `BACKGROUNDS` per Schleife auf die neuen PNGs; Register `data/assets.json`: die fünf Slots `bg:<biom>` durch 15 Slots `bg:<biom>:<n>` ersetzen; `zuordnung-welt.md` nachziehen; `src/scenes/codemap.md`.
5. AC-06: Packs und Verweise löschen (Kontext), `git grep` nachher belegen.
6. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(cli): Hintergründe aus tools/gebaeude im Spiel (GEB7.5)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-05: Vitest `worldSprites.test.ts` grün über alle fünf Biome (je drei Schichten, Zoom 1, Scroll hinten < vorn, unbekanntes Biom `null`); Register und `zuordnung-welt.md` stimmen überein (`zuordnung.test.ts` grün).
- [ ] AC-06: alte Grafik gelöscht (Kandidaten oben), Index, CREDITS (Tabelle und Fließtext), `grafikPacks.ts`, `lic.warped` samt Lizenzseite, Zuordnungs-Doku, Tests und Mocks bereinigt; `git grep` ohne Treffer außerhalb historischer Doku (Befehl unter „Prüfen“); `task check`, `task check:go`, `task check:dev` grün.
- [ ] `task build` grün (keine fehlende Textur im Build).

## Prüfen

```bash
task check
task check:go
task check:dev
git grep -n -E "forest-background|blue-cave-background|warped-super-grotto|warped-caves|parallax-forest|startcavebg|lic\.warped" -- . ':!docs/sprints' ':!docs/backlog' ':!docs/funde'
```

Die Suche darf nur Treffer liefern, die zu bewusst behaltenen Dateien gehören (Begründung im Ergebnis); Treffer in Planung und Tickets gelten als historisch. Mit laufender Workbench `check_run` (mit `checkout`). Keine manuellen Prüfungen; der Blick ins Spiel gehört GEB7.6.

## Ergebnis

–
