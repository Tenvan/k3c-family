# native/k3c-game/actors/

## Responsibility

Fachpaket Figuren (NG3, B-796): zeichnet Monarchen auf ihrem Reittier, Truppen je Beruf, Gegner und Bosse samt Anzeigen (Lebensbalken, Beute, Last) aus dem Spielzustand.

## Design

- `actors.go`: `New(lib)` liefert einen Drawer auf `LayerActors`, der zugleich `Listener` und `AssetSource` ist; Paketkommentar.
- `catalog.go`: `Catalog` (`LoadCatalog` aus `k3c/data`): Spielrolle → Papier-Asset, Höhe, Bodenabstand, Rig oder Sheet (`Player`, `Enemy`, `Look`, `SheetAssets`).
- `anim.go`: Zustand → Animation als reine Funktionen (`MonarchWant`, `FigureWant`, `MountGait`, `FallbackGait`, `MonarchFrame`, `SheetFrame`, `AnimTime`).
- `figures.go`: Figuren mit Zustand je stabiler Sim-ID (Rig-Actor oder Sheet, Pose, Richtung); `Listen` gleicht mit der Welt ab (`TroopWant`, `MonarchAlpha`).
- `pose.go`: schreibt Animation und Umhang der Rig-Figuren im festen Takt fort.
- `draw.go`: zeichnet je Zelle hinten nach vorn Truppen, Gegner, Monarchen.
- `marks.go`/`markdraw.go`: Anzeigen an Figuren (NG12, B-806) als reine Regeln und eingefärbte Vorlagen.
- `levels.go`: `LevelSheets` nennt nur die `requisite`-Figuren, die im Level vorkommen können.

## Flow

1. `New(lib)` lädt den Katalog und prüft die Rigs (`checkRigs`).
2. Levelwechsel: `Assets(level, world)` → `LevelSheets` → `art.Library.SetLevels`.
3. Je Frame `Listen(frame)` → Figuren abgleichen, `anim.go` wählt Pose, `pose.go` taktet Rig und Umhang (`art.Actor`).
4. Draw je Zelle → `draw.go` zeichnet Rig-Figuren über `art.Painter`/`art.RigView`, Sheets über `art.Sheet`, dann `markdraw.go`.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art`, `k3c/engine/sim`, `k3c/engine/level`, `k3c/data`.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`).
