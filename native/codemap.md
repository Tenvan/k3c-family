# native/

## Responsibility

Natives Spiel und Rig-Prototyp (Projekt NAT, Entscheidung 008: Go + Ebitengine) mit gemeinsamem Taskfile und Messskript für den Mini-PC.

## Design

- `k3c-game/`: natives Spiel, eigenes Modul `k3c/native/k3c-game` (siehe `k3c-game/codemap.md`).
- `k3c-rig/`: Rig-Prototyp und Rig-Bibliothek (`rig`, `paint`), eigenes Modul `k3c/native/k3c-rig`, vom Spiel per `replace` eingebunden.
- `Taskfile.yml`: Namensraum `native` der Wurzel; Variablen für EXE- und Paket-Pfade (`GAME_EXE`, `GAME_PAKET` = `dist/k3c-game`, `RIG_EXE`, `RIG_PAKET` = `.work/native-rig`), Version aus `git describe` (`game:version`).
- `measure.ps1`: misst Spiel bzw. Rig-Prototyp (FPS, GPU-Speicher über WMI, RAM) und schreibt `reports/native-game-*.json` bzw. `reports/native-rig-*.json`; `messen.cmd`: Doppelklick-Starter im Paket.

## Flow

1. Spiel: `task native:game` → `game:exe` (EXE nach `k3c-game/bin/`) → Start; `task native:game:build` → `game:exe` + `gen:paper` → `game:dist` (`k3c-game --assets`, `cmd/sync`, `menu/licensefile`) → `dist/k3c-game/`.
2. Abnahme: `game:shots`, `game:e2e`, `game:measure` (über `measure.ps1 -Engine game`), `game:paket` (Paket leeren, neu füllen, Probe-Aufnahme nach `.work/native-game-probe/`).
3. Rig: `rig:export` (Bun, `k3c-rig/export/export.ts` → `k3c-rig/assets/`), `rig:test`, `rig:build`, `rig`, `rig:shots`, `rig:vergleich`, `rig:measure`, `rig:paket`.

## Integration

- Abhängigkeiten: Wurzel-Modul `k3c` (`engine/`, `data/`), `public/grafik/` (über `task gen:paper`), `public/audio/`, `tools/paper` (Rig-Export).
- Konsumenten: Wurzel-`Taskfile.yml` (`task build`), CI-Workflow `.github/workflows/ci.yml`.
