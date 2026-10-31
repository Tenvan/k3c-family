# GEB15.2 · Felsen und Minen: größer, neue Varianten

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb15/2-felsen-minen
- **Abhängig von:** GEB15.1
- **Tickets:** B-583
- **Kriterien:** AC-02

## Ziel

Fels und Mine-Fels sind größer (etwa 1,5-fach) und haben je mindestens zwei neue Varianten, die der Client deterministisch nach Position wählt.

## Kontext

- `public/grafik/gebaeude/felsen.json` erzeugt drei Looks (`felsen-1` Wald, `-2` Höhle, `-3` Mine und andere), gerendert von `tools/buildings`. Der Client wählt in `src/scenes/render/natureSprites.ts` (`nodeSprite`, Art `rock`) nur nach Biom.
- Entscheidung (autonom, Claude, 2026-10-25): Größer über größere Vorlagen, nicht über Skalierung der Pixel. Faktor etwa 1,5, zwei neue Varianten je Look. Die Wahl trifft der Client nach `mix(x)`; Simulation und Golden bleiben gleich.

## Erlaubte Dateien

- `public/grafik/gebaeude/felsen*.json` und PNGs (neue Varianten), ggf. `erzbrocken*`
- `src/scenes/render/natureSprites.ts` samt Tests
- `data/assets.json`, Katalog, `docs/assets/zuordnung-welt.md`
- Studio-Vorschau `tools/k3c-studio/frontend/src/graphics/**`

## Nicht-Ziele

Fels-Knoten in der Simulation; Balancing.

## Schritte

1. Felsen-Vorlage größer, zwei neue Varianten je Look erzeugen.
2. Client wählt die Variante nach Position.
3. Katalog, Slots, Vorschau und Tests.

## Fertig, wenn

- [ ] AC-02: Vitest zeigt drei Varianten je Look nach Position. Die Grafiken sind größer und stehen im Katalog und in der Vorschau.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02: umgesetzt. Der Fels ist jetzt 1,5 × 1,5 Kacheln (neu gezeichnet, nicht skaliert), in drei Formen: `felsen` (Haufen), `felsen-b` (Findling), `felsen-c` (flacher Haufen). Jede Form gibt es in den Looks 1–3, Look 3 (Mine) mit Erzglanz. `rockTexture` wählt die Form nach `mix(x)`.
- Geprüft mit `task check` (grün), Vitest `src/scenes/render` und `tests/`, Studio `src/graphics`, `go test ./tools/buildings/... ./engine/level/... ./engine/sim/...` (Golden unverändert). Claude hat die Vorschau angesehen.
- Die Regeldatei hat die Fels-Zeile mit 1,5 × 1,5 und Begründung B-583. Der Slot `node:rock` zeigt weiter auf `felsen-1-32.png`.
