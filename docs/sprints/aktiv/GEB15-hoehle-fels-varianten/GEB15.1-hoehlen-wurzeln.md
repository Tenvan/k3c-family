# GEB15.1 · Höhlen-Wurzeln: mehr Varianten und dichter

- **Status:** in Arbeit
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
