# SP04 · SIM · Golden-Tests, RNG, Level-Generator in Go

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-043
- **Start-Commit:** 4f4cb2f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (3 Simulationsläufe, Generator als Vitest-Datei, FMA: Regel + Ticket B-071)

## Ausgangslage

RNG und Level-Generator gibt es nur in TypeScript (`createRng` in `src/core/rng.ts`, `src/world/levelGenerator.ts`).

## Ziel

Go erzeugt für dieselben Seeds exakt dieselben Level wie TypeScript. Am Ende sichtbar: Golden-Tests für RNG und Level grün.

## Beteiligte und Zielgruppen

Entwickler oder Agent; alle folgenden SIM-Sprints stützen sich auf die Golden-Daten.

## Anforderungen

B-043 › Anforderungen. Sprint-eigen:

- Golden-Daten aus TS in `testdata/golden/`: RNG-Folgen (auch Seeds mit Umlauten und Emoji), Level für je 20 Seeds
  pro Biom, Simulationsläufe (feste Seeds und Eingabe-Folgen, Snapshot alle 30 Ticks, mindestens ein Lauf ohne Gegner
  für SP05). Erzeugt von `tests/golden.test.ts` über `toMatchFileSnapshot` (Vitest): `npm test` prüft, dass TS die
  Daten unverändert erzeugt, `npm run golden` schreibt sie neu.
- `engine/rng` (mulberry32, FNV-1a über UTF-16) und `engine/level` (`Generate`, `Validate`) in Go, Biome aus `data/`.

## Nicht-Ziele

Simulation in Go (SP05, SP06); Go-Tests der Simulationsläufe (SP05, SP06). `src/world/` wird nur gelesen.

## Regeln und Einschränkungen

- `src/world/` und `src/core/rng.ts` werden nur gelesen (Grenzfall Portierung). Portierungs-Fallen aus Entscheidung 001.
- Weitere Fallen: stabile Sortierung (`sort.SliceStable` wie `Array.prototype.sort`); Reihenfolge der JSON-Schlüssel
  aus `data/biomes/*.json` (`Object.entries`) bleibt erhalten; `Math.round` → `math.Round` nur für positive Werte;
  Go darf Fließkomma-Operationen fusionieren (FMA, z. B. auf arm64): Produkte vor einer Addition mit `float64(…)` runden.
- Keine neuen Abhängigkeiten. Ausnahmen außerhalb der Domäne (INF): `tests/golden.test.ts` (neu), `package.json`
  (Skript `golden`), `.gitattributes` (nur falls Zeilenenden die Golden-Daten brechen).
- Schichtgrenzen: `engine/rng` und `engine/level` importieren nichts aus `engine/room`, `engine/net`, `cmd/`.

## Beispiele

Seed „Käse🧀“ → Go und TS liefern dieselbe RNG-Folge und dasselbe Level.

## Ausnahme- und Fehlerfälle

Golden-Abweichung → der Test nennt Seed, Tick bzw. Position in der Folge und Feld.

## Akzeptanzkriterien

- **AC-01** Ein TS-Skript erzeugt die Golden-Daten in `testdata/golden/`.
- **AC-02** Die RNG-Golden-Tests sind grün, auch für Seeds mit Umlauten und Emoji (B-043/AC-01).
- **AC-03** Die Level-Golden-Tests sind für alle Biome grün.

## Offene Fragen

keine. Geklärt von 🧑 (2026-09-30, Chat): Golden-Daten entstehen über `tests/golden.test.ts` (Vitest,
`toMatchFileSnapshot`), TS zeichnet nur auf, die neue Logik entsteht in Go; drei Simulationsläufe wie in SP04.1;
FMA: Regel im Go-Code, arm64-Lauf als Ticket B-071.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP04.1 | `SP04.1-golden-daten.md` | Umsetzung | autonom | fertig |
| SP04.2 | `SP04.2-rng.md` | Umsetzung | autonom | fertig |
| SP04.3 | `SP04.3-level.md` | Umsetzung | autonom | fertig |
| SP04.4 | `SP04.4-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-09-30, leichtes Review über `4f4cb2f..main` (Agent, im selben Lauf wie die Umsetzung, auf Auftrag von 🧑).
  Kriterien: AC-01 (SP04.1), AC-02 (SP04.2), AC-03 (SP04.3), alle mit Gegentest belegt. B-043/AC-02 folgt in SP05/SP06.
- Befunde: keine schweren. `src/` und `data/` sind unverändert, `engine/rng` und `engine/level` importieren nur
  `k3c/data` und `k3c/engine/rng`. Neue Tickets: B-071 (Golden-Tests auf arm64), B-072 (depguard für `engine/rng`).
