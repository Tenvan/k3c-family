# GEB15.1 · Höhlen-Wurzeln: mehr Varianten und dichter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb15/1-hoehlen-wurzeln
- **Abhängig von:** –
- **Tickets:** B-582
- **Kriterien:** AC-01

## Ziel

Die Höhle hat mindestens fünf verschiedene Wurzel-Formen und sichtbar mehr Wurzeln. Die Dichte ist ein Wert in `data/`.

## Kontext

- Die Wurzeln stehen heute statt Bäumen außerhalb des Walds: `src/scenes/render/natureSprites.ts` (`WURZELN` mit drei Formen, Wahl nach `mix(x)`), Grafiken `public/grafik/gebaeude/wurzel*.json` aus `tools/buildings` (`task gen:gebaeude`).
- Studio-Vorschau: Szene „Umgebung“ bzw. die Grafik-Seite (`tools/k3c-studio/frontend/src/graphics/`).
- Entscheidung (autonom, Claude, 2026-10-25): Die zusätzliche Dichte ist reine Darstellung. Je Baum-Knoten zeichnet der Client weitere, nicht abbaubare Wurzeln mit festem Versatz nach `mix(x)`. Simulation, Holz-Menge und Golden bleiben unverändert, damit das Balancing nicht kippt.

## Erlaubte Dateien

- `public/grafik/gebaeude/wurzel-*.json` und die PNGs dazu (neu: `wurzel-4`, `wurzel-5`)
- `data/biomes/*.json` (Wert der Dichte), die zugehörigen Typen in `src/model/` und im Go-Struct, falls der Loader unbekannte Felder ablehnt
- `src/scenes/render/natureSprites.ts`, `worldRenderer.ts` bzw. die Stelle, die Knoten zeichnet, samt Tests
- `data/assets.json` und Katalog-Einträge der neuen Grafiken, `docs/assets/zuordnung-welt.md`
- Studio-Vorschau: `tools/k3c-studio/frontend/src/graphics/**`

## Nicht-Ziele

Mehr Holz-Knoten in der Simulation; Balancing; Felsen (GEB15.2).

## Schritte

1. Zwei neue Wurzel-Vorlagen mit anderer Länge und Form erzeugen (`task gen:gebaeude`).
2. `WURZELN` auf fünf Formen erweitern.
3. Dichte-Wert in den Daten. Der Client zeichnet zusätzliche Deko-Wurzeln je Knoten, deterministisch nach Position.
4. Katalog, Slots und Studio-Vorschau nachziehen; Tests.

## Fertig, wenn

- [ ] AC-01: Vitest zeigt fünf Wurzel-Formen und die Zahl der Wurzeln je Knoten aus den Daten. Die neuen Grafiken stehen im Katalog und in der Studio-Vorschau.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01: umgesetzt.
  - Neu `wurzel-4` (dick, buschig, 3 × 15) und `wurzel-5` (lang, dünn, 2 × 15), beide regelkonform von der Decke bis in den Boden. `WURZELN` hat jetzt fünf Formen.
  - Die Dichte steht als `rootsPerNode` in `data/biomes/cave.json` (2, sonst 1). `decoRoots` zeichnet Deko-Wurzeln je Knoten mit festem Versatz nach Position und immer in einer anderen Form als der Knoten.
  - Simulation und Golden sind unverändert.
- Geprüft mit `task check` (grün), Vitest `src/scenes/render` und `tests/`, Studio `src/graphics`, `go test ./tools/buildings/... ./engine/level/... ./engine/sim/...`. Claude hat die PNGs selbst angesehen.
- Abweichung: Die Deko-Wurzeln hängen am Container des Knotens und verschwinden mit ihm. `data/assets.json` führt Wurzeln nicht als Slot, die Studio-Vorschau liest sie aus `index.json`.
