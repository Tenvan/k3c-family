# native/k3c-game/world/

## Responsibility

Fachpaket Welt (B-795): zeichnet je Zelle Himmel, Kulisse (fern, mitte, nah), Boden, Fluss und Licht und führt die Kamera der Zelle.

## Design

- `world.go`: `New(lib)` liefert die Drawer für `LayerFar`, `LayerMid`, `LayerNear`, `LayerGround` und zweimal `LayerRiver` (Fluss, Licht); die ferne Ebene `far` trägt zusätzlich `CameraRule` (`Follow`) und `AssetSource` (`Assets`).
- Reine Rechnung ohne `ebiten.Image`: `camera.go` (Folgen der Stufe je Zelle), `cull.go` (`CullMask`, `Span`, `Visible`), `backdrop.go` (Kacheln `hintergruende/hg-<teil>-<fern|mitte|nah>-v<n>` je Biom aus `data/paper.json` und `data/biomes/`), `ground.go` (Uferstreifen), `river.go` (Kachel-Lage), `light.go` (Tag/Nacht-Regeln).
- Dünne Zeichner: `layers.go` (Kulisse, Boden), `lightdraw.go` (Lichtkarte multipliziert die Zelle), `water.go` (Fluss als Kage-Shader `water/river.kage`, Modus über Umgebung, `RiverStart`).

## Flow

1. `game` ruft im Update `far.Follow(frame, cell, prev)` → neue `view.Camera` je Zelle.
2. Bei Levelwechsel `far.Assets(level, world)` → Kulissen-Assets für `art.Library.SetLevels`.
3. Draw je Zelle: `drawFar` → `drawMid` → `drawNear` → `drawGround` → `drawRiver` → `drawLight`, jeweils mit Culling über die Kamera.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art`, `k3c/engine/sim` (lesend), `k3c/data`, Ebitengine.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`).
