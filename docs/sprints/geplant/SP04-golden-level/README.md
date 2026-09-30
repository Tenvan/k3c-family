# SP04 · SIM · Golden-Tests, RNG, Level-Generator in Go

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-043
- **Start-Commit:** –

## Ziel

Go erzeugt für dieselben Seeds exakt dieselben Level wie TypeScript. Am Ende sichtbar: Golden-Tests für RNG und Level grün.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben.

- SP04.1 TS-Skript erzeugt Golden-Daten nach `testdata/golden/`: RNG-Folgen (auch Seeds mit Umlauten/Emoji), Level für je 20 Seeds pro Biom, Simulationsläufe (feste Seeds + Eingabe-Folgen, Snapshot alle N Ticks).
- SP04.2 `engine/rng` (mulberry32, FNV-1a über UTF-16) + `engine/level` inkl. `validateLevel`; Golden-Tests grün.
- SP04.3 🔍 Review.

## Nicht im Sprint

Simulation (SP05, SP06). `src/world/` wird nur gelesen.

## Abnahme

–
