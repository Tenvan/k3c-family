# native/k3c-game/art/

## Responsibility

Grafik-Bibliothek des nativen Spiels: lädt Papier-Sheets je Level, Rig-Figuren und die Schrift, verwaltet das Texturbudget und ist der einzige Zugang der Fachpakete zu `native/k3c-rig`.

## Design

- `library.go`: `Config`/`DefaultConfig` (Pfade relativ zur Repo-Wurzel), `Library` (`New`, `SetLevels`, `Sheet`, `Levels`, `Budget`, `RigMiB`); Sheets werden je Level-Satz geladen und freigegeben (`planLevels`, `diffSheets`, `uploadAll`, `freeSheets`).
- `plan.go`: `PlanLevel` liest Manifeste und PNG-Köpfe → `Plan` mit `SheetSpec` je Animation.
- `manifest.go`: Papier-Manifeste `<grafik>/<gruppe>/<asset>/manifest.json` (`ReadManifest`, `Manifest`, `Anim`, `AnimNames`, `GroupAssets`).
- `sheet.go`: `Sheet` als Streifen auf der GPU (`Frame`, `FrameAt`, `Draw` ohne Allokation).
- `budget.go`: `Budget` (Texturbytes, `MiB`, `String`); Paketkommentar.
- `rigs.go`: Rig-Fassade für Fachpakete (`NewActor`, `NewPainter`, `ActorFrame`, `RigView`, Typ-Aliase auf `paint`/`rig`).
- `font.go`: OFL-Schrift aus festem Pfad (`LoadFont`, `Font.Draw`, `Measure`); `outline.go`: Text mit Kontur über ein Zwischenbild (`OutlineCache`, `Sweep`, `OutlineStroke`).
- `testdata/grafik/`: Mini-Grafik für Tests.

## Flow

1. `main.build` → `art.New(art.DefaultConfig())` lädt Rig-Teile und Schrift.
2. Bei Levelwechsel: `game` sammelt Assets über `view.AssetSource` → `Library.SetLevels(want)` → `PlanLevel` → `diffSheets` → `uploadAll`/`freeSheets`; fehlende Dateien liefern `ErrMissing` mit Pfad.
3. Fachpakete zeichnen mit `Library.Sheet(asset, anim).Draw(...)` bzw. `Library.NewActor` + `Painter`.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-rig/rig`, `native/k3c-rig/paint`, Ebitengine.
- Daten: `public/grafik/<gruppe>/` (Zuordnung über `data/paper.json`), `native/k3c-rig/assets/`.
- Konsumenten: `game`, `actors`, `props`, `world`, `hud`, `fx`, `menu`, `devtools`, `input`, `audio` (Signatur `New(*art.Library)`).
