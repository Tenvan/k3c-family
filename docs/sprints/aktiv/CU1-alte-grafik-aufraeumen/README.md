# CU1 · PAP, CLI, DEV, INF · Cleanup zum Schluss: alte Pixel-Grafik, Register und Gebäude-Renderer entfernen

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP, CLI, DEV, INF
- **Reife:** bereit
- **Tickets:** B-700, B-701
- **Start-Commit:** 89ad1b81
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Letzter Sprint des Projekts PAP, erst nach Ralfs Abnahme in PS1 (die Sessions CU1.1 und CU1.4 hängen von PS1 ab). Umfang: B-700 und D-US2-02 (B-701). Stand `origin/feature/pap` eff4d144 (bis US2): Das Spiel zeichnet Figuren, Truppen, Gegner, Bosse, Reittiere, Gebäude, Hintergründe, Natur, Objekte und Icons aus den Papier-Gruppen. Erfasster Altbestand (Suche `git grep` am 2026-11-07, ohne `docs/sprints/erledigt/`, `docs/backlog/archiv/`, `docs/decisions/`):

| Altbestand | Wer lädt oder erzeugt ihn noch | Session |
|---|---|---|
| Gerüste `gebaeude/geruest-<klasse>-32.png`, `hub-geruest`, Plantage `setzling-32.png`, `pappel-niedrig-32.png` (D-US2-02) | `src/scenes/render/buildingSprites.ts`, `scaffoldView.ts`, `hubGrowth.ts`, `worldRenderer.ts` | CU1.1, CU1.2 |
| `data/sprites.json` (`sheets`, `players`, `troops`, `enemies`, `mounts`, `riderWaist`) | `src/scenes/render/sprites.ts` (Maße, Höhe, Deckkraft, `preloadSheet` aus `public/sprites/`), `paperAssets.ts`, `data/embed_test.go`, Kommentar `engine/sim/monarch_data.go`, Tests `mountData`, `monarchAnim`, `mountPose`, `tests/sprites.test.ts` | CU1.2 |
| `data/assets.json` (288 Slots: 256 Grafik, 32 Sound) | Grafik: `src/scenes/render/buildingSize.ts`, `siteView.ts`; Sound: `src/audio/events.ts`, `ui.ts`, `music.ts`, `src/tools/soundtest*.ts`, `tools/soundatlas`; Workbench: `tools/assetkit/register.go`, `catalog.go`, `zuordnung.go`, `slotdetails.go` | CU1.2, CU1.3, CU1.4, CU1.5 |
| `public/sprites/` (Figuren-Packs, CREDITS) und `public/atlas/` | `sprites.ts` (`preloadSheet`), `src/scenes/loadLogic.test.ts`, `tools/atlas`, `tools/assetkit/atlasread.go`, `Dockerfile` (Stufe `atlas`), `tools/Taskfile.yml` › `atlas`, `src/tools/credits.ts`, `tools/spritegen` | CU1.2, CU1.3, CU1.4, CU1.5 |
| `public/grafik-db32/` und alte Packs in `public/grafik/` (alle Ordner ohne `*.paper.json`, dazu `index.json`, 871 PNGs in `public/grafik/gebaeude/`) | `src/scenes/render/worldSprites.ts`, `natureSprites.ts` (Restpfade), `src/tools/pipeline.ts`, `src/tools/grafikPacks.test.ts`, `tools/pipeline`, `tools/buildings`, `tests/projectRules.test.ts` | CU1.2, CU1.3, CU1.5 |
| Generatoren und Tasks: `tools/buildings` (`gen:gebaeude`), `tools/atlas` (`gen:atlas`/`atlas`), `tools/pipeline`, `tools/spritegen` | `tools/Taskfile.yml`, `Taskfile.yml` (`check:diff`-Muster Zeile 175), `Dockerfile`, `tools/k3c-workbench/task-freigaben.json`, `.serena/memories/suggested_commands.md` | CU1.5 |
| Doku: `docs/assets/zuordnung*.md`, `docs/assets/stil-bibel.md`, `docs/standards/gebaeude-grafik.md`, `public/grafik/CREDITS.md`, `public/sprites/CREDITS.md`, `LICENSE-ASSETS`, `README.md`, `docs/release/*`, `CLAUDE.md`, Codemaps | – | CU1.5 |

## Ziel

Im Repo gibt es keine alte Pixel-Grafik und keinen Code mehr, der sie erzeugt oder lädt.

Am Ende sichtbar: `public/grafik-db32/`, `public/atlas/`, `tools/buildings/` fehlen; `task check:all` grün.

## Beteiligte und Zielgruppen

Ralf (🧑) gibt nach PS1 frei und nimmt ab; autonome Agenten führen CU1.1–CU1.6 aus; Spieler merken nichts außer Papier-Gerüsten und -Plantage.

## Anforderungen

B-700 › Anforderungen. Dazu:

- D-US2-02 zuerst: Gerüste und Plantage bekommen Papier-Assets (CU1.1), bevor ihre alten PNGs fallen.
- Werte, die heute in `data/sprites.json` stehen und das Spiel noch braucht (Größe, Höhe über Boden, Deckkraft, Reittier-Schlüssel, `riderWaist`), ziehen nach `data/paper.json`; Atlas-Frames (`sheets`) entfallen.
- Die 32 Sound-Slots ziehen aus `data/assets.json` nach `data/sounds.json` (gleiches Slot-Format); Sound klingt danach unverändert, der Sound-Atlas bleibt bytegleich.
- Löschen erst, wenn die Suche keinen Nutzer mehr findet; der Beleg (Suchbefehl und Ergebnis) steht im Session-Ergebnis.

## Nicht-Ziele

Neue Grafik außer Gerüsten und Plantage; Sound-Dateien und -Auswahl; `site/` (GitHub Pages, eigene Bild-Kopien in `site/img/`); Umbenennen der Gruppe `gebaeude-paper`; die Recherche-Tools der Workbench (`assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import`) und ihre Dateien `data/assets-recherche.json`, `data/assets-vorschlaege.json`, `data/asset-prompts.json`; Kleinbefunde D-US2-04.

## Regeln und Einschränkungen

Gelöscht wird nur, was per Suche nachweislich ungenutzt ist. Sessions derselben Domäne nacheinander; CU1.4 (DEV, Lane `mcp`) läuft parallel zu CU1.1–CU1.3. Assets nur über MCP `assets_*`. Ein Pack, der doch noch gebraucht wird, bleibt liegen und bekommt eine D-Zeile in B-701.

## Beispiele

- `git grep -l -E "grafik-db32|atlas/atlas.json|tools/buildings|data/assets.json|data/sprites.json|public/sprites" -- ':!docs/sprints/erledigt' ':!docs/backlog/archiv' ':!docs/decisions'` → keine Treffer.
- Ein Gebäude im Bau zeigt das Papier-Gerüst seiner Größenklasse; die Plantage zeigt Papier-Setzling und -Pappel.
- Der Münz-Sound spielt wie vorher, `public/audio/atlas.json` ist nach `task soundatlas` unverändert.

## Ausnahme- und Fehlerfälle

- Ein Papier-Asset fehlt für einen Schlüssel → das Spiel warnt einmal im Log (🤒) und zeichnet nichts, kein Rückfall auf Pixel-Grafik.
- Ein Werkzeug außerhalb des Plans hängt doch an einer Datei → nicht löschen, D-Zeile in B-701 mit Fundstelle.

## Akzeptanzkriterien

- **AC-01** B-700/AC-01: kein alter Pack, kein Atlas, kein `public/sprites/`, kein `public/grafik-db32/`, kein `tools/buildings`, `tools/atlas`, `tools/pipeline`, `tools/spritegen`, kein `data/assets.json` und `data/sprites.json` mehr; die Suche aus „Beispiele“ ohne Treffer.
- **AC-02** `task check:all` und `task buildall` grün.
- **AC-03** B-700/AC-03: `CREDITS.md` und `LICENSE-ASSETS` nennen keine Grafik-Packs mehr; Sound-Credits bleiben.
- **AC-04** D-US2-02: Gerüste (`geruest-<klasse>` je Größenklasse, `hub-geruest`) und Plantage (`setzling`, `pappel-niedrig`) sind Papier-Assets und werden im Spiel daraus gezeichnet.
- **AC-05** Spiel und Werkzeugseiten laden nur Papier-Assets; Maße und Reittier-Schlüssel kommen aus `data/paper.json`, Sound-Slots aus `data/sounds.json`, Sound unverändert.

## Offene Fragen

Autonom mit Default entschieden (D-Zeilen D-CU1-01 … in B-701 beim Abarbeiten):

- Sound-Slots: eigene Datei `data/sounds.json` statt eines auf Sound geschrumpften `data/assets.json` (Suche bleibt eindeutig).
- `data/sprites.json`: noch gebrauchte Werte nach `data/paper.json`, `sheets` entfällt.
- `tools/pipeline` und `tools/spritegen` fallen mit, weil sie nur alte Pixel-Grafik erzeugen; `tools/soundatlas` bleibt und liest `data/sounds.json`.
- Recherche-Tools bleiben; ihre Slot-Prüfung gegen das Register entfällt, `id` ist frei.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| CU1.1 | `CU1.1-geruest-plantage-papier.md` | Umsetzung | autonom | fertig |
| CU1.2 | `CU1.2-spiel-nur-papier.md` | Umsetzung | autonom | fertig |
| CU1.3 | `CU1.3-werkzeugseiten-ohne-register.md` | Umsetzung | autonom | fertig |
| CU1.4 | `CU1.4-workbench-ohne-register.md` | Umsetzung | autonom | fertig |
| CU1.5 | `CU1.5-altbestand-loeschen.md` | Umsetzung | autonom | offen |
| CU1.6 | `CU1.6-review.md` | Review | autonom | offen |

- CU1.1 PAP · Sonnet · Lane `mg`: Papier-Assets Gerüste und Plantage (AC-04), nach PS1
- CU1.2 CLI · Sonnet: Spiel lädt nur Papier, `data/sprites.json` → `data/paper.json`, Sound-Slots → `data/sounds.json` (AC-04, AC-05)
- CU1.3 CLI · Sonnet: Werkzeugseiten `src/tools` ohne Register, Packs und Atlas (AC-05)
- CU1.4 DEV · Sonnet · Lane `mcp`: `tools/assetkit` und Workbench ohne Register und Atlas (AC-01), parallel, nach PS1
- CU1.5 INF · Sonnet: Altbestand, Generatoren, Tasks, Dockerfile, Doku, CREDITS löschen (AC-01–AC-03)
- CU1.6 Review · Opus: alle

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
