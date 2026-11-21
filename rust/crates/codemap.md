# rust/crates/

## Responsibility

Die Crates des Cargo-Workspaces `rust/` (`members = ["crates/*"]`): Spiel `k3c-game`, Simulation `k3c-sim` mit Daten, Level, Raum und Zufall (Port der Go-Engine), Plattformkern, Rig und das Cheat-Fenster samt Protokoll. Dieser Ordner enthält nur Crates, keinen eigenen Code.

## Design

| Crate | Aufgabe |
|---|---|
| `k3c-rng` | Zufallsgenerator mulberry32 (Port `engine/rng`), ohne Abhängigkeit |
| `k3c-det` | Gleitkomma wie Go auf amd64 (`exp`, `log`, `pow`, `fmax`, `fmin`) und Golden-Vergleich |
| `k3c-data` | typisierte Strukturen und Lader für `data/*.json`, `Source` (Ordner oder eingebettet) |
| `k3c-level` | Level-Generator (Port `engine/level`): Biome, Chunks, Natur, Ruinen, Plätze, Prüfung |
| `k3c-sim` | Simulation (Port `engine/sim`); hier nicht beschrieben, siehe eigenen Ordner |
| `k3c-room` | Raum mit Stufen und Spielstand (Port `engine/room`), Dateispeicher (Port `engine/store`) |
| `k3c-balance` | Gerüst für den Port von `engine/balance` (nur ein `crate_baut`-Test) |
| `k3c-platform` | Fenster (winit), Zeichnen (wgpu), Eingabe (gilrs), Ton (rodio), Takt, headless |
| `k3c-rig` | Papierschnitt-Figuren aus Teilen rechnen (`rig`) und zeichnen (`paint`) |
| `k3c-devproto` | Zeilen-JSON-Protokoll zwischen Spiel und Cheat-Fenster |
| `k3c-devwin` | Cheat-Fenster `k3c-devwin.exe` (Win32, nur Windows) |
| `k3c-alloc-probe` | zählender `GlobalAlloc` für Allokationstests |
| `k3c-game` | das Spiel (EXE); hängt von fast allen anderen ab, hier nicht beschrieben |

Abhängigkeitsrichtung (path-Abhängigkeiten aus den `Cargo.toml`, von unten nach oben):

- Blätter ohne interne Abhängigkeit: `k3c-rng`, `k3c-det`, `k3c-platform`, `k3c-devproto`, `k3c-alloc-probe`.
- `k3c-data` → `k3c-det`; `k3c-level` → `k3c-rng`, `k3c-data`, `k3c-det`; `k3c-sim` → `k3c-level`, `k3c-rng`, `k3c-data`, `k3c-det`.
- `k3c-room` → `k3c-sim`, `k3c-level`, `k3c-data`, `k3c-rng`; `k3c-balance` → `k3c-sim`, `k3c-data`.
- `k3c-rig` → `k3c-platform`; `k3c-devwin` → `k3c-devproto`.
- `k3c-game` → `k3c-rng`, `k3c-det`, `k3c-data`, `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-balance`, `k3c-rig`, `k3c-platform`, `k3c-devproto`, `k3c-devwin`, `k3c-alloc-probe`.

## Flow

1. `k3c-data` liest die Balancing-JSON über `Source`; `k3c-level` erzeugt daraus mit `k3c-rng` ein Level; `k3c-sim` rechnet es; `k3c-room` hält Räume, Stufen und Spielstände darüber.
2. `k3c-game` treibt Raum und Sim im festen Takt aus `k3c-platform` und zeichnet über `Canvas` (Figuren über `k3c-rig`).
3. Über `k3c-devproto` tauschen Spiel und `k3c-devwin` Dev-Befehle und Zustand.

## Integration

- Workspace-Wurzel: [`rust/codemap.md`](../codemap.md) (Workspace, Taskfile, Paket).
- Crate-Karten: [`k3c-alloc-probe`](k3c-alloc-probe/codemap.md), [`k3c-balance`](k3c-balance/codemap.md), [`k3c-data`](k3c-data/codemap.md), [`k3c-det`](k3c-det/codemap.md), [`k3c-devproto`](k3c-devproto/codemap.md), [`k3c-devwin`](k3c-devwin/codemap.md), [`k3c-level`](k3c-level/codemap.md), [`k3c-platform`](k3c-platform/codemap.md), [`k3c-rig`](k3c-rig/codemap.md), [`k3c-rng`](k3c-rng/codemap.md), [`k3c-room`](k3c-room/codemap.md).
