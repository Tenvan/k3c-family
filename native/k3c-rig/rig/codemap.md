# native/k3c-rig/rig/

## Responsibility

Domain-Layer ohne Grafik-Bibliothek: rechnet Papierschnitt-Figuren zur Laufzeit aus Teilen (Teile-Baum, Posen, Gangarten, Umhang-Physik, Bänder, Zügel), als Go-Gegenstück zu `tools/paper/rig/*.ts`.

## Design

- `matrix.go`: 2D-Affinmatrix `M` mit `Mul`, `Apply`, `Invert`, `Translate`, `Rotation`, `Scale`; Paketkommentar.
- `load.go`: Datenmodell der Rig-Dateien (`Rig`, `Part`, `Slot`, `Image`, `Band`, `Element`, `Keyframe`, `Animation`, `LegSpec`, `CloakSpec`, `Riding`, `Reins`, `Proportions`, `Index`) und Laden (`Load`, `LoadElements`, `ImageFile`).
- `keyframes.go`: Keyframe-Teile und ausgeblendete Teile (`KeyframeHidden`); `curve.go`: Catmull-Rom wie `tools/paper/rig/kurve.ts`.
- `figure.go`: Pose einer Figur zu Fuß (`FigurePose`, `FigureFreq`); `gait.go`: Gangarten des Vierbeiners mit Standphase, Schwungbahn und Zwei-Knochen-IK (`GaitPose`, `GaitFreq`, `RideMatrix`, `RiderAngles`) wie `gangarten.ts`.
- `frames.go`: Zustände wie in den Sheets (`Set`, `Ride`, `FrameTime`, `RideFloor`) samt eingeschwungenem Umhang (`FigureCloakFrames`, `RideCloakFrames`, `CloakAnchor`).
- `cloak.go`: Umhang-Physik mit festem Zeitschritt `CloakDT` (`Cloak`, `NewCloak`, `Step`, `StepTime`, `CloakRun`, `FrameSteps`).
- `band.go`: Dreiecksbänder für Umhang und Beine (`Strip`, `Vertex`, `LegPoints`); `ride.go`: Sattelplatz und Zügel (`RiderBase`, `ReinPoints`, `Hidden`).
- `order.go`: Zeichenreihenfolge als Liste `Item` je `ItemKind`; `Rig.World` rechnet die Welt-Matrix je Teil (Kinder erben die Drehung), `Rig.SlotWorld` die eines Platzes.

## Flow

1. `Load(dir)` liest Rig-JSON eines Ordners `gruppe/id` → `*Rig`.
2. Je Bild: `FigurePose`/`GaitPose` aus Zustand und Phase → Winkel je Teil; `RideMatrix`/`RiderBase` setzen den Reiter auf den Sattel.
3. `Rig.World` bildet aus Pose und Teile-Baum die Weltmatrizen und die Zeichenreihenfolge; `Cloak.StepTime` schiebt den Umhang, `Strip`/`LegPoints` liefern die Band-Ecken.
4. Der Zeichner (`native/k3c-rig/paint`) setzt die Ergebnisse in Ebitengine-Aufrufe um.

## Integration

- Abhängigkeiten: nur Stdlib.
- Konsumenten: `native/k3c-rig/paint`, `native/k3c-rig` (Prototyp), `native/k3c-game/art/rigs.go`, `native/k3c-game/game/deps.go`.
- Daten: Rig-Dateien und Sollwerte unter `native/k3c-rig/assets/`, erzeugt von `native/k3c-rig/export/export.ts` (`task native:rig:export`); Tests vergleichen gegen `assets/soll`.
