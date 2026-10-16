# GEB5.4 · Gebäude und Plantage im Spiel, alte Grafik gelöscht

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb5/4-zuordnung-aufraeumen
- **Abhängig von:** GEB5.3
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-05, AC-06

## Ziel

Im Spiel zeigen Lager, Farm, Kaserne, Treppe hoch (ein Bild) und Treppe runter die neuen PNGs in Maßstab 1, leere Plantage-Plätze einen Setzling und Plantage-Bäume die Pappel; die ersetzte Pack-Grafik ist samt Verweisen gelöscht.

## Kontext

- **Spec:** GEB5 AC-02, AC-05, AC-06, › Ausnahme- und Fehlerfälle; Muster aus GEB2.3/GEB3.4/GEB4.3 (`textureScale` = 1 für Dateien unter `gebaeude/`).
- **Zuordnung:** `src/scenes/buildingSprites.ts` › `BUILDING_TEXTURES` (LoadScene lädt genau diese) und `BASE`: `grafik:storage` → `gebaeude/lager-32.png`, `grafik:barracks` → `kaserne-32.png`, `grafik:farm` → `farm-32.png`, `grafik:stairsDown` → `treppe-runter-32.png`, neu `grafik:stairsUp` → `treppe-hoch-32.png`, `grafik:setzling` → `setzling-32.png`, `grafik:pappel` → `pappel-32.png`. `STAIRS_UP_TILES` und die drei `grafik:stairs*`-Kacheln fallen weg; `siteView.createSiteView` macht die Treppe hoch zu einem normalen `Image` (wie die Mauer in GEB3.4), der Ausschnitt `'grafik:stairsDown': [['entrance', …]]` in `siteView.ts` entfällt. `SITE_SIZE` (Platzhalter) bleibt.
- **Plantage:** Die Sim legt Plantage-Bäume als normale `tree`-Nodes an `plotX` an (`engine/sim/plantation.go`: `s.X − stripUnits + i·2·stripUnits/(slots−1)`, 6 Plätze, ±3). Neu: reine Funktion (z. B. `src/scenes/plantation.ts`, `plantationPlots(sites, nodes)`) aus gebauten Farmen, `data/buildings.json` › `farm.plantation` und den Bäumen im Snapshot: je Platz x-Position und ob dort ein Baum steht (Toleranz klein, z. B. 0,01 Units). Leere Plätze zeichnen den Setzling (eigene Views je Farm und Platz, bei jedem Sync abgleichen), Bäume auf einem Platz zeigen in `worldRenderer.createNode` die Pappel statt `node:tree`; Markierungsfahne und Abbau-Balken bleiben. Waldbäume bleiben bei `node:tree` (GEB6). Register: Slot `plantation` → `grafik/gebaeude/setzling-32.png` (Präferenz 1).
- **Fallstricke:** Stufen gibt es für diese Gebäude nicht, der Cache-Schlüssel in `updateSiteView` (Lehre GEB3.4: Stufe fehlte im Schlüssel) braucht keine Erweiterung; trotzdem prüfen, dass der Texturwechsel beim Übergang Baustelle → gebaut greift. Zeichenreihenfolge Setzling/Pappel vor der Farm prüfen (ungeprüft).
- **Register:** `data/assets.json` Slots `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `plantation` → neue PNGs (Präferenz 1).
- **Aufräumen (AC-06):** Kandidaten `gothicvania-town/props-einzeln/crate-stack.png`, `house-a.png`, `house-c.png`, `gothicvania-town/tileset-einzeln/stairs-left.png`, `stairs.png`, `stairs-right.png` (Pack `gothicvania-town` bleibt, er wird anderswo genutzt), `phantasy-dungeon-entrance/ebenen/dungeon-door.png` und `dungeon-door-skull.png` (Offene Frage 7; danach ist der Pack leer → Ordner, LICENSE, CREDITS-Zeile und Eintrag in `src/tools/grafikPacks.ts` weg). Je Kandidat `git grep` vorher und nachher; bei der Planung gesehen: `house-a`/`house-c` dienen als Fixtures in `tools/k3c-dev` (`app_assets_test.go`, `internal/assets/*_test.go`, `internal/mcpsrv/tools_assets*_test.go`, `frontend/src/assets/assets.test.ts`, `frontend/src/api/mockAssets.ts`) und in `src/tools/assetvorschauLogic.test.ts`: dort auf eine weiter vorhandene Datei umstellen, Test nicht löschen. Mitziehen: `public/grafik/index.json`, `docs/assets/zuordnung-objekte.md`, B-435 › Notizen (Ist-Stand). Historische Belege (`docs/funde/`, erledigte Sprints, Archiv, andere Tickets) bleiben.

## Erlaubte Dateien

- `src/scenes/` (`buildingSprites.ts`, `siteView.ts`, `worldRenderer.ts`, neue Regel-Datei für die Plantage mit Test, zugehörige Tests)
- `data/assets.json` (nur `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `plantation`)
- AC-06: die genannten alten PNGs und der Pack-Ordner `phantasy-dungeon-entrance`, `public/grafik/index.json`, `public/grafik/CREDITS.md`, `src/tools/grafikPacks.ts`, `docs/assets/zuordnung-*.md`, Tests, Fixtures und Mocks, die genau diese Dateien nennen (auch unter `tools/k3c-dev/`)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Sim- oder Protokolländerung, keine Änderung an den PNGs oder an `data/hub.json`/`data/buildings.json`, keine Waldbäume (GEB6), keine Burg oder Tor (GEB4).

## Schritte

1. `sprint/geb5` auschecken, `Status: in Arbeit`, committen, pushen.
2. Tests zuerst: `siteTexture` für die fünf Gebäude → `gebaeude/<id>-32.png`, Maßstab 1; `STAIRS_UP_TILES` weg; `plantationPlots` (6 Plätze an den `plotX`-Positionen, leer/Baum, Farm im Bau → keine Plätze, Waldbaum neben einem Platz → kein Plantage-Baum).
3. Zuordnung, Treppe hoch als `Image`, Setzlinge und Pappel im Renderer.
4. Register `data/assets.json`.
5. AC-06: Suche je Kandidat, löschen, Fixtures umstellen, Verweise bereinigen, Suche ohne Treffer belegen.
6. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-05: Vitest für die Zuordnung der fünf Gebäude (Maßstab 1) und für `plantationPlots`; Treppe hoch ist ein Bild; Register gesetzt, `assets.test.ts` grün.
- [ ] AC-02 (Spiel-Teil): leere Plätze zeigen den Setzling, Plantage-Bäume die Pappel (Pfad im Code und Test belegt).
- [ ] AC-06: alte Grafik gelöscht, `git grep` ohne Treffer außerhalb historischer Doku; leerer Pack ganz weg.
- [ ] `task check`, `task check:go` und `check_run dev:test` grün.

## Prüfen

```bash
task check
task check:go
task check:dev
```

Keine manuellen Prüfungen; der Blick ins Spiel gehört GEB5.5.

## Ergebnis

- **AC-05 geprüft (Vitest):** `buildingSprites.test.ts`: Lager, Kaserne, Farm, Treppe hoch, Treppe runter → `gebaeude/<id>-32.png`, Maßstab 1; Treppe hoch ein Bild (`STAIRS_UP_TILES`, `grafik:stairs*`-Kacheln und `FRAMES` in `siteView.ts` entfallen, `createSiteView` nutzt für alle Gebäude ein `Image`); `plantation.test.ts`: 6 Plätze an den `plotX`-Positionen, leer/Baum, Farm im Bau → keine Plätze, Waldbaum und Fels neben/auf einem Platz → kein Plantage-Baum. Register `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `plantation` → neue PNGs, `assets.test.ts` grün.
- **AC-02 (Spiel-Teil) umgesetzt/geprüft:** `plantation.ts` rechnet leere Plätze aus Farm-Position und `farm.plantation` (gleiche Formel wie `plotX` in Go); `worldRenderer.ts` zeichnet auf leeren Plätzen `grafik:setzling`, Bäume auf Plätzen als `grafik:pappel` (Tiefe 3, vor der Farm); Waldbäume bleiben `node:tree`. Pfad im Code und Test belegt, nicht im Browser angesehen.
- **AC-06 geprüft:** gelöscht: `crate-stack`, `house-a`, `house-c`, `stairs-left`, `stairs`, `stairs-right` (gothicvania-town), `dungeon-door`, `dungeon-door-skull` und damit der Pack `phantasy-dungeon-entrance` samt LICENSE; bereinigt: `public/grafik/index.json`, `CREDITS.md`, `src/tools/grafikPacks.ts` (+ Test 38 Packs), `docs/assets/zuordnung-objekte.md` und `-welt.md`, B-435 › Notizen, `src/scenes/codemap.md`; Fixtures und Mocks in `tools/k3c-dev` und `assetvorschauLogic.test.ts` auf `well`, `wagon`, `crate` umgestellt. `git grep` nach den Namen ohne Treffer außerhalb von `docs/funde`, `docs/sprints`, `docs/backlog` (einziger Rest: Negativ-Test auf `grafik:stairs`). Die übrigen Treppen-Kacheln des Packs (`stairs-mirror*`, `stairs-wall*`) sind unbenutzt, standen aber nicht in der Spec und bleiben.
- **Prüfen:** `check_run task:check` grün, `task:check:go` grün; `dev:test` im Ganzen rot durch zwei nicht zugehörige Fälle: `TestRepo` (GEB7 Entwurf ohne Sessions, schon auf `develop`, neues Ticket B-448) und `TestSprintDomaeneFolgtSessions` (Windows-Rename „Access denied“, einzeln wiederholt grün, Flake); die geänderten `TestAssets*` grün.
- **Review:** getrennter Reviewer (Formel gegen Go, Layer-Sync, tote Verweise, Regeln): keine schweren Befunde.
- Manuell geprüft hat niemand (Sichtprüfung gehört GEB5.5/Ralf). Neue Tickets: B-448.
