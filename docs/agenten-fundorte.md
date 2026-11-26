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

- Gegner: `rust/crates/k3c-sim/src/enemies/` (Wellen `waves.rs`, Wachen `guard.rs`, Eigenschaften `traits.rs`), Aggression in `enemies/` (Suche `aggression`)
- Truppen: `rust/crates/k3c-sim/src/unit/` (Bogenschützen `archer.rs`, Krieger `warrior.rs`, Takt `mod.rs`), Bosse `rust/crates/k3c-sim/src/boss/`
- Werte: `data/enemies.json`, `data/troops.json`, `data/waves.json`, `data/bosses.json`, `data/guards.json`
- Regeln: `docs/rules/gegner.md`, `docs/rules/bosse.md`
- Anzeige: `rust/crates/k3c-game/src/hud/kampf.rs`, Effekte `rust/crates/k3c-game/src/fx/`
- Dev-Aktionen und Stresstest: `rust/crates/k3c-room/src/room/dev_combat.rs`, `rust/crates/k3c-sim/src/dev/`, `rust/crates/k3c-game/src/game/stress.rs`, `data/stresstests.json`

## Simulation

- Kern: `rust/crates/k3c-sim/src/` (Welt `world/`, Insel und Stufen `island/`, Gebäude `building/`, Wirtschaft `economy/`, Monarch und Skills `monarch/`, `skills/`, Ereignisse `events/`, Spielstand `state/save*.rs`, `island/save.rs`)
- Zufall nur `rust/crates/k3c-rng`; Level-Generator `rust/crates/k3c-level/`; Raum mit Stufen und Spielstand `rust/crates/k3c-room/src/room/`; Ablage `rust/crates/k3c-room/src/store/`
- Läufe: `k3c-game sim <run|level|balance|world|hub|save>` (`rust/crates/k3c-game/src/runs/`, Doku `docs/standards/k3c-game-laeufe.md`; das Studio startet die EXE); Go-CLI `cmd/k3c-sim` ist entfernt (RU41)
- Regeln: `docs/rules/stufen.md`, `docs/rules/welten.md`, Protokoll `docs/protocol.md`

## Balancing

- Werte: `data/economy.json`, `data/troops.json`, `data/enemies.json`, `data/waves.json`, `data/buildings.json`, `data/monarch.json`, `data/difficulty.json`, `data/bosses.json`
- Ziele: `data/balance-targets.json`, `data/balance-hints.json`, `docs/rules/zielkorridore.md`, `docs/rules/wirtschaft.md`
- Auswertung (Bots, Kennzahlen): `rust/crates/k3c-balance/`; Messungen `docs/balancing/`, `docs/messung/`; Studio-Seite `tools/k3c-studio/frontend/src/balancing/`, Backend `tools/k3c-studio/internal/balance/` und `simrun/` (MCP `mcp__k3c-studio__sim_test` usw.)
- **Sim- und Balancing-Läufe sind gesperrt** (🧑 2026-11-11): nur lesen. Das Balancing schreibt Quelldateien nur unter `data/worlds/` (Wächter `balance.Guard`, WZ63.7); Code- oder andere Datenänderungen erst nach Plan im Chat und Freigabe von 🧑.

## Welten

- Welt-Dateien `data/worlds/`, Biome `data/biomes/`, `data/island-sizes.json`, `data/hub.json`, `data/ruins.json`
- Generator `rust/crates/k3c-level/`, Insel-Code `rust/crates/k3c-sim/src/island/`, Welt-Editor `tools/k3c-studio/frontend/src/world/`
- Gebäudeplaner `tools/k3c-studio/internal/hubplan/`, Seite `tools/k3c-studio/frontend/src/planner/`

## Grafik

- Papierschnitt: Specs und Indizes `public/grafik/<gruppe>/` (`index.paper.json`, `golden.json`), Zuordnung `data/paper.json`
- Renderer und Verträge `tools/paper/`, Standards `docs/standards/paper-spec.md`, `docs/standards/paper-mcp.md`
- Rig `rust/crates/k3c-rig/`, Laden im Spiel `rust/crates/k3c-game/src/art/`, Figuren `rust/crates/k3c-game/src/actors/`
- Assets ändern nur MCP `assets_*`; Asset-Logik `tools/assetkit/`
- Im Code gezeichnete HUD-Grafik ohne Asset (vorläufig): `rust/crates/k3c-game/src/hud/purse.rs`, `rust/crates/k3c-game/src/hud/icons.rs`

## Sound

- Slots `data/sounds.json`, Atlas `public/audio/` (`atlas.json`), Werkzeug `tools/soundatlas/`
- Wiedergabe `rust/crates/k3c-game/src/audio/`, Lizenzen `rust/crates/k3c-game/src/menu/licenses.rs`

## HUD, Menüs, Eingabe

- HUD `rust/crates/k3c-game/src/hud/` (Layout `layout.rs`, Radar `radar.rs`, Skill-Menü `skillmenu.rs`)
- Menüs `rust/crates/k3c-game/src/menu/`, Eingabe `…/input/`, Spielablauf `…/game/`, Sicht `…/view/` (alle unter `rust/crates/k3c-game/src/`)
- Durchläufe `rust/crates/k3c-game/tests/e2e_*.rs`, Regeln `docs/rules/bedienung.md`

## Workbench

- App `tools/k3c-workbench/` (Wails, `uiapi.go`), MCP-Server `tools/k3c-workbench/internal/mcpsrv/` (Anleitung `instructions.md`)
- Planung `tools/k3c-workbench/internal/planning/`, Hook `tools/hooks/planGuard.ts`
- Oberfläche `tools/k3c-workbench/frontend/src/`, Bausteine `tools/webkit/`, Seiten-Spec `docs/standards/workbench-seiten.md`
