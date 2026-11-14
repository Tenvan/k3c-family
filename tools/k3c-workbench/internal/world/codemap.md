# tools/k3c-workbench/internal/world/

## Responsibility

Backend des Welten-Editors der Workbench (B-879): Welten unter `data/worlds/` auflisten und lesen, das Level einer Insel-Stufe für die Chunk-Vorschau holen, Werte-Overrides und Natur-Werte einer Welt setzen; Level, Basiswerte und Prüfung rechnet `bin/k3c-sim world`, geschrieben wird nur die Welt-Datei.

## Design

- `Editor{Root, Engine}` (`world.go`): `List()` → `[]Info`, `Load(id)` → `Detail` mit `Island` (Tiefen, Biome, geplante Chunks, Chunk-Anzahl); IDs nur nach `idPattern`.
- Engine-Seite (`level.go`): `Engine func(ctx, input) (json.RawMessage, error)` mit JSON-Anfragen `{"op": "level" | "values" | "check", …}`; `SimExe(root)` ruft `bin/k3c-sim world` über `internal/simexe` (B-726: kein Import des Engine-Kerns); Tests setzen einen Ersatz.
- `Level(id, island, stage, seed)` → `LevelView` (Layout wie im Spiel, geplante Chunks der Stufe, Warnungen).
- Werte (`values.go`, B-680/B-682): `Values(id)` → `ValuesView` mit `ValueEntry` je Feld (Basiswert aus `data/` plus Eltern-Overrides, Override der Welt).
- Schreiben: `SetOverride(id, base, path, value)` (`override.go`; `null` oder Basiswert entfernt den Override) und `SetNature(id, island, stage, key, value, seed)` (`nature.go`, B-891; nur `natureByDepth` der Insel). Vor dem Schreiben prüft k3c-sim den Entwurf (`op: check`); bei Fehler bleibt die Datei unverändert und `Error` nennt den Grund.
- `patch.go`: format-stabiles Schreiben, nur der geänderte Wert bekommt neue Bytes, Zeilenenden bleiben; atomar über `.tmp-`-Datei und `store.Rename`.

## Flow

1. Oberfläche → `App.WorldList`/`WorldDetail` (`app_world.go`) → `Editor.List`/`Load`.
2. Chunk-Vorschau → `App.WorldLevel` → `Level` → `Engine` (`op: level`) → `LevelView`.
3. Werte-Ansicht → `App.WorldValues` → `Values` (`op: values`); Ändern → `App.WorldOverrideSet` → `SetOverride` → Patch → `op: check` → atomar schreiben → neue `ValuesView`.
4. Natur → `App.WorldNatureSet` → `SetNature` → Patch → `op: check` → schreiben → neues Level.

## Integration

- Konsument: `tools/k3c-workbench/app_world.go` (Bindings, `worldEngine = world.SimExe`).
- Abhängigkeiten: `internal/simexe`, `engine/store` (`Rename`); Daten `data/worlds/*.json`; Regeln `docs/rules/welten.md`.
