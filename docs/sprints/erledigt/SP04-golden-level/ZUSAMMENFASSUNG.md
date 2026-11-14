# SP04 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Golden-Tests, RNG, Level-Generator in Go
- **Projekt:** –

## Ziel

Go erzeugt für dieselben Seeds exakt dieselben Level wie TypeScript. Am Ende sichtbar: Golden-Tests für RNG und Level grün.

## Ergebnis

- 2026-09-30, leichtes Review über `4f4cb2f..main` (Agent, im selben Lauf wie die Umsetzung, auf Auftrag von 🧑). Kriterien: AC-01 (SP04.1), AC-02 (SP04.2), AC-03 (SP04.3), alle mit Gegentest belegt. B-043/AC-02 folgt in SP05/SP06. - Befunde: keine schweren. `src/` und `data/` sind unverändert, `engine/rng` und `engine/level` importieren nur `k3c/data` und `k3c/engine/rng`. Neue Tickets: B-071 (Golden-Tests auf arm64), B-072 (depguard für `engine/rng`).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Ein TS-Skript erzeugt die Golden-Daten in `testdata/golden/`. |
| AC-02 | geprüft | Die RNG-Golden-Tests sind grün, auch für Seeds mit Umlauten und Emoji (B-043/AC-01). |
| AC-03 | geprüft | Die Level-Golden-Tests sind für alle Biome grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-043 | erledigt | AC-02 geprüft | Portierungs-Fallen sind durch Golden-Tests abgedeckt |
