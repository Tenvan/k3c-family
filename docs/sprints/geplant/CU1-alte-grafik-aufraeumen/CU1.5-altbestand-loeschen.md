# CU1.5 · Altbestand, Generatoren und Tasks löschen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** CU1.3, CU1.4
- **Tickets:** B-700
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Altbestand ist gelöscht: alte Packs, Atlas, `public/sprites/`, `public/grafik-db32/`, die alten Gebäude-PNGs, `data/assets.json`, die Generatoren `tools/buildings`, `tools/atlas`, `tools/pipeline`, `tools/spritegen` mit ihren Tasks, Docker- und CI-Schritten; `tools/soundatlas` liest `data/sounds.json`; Doku und CREDITS sind bereinigt. `task check:all` und `task buildall` grün.

## Kontext

- Vorher: CU1.2–CU1.4 haben alle Leser im Code entfernt. Erst suchen, dann löschen; Befehl und Ergebnis ins Ergebnis.
- Dateien (git ls-files 2026-11-07): `public/grafik-db32/` + `public/atlas/` + `public/sprites/` = 191 Dateien; `public/grafik/` außerhalb der Papier-Gruppen (`bosse`, `figuren`, `gebaeude-paper`, `gegner`, `hintergruende`, `icons`, `objekte`, `reittiere`, `truppen`) = 1078 Dateien, davon 871 in `public/grafik/gebaeude/`, dazu `public/grafik/index.json`. `public/grafik/stil.paper.json` bleibt.
- Generatoren und Tasks: `tools/Taskfile.yml` (`atlas`, Gebäude-Task, `pipeline`, `soundatlas`), Wurzel-`Taskfile.yml` Zeile 175 (`check:diff`-Muster `tools/atlas/*`, `tools/buildings/*`, `tools/pipeline/*`), `Dockerfile` (Stufe `atlas`, `COPY --from=atlas`), `tools/k3c-workbench/task-freigaben.json`, `.serena/memories/suggested_commands.md`, CI `.github/workflows/ci.yml` prüfen.
- `tools/soundatlas/atlas.go` Zeilen 87, 95, 163 lesen `data/assets.json` → `data/sounds.json`; der Kommentar im erzeugten `public/audio/atlas.json` ändert sich, Offsets und Dateien bleiben gleich.
- Doku: `docs/assets/zuordnung.md`, `zuordnung-objekte.md`, `zuordnung-welt.md`, `pipeline-liste.json`, `stil-bibel.md` (Abschnitte zu DB32/Packs), `docs/standards/gebaeude-grafik.md` (auf Papier verweisen oder löschen), `public/grafik/CREDITS.md`, `public/sprites/CREDITS.md`, `LICENSE-ASSETS`, `README.md`, `docs/release/LIES-MICH.txt`, `docs/release/itch-io.md`, `docs/glossar.md`, `docs/game-design.md`, `docs/arbeitsweise.md`, `CLAUDE.md` (Befehle `gen:gebaeude`, Struktur `tools/atlas`, `tools/buildings`), Codemaps (`codemap.md`, `tools/codemap.md`, `data/codemap.md`, `.codemap/state.json`), `engine/watch/watch_test.go` (Pfad `public/sprites`).
- Offene Tickets, die nur alte Pixel-Grafik betreffen (z. B. B-462, B-570, B-571, B-641, B-685, Sprint GRJ2): in der D-Zeile auflisten; Verwerfen entscheidet Ralf in der Abnahme.

## Erlaubte Dateien

- `public/grafik/` (nur Nicht-Papier-Ordner, `index.json`, `CREDITS.md`), `public/grafik-db32/`, `public/atlas/`, `public/sprites/`
- `data/assets.json`
- `tools/buildings/`, `tools/atlas/`, `tools/pipeline/`, `tools/spritegen/`, `tools/soundatlas/`, `tools/Taskfile.yml`, `tools/codemap.md`, `tools/k3c-workbench/task-freigaben.json`
- `Taskfile.yml`, `Dockerfile`, `.github/workflows/*.yml`, `.serena/memories/suggested_commands.md`, `.codemap/state.json`
- `codemap.md`, `data/codemap.md`, `CLAUDE.md`, `README.md`, `LICENSE-ASSETS`, `docs/release/*`, `docs/assets/*`, `docs/standards/gebaeude-grafik.md`, `docs/glossar.md`, `docs/game-design.md`, `docs/arbeitsweise.md`
- `engine/watch/watch_test.go`, `public/audio/atlas.json`
- `docs/backlog/B-700-alte-pixel-grafik-aufraeumen.md`, `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Papier-Gruppen, `public/grafik/stil.paper.json`, `public/audio/` (außer neu erzeugtem `atlas.json`), `site/`, `docs/sprints/erledigt/`, `docs/decisions/`; Tickets verwerfen.

## Schritte

1. Suche wie in der Sprint-README › Beispiele, dazu `rg -l "tools/(buildings|atlas|pipeline|spritegen)"`; jeder Treffer außerhalb der erlaubten Dateien → stoppen, D-Zeile.
2. `tools/soundatlas` auf `data/sounds.json`, `task soundatlas` und Diff von `public/audio/atlas.json` prüfen (nur Kommentar).
3. Dateien und Ordner löschen (`git rm -r`), Tasks, Dockerfile, CI und `check:diff`-Muster bereinigen.
4. Doku, CREDITS, `LICENSE-ASSETS`, `CLAUDE.md`, Codemaps bereinigen.
5. `task check:all`, danach `task buildall` (laufende EXE vorher schließen), beide im Hintergrund; Docker-Build nur, wenn Docker verfügbar (sonst im Ergebnis als übersprungen nennen).

## Fertig, wenn

- [ ] AC-01: Suche aus der README › Beispiele und `rg -l "tools/(buildings|atlas|pipeline|spritegen)"` ohne Treffer außerhalb von `docs/sprints/erledigt/`, `docs/backlog/archiv/`, `docs/decisions/`; die Ordner fehlen.
- [ ] AC-02: `task check:all` und `task buildall` grün.
- [ ] AC-03: `public/grafik/CREDITS.md` und `LICENSE-ASSETS` ohne Grafik-Packs; Sound-Credits unverändert.
- [ ] `public/audio/atlas.json` unterscheidet sich nur im Kommentar.

## Prüfen

```bash
task check:diff
task check:all
task buildall
```

Review-Agent (Sonnet) vor dem Commit, weil Taskfile, Dockerfile und CI geändert werden.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
