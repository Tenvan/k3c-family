# GEB6.5 · Natur und Objekte im Spiel, alte Grafik gelöscht

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb6/5-zuordnung-aufraeumen
- **Abhängig von:** GEB6.4, GEB5 (auf develop gemergt, Domänen-Sperre CLI)
- **Tickets:** B-435, B-438
- **Kriterien:** AC-02, AC-05, AC-06

## Ziel

Im Spiel zeigen Bäume, Busch, Fels, Erzbrocken, Adern, Truhe, Portal, Ausgang und Rekrutierungslager die neuen PNGs in Maßstab 1; Baumform wählt die Position, den Look das Biom, Adern zeigen `vein:*`. Die ersetzte Pack-Grafik ist samt Verweisen gelöscht.

## Kontext

- **Spec:** GEB6 AC-02, AC-05, AC-06, › Ausnahme- und Fehlerfälle, Regeln (Aufräum-Regel, Merge-Reihenfolge); B-438/AC-01. Vorbilder: GEB3.4, GEB4.3 (Gebäude im Spiel, Aufräumen), GEB5.4 (Plantage-Pappel).
- **Heute:** `src/scenes/worldSprites.ts` führt `WORLD_TEXTURES` (Dateien unter `public/grafik/`) und `OBJECTS` (`objectSprite(id)`, Standard-Zoom 2) für `node:tree`, `node:rock`, `node:copperOre`, `node:bush`, `portal`, `exit`, `pickup:chest`, `camp:recruit`, `camp:recruit:fire`; `objectView.ts` lädt sie (`preloadWorld`), legt Animationen für `portal` und `camp:recruit:fire` an (`prepareWorld`) und setzt Sprites mit Fußpunkt (`objectImage`). `worldRenderer.ts`: `drawStatic` (Portal, Ausgang mit Beschriftung bei `G - 210`, Busch, Rekrutierungslager mit Feuer bei `ex + 90`), `createNode` (Sprite aus `node:${n.kind}`, Fahne bei y −170, Balken −190, Platzhalter-Formen), Truhe in der Pickup-Erzeugung (`pickup:chest`). Gebäude laden ihre PNGs über `buildingSprites.ts` (`BUILDING_TEXTURES`, `textureScale`: `gebaeude/…` = ×1).
- **Belegt:** Ader-Knoten haben die Art `stoneVein` … `crystalVein`; `node:<Art>` hat keinen Eintrag, die Adern erscheinen heute als Platzhalter-Kreis (`createNode`, `else`-Zweig). Neu: Zuordnung Art → Slot `vein:<stone|copper|iron|crystal>`; Textur-Schlüssel je Stufe der Datei `ader-<n>-32.png`.
- **Biom-Look (GEB6.1–GEB6.3):** Stufe = Look (1 Wald, 2 Höhle, 3 Mine), Ader-Stufe = Material, Datei `<id>-<stufe>-32.png`; die drei alten Bäume (`tanne-32.png` …) sind Form 1 im Wald-Look ohne Stufen. Die Biom-ID kommt aus `world.biome.id` (`forest`, `cave`, `mine`, `ironhold`, `crystal`); Eisenstollen und Kristallhöhle → Look Mine (Offene Frage 6), fehlende ID → Wald; der Look wird auf den höchsten Look des Objekts begrenzt (Bäume: Mine → Höhle, Busch → Wald).
- **Register und Schlüssel:** Je Slot steht die Grundform als `wahl` in `data/assets.json`; Varianten (Baumform, Look, Ader-Stufe) entstehen per Code-Konvention aus Schlüsseln wie `grafik:<id>-<stufe>` (Muster `stufen()` in `buildingSprites.ts`, per Schleife erzeugt, nicht von Hand aufgelistet). Mitziehen: `worldSprites.test.ts` (Schlüssel und Dateien), `zuordnung.test.ts` (Credits-Ausnahme `grafik/gebaeude`), die Zeilen von `node:*`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` in `docs/assets/zuordnung-welt.md` (Pack `grafik/gebaeude`, „eigen“, Muster `wall:1`).
- **Auswahl:** reine Funktionen ohne Phaser in einer neuen Datei unter `src/scenes/` mit Test: `treeTexture(x, biomeId)` (Art und Form aus der gerundeten Position, nur Formen, die für das Biom einen Look haben; stabile Zuordnung, z. B. ganzzahliger Hash der Position, kein `Math.random()`), `natureLook(biomeId)` (Stufe) und `veinSlot(kind)`. Die Plantage-Bäume aus GEB5 behalten die Pappel (GEB5.4 entscheidet an der Position des Platzes); der Aufruf in `createNode` ruft die Baumwahl nur für Waldbäume.
- **Höhen:** Baum 72 px, Fels/Erzbrocken/Busch/Truhe 32 px, Ader 64 px, Portal/Ausgang 96 px, Lager 64 px (Maßstab 1, Unterkante auf dem Boden). Fahne, Balken, Ausgang-Beschriftung und `camp:recruit:fire` (entfällt, das Bild enthält das Feuer) an die Höhen anpassen; dasselbe `createNode` zeichnet auch die Plantage-Pappel aus GEB5.4 (72 px) mit derselben Fahne und demselben Balken (Schnittstelle zu GEB5.4, die Höhe gilt für beide).
- **Aufräumen (AC-06), Kandidaten (bei der Planung belegt, vor dem Löschen erneut mit `git grep` prüfen):** `gotthicvania-swamp/umgebung/trees.png` (die übrigen Dateien des Packs prüfen), `sunnyland-tall-forest-environment/props/Plant.png` (Pack hat weitere Dateien), `portals-32-x-48/…` und `tent-8/…` und `16x16-animated-campfire/…` (ganze Packs, wenn nichts anderes sie nutzt), `k3c-paletten/props/fort-door.png` (danach ist der Pack leer). **Nicht löschen:** das Erz-Blatt (genutzt von `material:*`) und `gold-treasure-icons-16x16/icons/8.png` (genutzt von `icon:8`); nur die Register-Wahl der Slots `node:*`, `vein:*`, `pickup:chest` wechselt dort. Mitziehen: `public/grafik/index.json`, `public/grafik/CREDITS.md`, `src/tools/grafikPacks.ts` (und Pack-Test), LICENSE-Dateien, `docs/assets/zuordnung-welt.md`/`zuordnung.md`, `src/scenes/worldSprites.test.ts`, `src/tools/zuordnung.test.ts`, Mocks; `src/scenes/codemap.md` nachziehen (wie GEB4.3).

## Erlaubte Dateien

- `src/scenes/` (`worldSprites.ts`, `objectView.ts`, `worldRenderer.ts`, `buildingSprites.ts` nur falls nötig, neue Auswahl-Datei mit Test, zugehörige Tests, `codemap.md`)
- `data/assets.json` (nur `node:*`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit`)
- AC-06: die genannten alten PNGs und Pack-Ordner, `public/grafik/index.json`, `public/grafik/CREDITS.md`, `src/tools/grafikPacks.ts`, `docs/assets/zuordnung-*.md`, Tests und Mocks, die genau diese Dateien nennen
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Sim-, Protokoll- oder Spielstand-Änderung, keine Änderung an den PNGs und JSONs aus GEB6.1–GEB6.4, keine Hub-Gebäude und keine Plantage (GEB5), keine Hintergründe `bg:*` (GEB7), keine Animationen, keine neue Platzierung der Objekte.

## Schritte

1. Prüfen, dass GEB5 auf `develop` steht und GEB5.4 (CLI) nicht mehr `in Arbeit` ist; `git merge origin/develop` in `sprint/geb6`; sonst nichts tun und melden. `Status: in Arbeit`, committen, pushen.
2. Aufräum-Kandidaten mit `git grep` vor dem Löschen belegen.
3. Tests zuerst: `treeTexture(x, biomeId)` (gleiche Position → gleiche Wahl, Nachbarn wechseln, nur Formen mit Look für das Biom, Position negativ und gebrochen, Biom-ID fehlt → Wald, Mine → Höhle für Bäume), `natureLook(biomeId)` (Wald 1, Höhle 2, Mine 3, Eisenstollen und Kristallhöhle → 3, fehlend → 1), `veinSlot(kind)` (alle vier Arten, unbekannt → `null`).
4. Zuordnung: `WORLD_TEXTURES`/`OBJECTS` auf die PNGs (Maßstab 1, Textur-Schlüssel je Look und Form), `prepareWorld` ohne Animationen für Portal und Feuer; `createNode` mit Baumwahl, Look, Adern-Slot; Plantage-Bäume bleiben Pappel (Fahne und Balken auch dort passend); Fahne, Balken und Ausgang-Beschriftung an die Bildhöhe; `camp:recruit:fire` entfällt.
5. Register `data/assets.json` für die genannten Slots; `src/scenes/codemap.md` nachziehen.
6. AC-06: Alt-Grafik und Verweise löschen (Erz-Blatt und `icons/8.png` bleiben), Index, CREDITS, `grafikPacks.ts` samt Pack-Test, LICENSE, Zuordnungs-Doku, Tests und Mocks nachziehen; `git grep` ohne Treffer außerhalb historischer Doku belegen.
7. Code-Review (getrennter Agent), Ergebnis ausfüllen, `Status: fertig`, ein Commit `feat(cli): Natur und Objekte aus tools/gebaeude im Spiel (GEB6.5)`, `git fetch`, `git merge origin/develop`, push.

## Fertig, wenn

- [ ] AC-02: Register `data/assets.json` und `worldSprites.ts` ordnen `node:tree`, `node:rock`, `node:copperOre`, `node:bush`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` den neuen PNGs zu (Maßstab 1; `zuordnung.test.ts` grün).
- [ ] AC-05: Vitest für `treeTexture`, `natureLook`, `veinSlot` (siehe Schritt 3); Ader-Knoten zeigen `vein:*` (Test oder im Code belegter Pfad); Plantage-Bäume bleiben Pappel; Fahne, Balken und Ausgang-Beschriftung sitzen über der neuen Bildhöhe (im Code belegt); kein `Math.random()` in der Baumwahl.
- [ ] AC-06: alte Grafik gelöscht, Erz-Blatt und `icons/8.png` bleiben; Index, CREDITS, `grafikPacks.ts`, LICENSE, Zuordnungs-Doku, Tests, Mocks bereinigt; Suche (`git grep`) ohne Treffer außerhalb historischer Doku.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check
task check:go
git grep -n -E "gotthicvania-swamp/umgebung/trees|portals-32-x-48|tent-8|16x16-animated-campfire|k3c-paletten|Plant\.png" -- . ':!docs/sprints' ':!docs/backlog' ':!docs/funde'
```

Die Suche darf nur Treffer liefern, die zu bewusst behaltenen Dateien gehören (Begründung im Ergebnis); Treffer in Planung und Tickets (`docs/sprints`, `docs/backlog`, z. B. B-331) gelten als historisch. Mit laufender Workbench stattdessen `check_run` (mit `checkout`). Keine manuellen Prüfungen; der Blick ins Spiel gehört GEB6.6.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
