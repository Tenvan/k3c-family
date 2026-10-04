# tools/k3c-dev/internal/enginetools/

## Responsibility
In-process-Fassade auf `engine/level` und `engine/sim` (M6, B-047): erzeugt Level und rechnet kurze Simulationen und gibt knappen Text zurück. Kein Server nötig; gleiche Eingabe ergibt denselben Text.

## Design
- Facade: `Level(biomeID, seed)` (`level.go`) und `Run(biomeID, seed, ticks, inputs)` (`run.go`) kapseln Biom-Laden und Weltaufbau.
- `Biomes` (`forest`, `cave`, `mine`) lädt `loadBiome` aus den embedded Daten; `entityLines` fasst Objekte nach Art zusammen.
- Eingabemodell: `Segment` (Eingabe eines Monarchen von FromTick bis ToTick); `commandsAt`, `playerCount` (max. `maxPlayers`=4) leiten je Tick `sim.PlayerCommand`s ab.
- Schutzgrenzen: `MaxTicks`=100000, `MaxSegments`=100; `TickHz`=30 spiegelt den Server-Takt (`engine/room`).
- Ausgabe: `summary` mit Ereigniszähler (`tally`) und `counts`.

## Flow
1. `Level`: `loadBiome` → Level-Generator mit Seed → Kopfzeile, Zeile je Abschnitt, Objekte nach Art, Prüfwarnungen.
2. `Run`: Grenzen prüfen → `playerCount` → Welt aufbauen → je Tick `commandsAt` → `sim.Step(w, cmds, 1/TickHz)` → Ereignisse zählen → `summary`.

## Integration
- Abhängigkeiten: `k3c/engine/level`, `k3c/engine/sim`, `k3c/data`.
- Konsumenten: `internal/mcpsrv/tools_engine.go` (MCP `level_generate`, `sim_run`), `internal/balance` (`TickHz`, `MaxTicks`).
- Rechnet mit der Engine, mit der k3c-dev gebaut ist, nicht mit dem Worktree-Stand.
