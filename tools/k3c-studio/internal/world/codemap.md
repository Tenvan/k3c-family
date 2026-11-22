# tools/k3c-studio/internal/world/

## Responsibility

Backend des Welten-Editors des Studios (B-879): Welten unter `data/worlds/` auflisten, lesen, anlegen, ändern und löschen, das Level einer Insel-Stufe für die Chunk-Vorschau holen, Werte-Overrides, Natur-Werte, Etappen und die Geschichte (`story`) setzen; Level, Basiswerte und Prüfung rechnet `simcli world` im Prozess, geschrieben wird nur die Welt-Datei.

## Design

- `Editor{Root, Engine}` (`world.go`): `List()` → `[]Info` (zuerst die Standard-Welt, dann die festen Welten aus `data/paper.json › weltbilder`, auch ohne Datei als `Missing` mit `MissingNote`, danach die übrigen Dateien nach ID), `Load(id)` → `Detail` mit `Island` (Tiefen, Biome, geplante Chunks, Chunk-Anzahl, `Etappe`, `Story`), `Story` der Welt und `BiomeInfo` (Palette, Portal-HP); IDs nur nach `idPattern`.
- Engine-Seite (`level.go`): `Engine func(ctx, input) (json.RawMessage, error)` mit JSON-Anfragen `{"op": "level" | "values" | "check", …}`; `Direct(root)` ruft `world` über `engine/simcli` im Prozess auf (B-924, `simcli.DirFiles` auf `data/` des Checkouts, Timeout `engineTimeout`).
- `Level(id, island, stage, seed)` → `LevelView` (Layout wie im Spiel, geplante Chunks der Stufe, Warnungen).
- Werte (`values.go`, B-680/B-682): `Values(id)` → `ValuesView` mit `ValueEntry` je Feld (Basiswert aus `data/` plus Eltern-Overrides, Override der Welt).
- Welt anlegen und bearbeiten (B-954): `Create`/`CreateWithStory`, `Set` (Kopf, Inseln, Tiefen/Biome), `SetChunks`, `Check`, `Raw`, `SetEtappeView`, `CreateView` (`edit.go`, `edit_island.go`; `inline.go` schreibt Werte im Zeilenstil der Dateien); gleiches Muster: Prüfung `op: check`, dann `writeChecked` (atomar, nur wenn die Datei noch den gelesenen Stand hat, neue Dateien exklusiv; sonst `errChanged`). MCP: `world_*` in `mcpsrv/tools_world.go`.
- Löschen (`delete.go`, B-1172): `Delete`/`DeleteView` lehnen feste Welten, fehlende Dateien und Welten mit Erben (`extends`) ab; Spielstände unter `saves/`, die die Welt nutzen (`savesUsing`), bleiben und erscheinen als Warnung.
- Geschichte (`story.go`, B-975): `SetStory(id, StoryPatch)` ändert `story` der Welt und je Insel format-stabil; die Regeln prüft die Engine vor dem Schreiben (`StoryResult` nennt gesetzte, entfernte und fehlende Einträge).
- Schreiben: `SetOverride(id, base, path, value)` (`override.go`; `null` oder Basiswert entfernt den Override; Ablehnung unbekannter oder gesperrter Felder erklärt `rejectOverride` in `override_reject.go` mit den erlaubten Dateien und Feldern) und `SetNature(id, island, stage, key, value, seed)` (`nature.go`, B-891; nur `natureByDepth` der Insel; eine fehlende Insel meldet `noIslandError`). Vor dem Schreiben prüft k3c-sim den Entwurf (`op: check`); bei Fehler bleibt die Datei unverändert und `Error` nennt den Grund.
- `patch.go`: format-stabiles Schreiben, nur der geänderte Wert bekommt neue Bytes, Zeilenenden bleiben; atomar über `.tmp-`-Datei und `store.Rename`.

## Flow

1. Oberfläche → `App.WorldList`/`WorldDetail` (`app_world.go`) → `Editor.List`/`Load`.
2. Chunk-Vorschau → `App.WorldLevel` → `Level` → `Engine` (`op: level`) → `LevelView`.
3. Werte-Ansicht → `App.WorldValues` → `Values` (`op: values`); Ändern → `App.WorldOverrideSet` → `SetOverride` → Patch → `op: check` → atomar schreiben → neue `ValuesView`.
4. Natur → `App.WorldNatureSet` → `SetNature` → Patch → `op: check` → schreiben → neues Level.

## Integration

- Konsumenten: `tools/k3c-studio/app_world.go` (Bindings `WorldList` … `WorldCreate`, `WorldDelete`, `WorldIslandEtappe`; `worldEngine = world.Direct`), `internal/mcpsrv/tools_world.go` und `tools_world_story.go` (`world_*`).
- Abhängigkeiten: `engine/simcli`, `engine/store` (`Rename`); Daten `data/worlds/*.json`, `data/paper.json`, `data/biomes/`; Regeln `docs/rules/welten.md`.
