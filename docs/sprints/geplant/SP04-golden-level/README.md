# SP04 · SIM · Golden-Tests, RNG, Level-Generator in Go

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-043
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RNG und Level-Generator gibt es nur in TypeScript (`createRng`, `src/world/levelGenerator.ts`).

## Ziel

Go erzeugt für dieselben Seeds exakt dieselben Level wie TypeScript. Am Ende sichtbar: Golden-Tests für RNG und Level grün.

## Beteiligte und Zielgruppen

Entwickler oder Agent; alle folgenden SIM-Sprints stützen sich auf die Golden-Daten.

## Anforderungen

B-043 › Anforderungen. Sprint-eigen: Golden-Daten aus TS (RNG-Folgen, Level für je 20 Seeds pro Biom, Simulationsläufe); `engine/rng` und `engine/level` inkl. `validateLevel`.

## Nicht-Ziele

Simulation (SP05, SP06). `src/world/` wird nur gelesen.

## Regeln und Einschränkungen

`src/world/` wird nur gelesen (Grenzfall Portierung). Portierungs-Fallen aus Entscheidung 001.

## Beispiele

Seed „Käse🧀“ → Go und TS liefern dieselbe RNG-Folge und dasselbe Level.

## Ausnahme- und Fehlerfälle

Golden-Abweichung → der Test nennt Seed, Tick und Feld.

## Akzeptanzkriterien

- **AC-01** Ein TS-Skript erzeugt die Golden-Daten in `testdata/golden/`.
- **AC-02** Die RNG-Golden-Tests sind grün, auch für Seeds mit Umlauten und Emoji (B-043/AC-01).
- **AC-03** Die Level-Golden-Tests sind für alle Biome grün.

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP04.1 TS-Skript erzeugt Golden-Daten nach `testdata/golden/`: RNG-Folgen (auch Seeds mit Umlauten/Emoji), Level für je 20 Seeds pro Biom, Simulationsläufe (feste Seeds + Eingabe-Folgen, Snapshot alle N Ticks) (AC-01).
- SP04.2 `engine/rng` (mulberry32, FNV-1a über UTF-16) + `engine/level` inkl. `validateLevel`; Golden-Tests grün (AC-02, AC-03).
- SP04.3 🔍 Review (alle).

## Abnahme

–
