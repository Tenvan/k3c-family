# B-018 · worldRenderer und GameScene liegen unter 300 Zeilen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP08
- **Erstellt:** 2026-09-29

## Beschreibung

`src/scenes/worldRenderer.ts` (383 Z.) und `src/scenes/GameScene.ts` (371 Z.) liegen über dem Ziel.

## Warum

Große Dateien sind schwer zu prüfen und zu ändern.

## Akzeptanz

Beide ≤ 300 Zeilen, Einträge aus der Ausnahmeliste entfernt.

## Notizen

Beim Umbau zum reinen Client (SP08) aufteilen.
