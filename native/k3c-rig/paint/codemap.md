# native/k3c-rig/paint/

## Responsibility

Adapter zwischen Rig-Rechnung und Ebitengine: lädt die Teil-Bilder der Rig-Figuren und zeichnet eine Figur (zu Fuß, Reiter mit Reittier oder Vierbeiner allein) mit Matrizen und Dreiecksbändern.

## Design

- `assets.go`: `Assets` hält Teil-Bilder, Elemente und Skins (`LoadAssets`, `Image`, `ElementImage`, `Rig`, `Skin`); `ReadPNG`/`WritePNG` für Dateien.
- `actor.go`: `Actor` ist eine laufende Figur aus einem oder zwei Rigs: Zustand (`State`, `Play`, `SetFrame`), Phase und Frequenz (`Phase`, `Freq`), Umhang (`ResetCloak`), `Update`/`Refresh` rechnen die Pose aus der Zeit, `Draw` gibt an den Painter weiter.
- `painter.go`: `Painter` (`NewPainter`) zeichnet Teile als Bild mit Matrix, Bänder (Umhang, Beine) als Dreiecksband über ein weißes Hilfsbild und Zügel als Linie; Puffer werden wiederverwendet.

## Flow

1. `LoadAssets(dir)` → Rigs über `rig.Load`, PNGs über `ReadPNG` als `*ebiten.Image`.
2. `NewActor(assets, rider, mount, variant)` verbindet Reiter- und Reittier-Rig; je Tick `Actor.Update(dt)` → `rig.GaitPose`/`rig.FigurePose`, `rig.Cloak.StepTime`.
3. `Actor.Draw(painter)` → Reittier in `Rig.Order`-Reihenfolge, am Platz `sattel` der Reiter (Matrizen aus `Rig.World`) → `Painter` zeichnet Teile, `rig.Strip`-Bänder und `rig.ReinPoints`-Zügel.

## Integration

- Abhängigkeiten: `native/k3c-rig/rig`, `github.com/hajimehoshi/ebiten/v2`.
- Konsumenten: `native/k3c-rig` (Prototyp-Hauptprogramm), `native/k3c-game/art` (`library.go`, `rigs.go`) und `native/k3c-game/game/deps.go`.
