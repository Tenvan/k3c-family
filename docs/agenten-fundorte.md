# Fundorte für Agenten

Zentrale Karte, wo was liegt. Die Agenten unter `.claude/agents/` verweisen hierher und nennen selbst nur ihre wichtigsten Pfade; neue oder verschobene Pfade werden **nur hier** gepflegt. Tiefer: `codemap.md` der Wurzel und je Ordner.

## Übergreifend

- Regeln für Claude: `CLAUDE.md`; Prozess: `docs/arbeitsweise.md`; Begriffe: `docs/glossar.md`
- Entscheidungen: `docs/decisions/` (005 Papierschnitt, 008 natives Spiel, 009 nur PC, 010 Asset-Pakete je Welt, Entwurf)
- Spielregeln: `docs/game-design.md`, `docs/rules/*.md`; Stand und nächste Schritte: `docs/roadmap.md`
- Planung (nur lesen, ändern nur `plan_*`): Tickets `docs/backlog/` (Archiv `docs/backlog/archiv/`), Sprints `docs/sprints/{aktiv,geplant,erledigt}/`, Projekte `docs/projekte/`, Vorlagen `docs/vorlagen/`
- Werte: `data/*.json` (einzige Quelle), Laden `data/embed.go`
- Golden-Dateien: `testdata/golden/`, Grafik-Golden `public/grafik/<gruppe>/golden.json`
- Repo-Regeln als Test: `tests/projectRules.test.ts`, Planung `tests/planning.test.ts`, Oberflächen `tests/atomicDesign.test.ts`

## Kampf

- Gegner: `engine/sim/enemies*.go` (Wellen `enemies_waves.go`, Wachen `enemies_guard.go`, Eigenschaften `enemies_traits.go`), Aggression `engine/sim/aggression*.go`
- Truppen: `engine/sim/unit_*.go` (Verhalten und Zielwahl inline: Bogenschützen `unit_archer.go`, Krieger `unit_warrior.go`, Takt `unit_common.go`), Bosse `engine/sim/boss*.go`
- Werte: `data/enemies.json`, `data/troops.json`, `data/waves.json`, `data/bosses.json`, `data/guards.json`
- Regeln: `docs/rules/gegner.md`, `docs/rules/bosse.md`
- Anzeige: `native/k3c-game/hud/kampf.go`, Effekte `native/k3c-game/fx/`
- Dev-Aktionen und Stresstest: `engine/room/dev_combat.go`, `engine/sim/dev*.go`, `native/k3c-game/game/stress.go`, `data/stresstests.json`

## Simulation

- Kern: `engine/sim/` (Welt `world*.go`, Insel und Stufen `island*.go`, Gebäude `building*.go`, Wirtschaft `economy*.go`, Monarch und Skills `monarch*.go`, `skills*.go`, Ereignisse `events*.go`, Spielstand `*save*.go`)
- Zufall nur `engine/rng`; Level-Generator `engine/level/`; Raum mit Stufen und Spielstand `engine/room/`; Ablage `engine/store/`
- Engine-CLI: Logik `engine/simcli/` (das Studio ruft sie im Prozess, seit WZ63.5), Hülle `cmd/k3c-sim/`; gemeinsame Aufrufe `engine/enginetools/`; Sim-Code nur im eigenen Worktree (Agent `k3c-sim-dev`, B-925)
- Regeln: `docs/rules/stufen.md`, `docs/rules/welten.md`, Protokoll `docs/protocol.md`

## Balancing

- Werte: `data/economy.json`, `data/troops.json`, `data/enemies.json`, `data/waves.json`, `data/buildings.json`, `data/monarch.json`, `data/difficulty.json`, `data/bosses.json`
- Ziele: `data/balance-targets.json`, `data/balance-hints.json`, `docs/rules/zielkorridore.md`, `docs/rules/wirtschaft.md`
- Auswertung (Bots, Kennzahlen): `engine/balance/`; Messungen `docs/balancing/`, `docs/messung/`; Studio-Seite `tools/k3c-studio/frontend/src/balancing/`, Backend `tools/k3c-studio/internal/balance/` und `simrun/` (MCP `mcp__k3c-studio__sim_test` usw.)
- **Sim- und Balancing-Läufe sind gesperrt** (🧑 2026-11-11): nur lesen. Das Balancing schreibt Quelldateien nur unter `data/worlds/` (Wächter `balance.Guard`, WZ63.7); Code- oder andere Datenänderungen erst nach Plan im Chat und Freigabe von 🧑.

## Welten

- Welt-Dateien `data/worlds/`, Biome `data/biomes/`, `data/island-sizes.json`, `data/hub.json`, `data/ruins.json`
- Generator `engine/level/`, Insel-Code `engine/sim/island*.go`, Welt-Editor `tools/k3c-studio/frontend/src/world/`
- Gebäudeplaner `tools/k3c-studio/internal/hubplan/`, Seite `tools/k3c-studio/frontend/src/planner/`

## Grafik

- Papierschnitt: Specs und Indizes `public/grafik/<gruppe>/` (`index.paper.json`, `golden.json`), Zuordnung `data/paper.json`
- Renderer und Verträge `tools/paper/`, Standards `docs/standards/paper-spec.md`, `docs/standards/paper-mcp.md`
- Rig `native/k3c-rig/` (`RIG.md`), Laden im Spiel `native/k3c-game/art/`, Figuren `native/k3c-game/actors/`
- Assets ändern nur MCP `assets_*`; Asset-Logik `tools/assetkit/`
- Im Code gezeichnete HUD-Grafik ohne Asset (vorläufig): `native/k3c-game/hud/purse.go`, `native/k3c-game/hud/icons.go`

## Sound

- Slots `data/sounds.json`, Atlas `public/audio/` (`atlas.json`), Werkzeug `tools/soundatlas/`
- Wiedergabe `native/k3c-game/audio/`, Lizenzen `native/k3c-game/menu/licensefile/`

## HUD, Menüs, Eingabe

- HUD `native/k3c-game/hud/` (Layout `layout.go`, Radar `radar.go`, Skill-Menü `skillmenu.go`)
- Menüs `native/k3c-game/menu/`, Eingabe `native/k3c-game/input/`, Spielablauf `native/k3c-game/game/`, Sicht `native/k3c-game/view/`
- Durchläufe `native/k3c-game/e2e/`, Regeln `docs/rules/bedienung.md`

## Workbench

- App `tools/k3c-workbench/` (Wails, `uiapi.go`), MCP-Server `tools/k3c-workbench/internal/mcpsrv/` (Anleitung `instructions.md`)
- Planung `tools/k3c-workbench/internal/planning/`, Hook `tools/hooks/planGuard.ts`
- Oberfläche `tools/k3c-workbench/frontend/src/`, Bausteine `tools/webkit/`, Seiten-Spec `docs/standards/workbench-seiten.md`
