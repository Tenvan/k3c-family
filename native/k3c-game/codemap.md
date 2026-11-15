# native/k3c-game/

## Responsibility

Natives Spiel `k3c-game` (Projekt NAT, Entscheidung 008): Go + Ebitengine, Simulation aus `engine/` im selben Prozess; eigenes Go-Modul `k3c/native/k3c-game`, das Wurzel-Modul `k3c` und `native/k3c-rig` per `replace` einbindet.

## Design

- `main.go`: Einstieg; `game.ParseArgs` → `--assets` druckt die ladbare Grafik, sonst `run`; Aufnahme und Messung bekommen einen Wegwerf-Ordner für Spielstände (`shotSaves`) und eine feste Uhr; Fehler vor dem Spiel zeigt `showFault` im Fenster.
- Schichten (Importregel in `game/imports_test.go`):
  - `view/`: Vertrag zwischen Zustand und Darstellung (nur `engine/sim`).
  - `local/`: `view.Session` über `engine/room` und `engine/store`.
  - `game/`: Schleife, kennt alle Pakete.
  - `art/`: Grafik-Bibliothek (Sheets, Rigs, Schrift).
  - Fachpakete: `world/`, `props/`, `actors/`, `fx/`, `hud/`, `input/`, `menu/`, `audio/`, `devtools/`; sie importieren nur `view`, `art`, `engine/sim` (lesend), `engine/level`, `data`.
- `cmd/sync/`: Kopier-Werkzeug für das Paket; `menu/licensefile/`: Lizenzdatei; `e2e/`: Ende-zu-Ende-Test.
- `go.mod`: Ebitengine v2, oto v3, `golang.org/x/image`; `replace k3c => ../..`, `replace k3c/native/k3c-rig => ../k3c-rig`.
- `bin/`: gebaute EXE und `version.txt` (Task `game:exe`, `game:version`).

## Flow

1. `task native:game` → `main` → `build`: `art.New(art.DefaultConfig())` → `game.Assemble(game.Packages(lib))` → `local.Open(cfg)` → `game.New(...)`.
2. `setupWindow` → `ebiten.RunGame(g)`; je Update ein Simulationstakt über `local.Session.Step`, Draw je Bild.
3. Ende: `g.Fault()` bzw. Fehler → stderr und Exit 1; sonst Meldung „beendet“.

## Integration

- Abhängigkeiten: `engine/sim`, `engine/room`, `engine/level`, `engine/store`, `data/` (Balancing, `paper.json`, `sounds.json`), `public/grafik/`, `public/audio/`, `native/k3c-rig`.
- Tasks in `native/Taskfile.yml`: `game`, `game:exe`, `game:build`, `game:shots`, `game:e2e`, `game:measure`, `game:paket`; Paket nach `dist/k3c-game/`.
- Die CI baut und testet das Modul unter Windows.
