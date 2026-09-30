# SP01 · INF · Leitplanken + Go-Gerüst

- **Status:** geplant
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-009, B-033
- **Start-Commit:** –

## Ziel

Das Komplexitäts-Budget wird für TypeScript und Go automatisch geprüft, und das Repo hat ein Go-Modul, in das die
Engine ab SP03 einzieht. Am Ende sichtbar: `npm run check` lokal und in der CI grün; die CI hat einen Go-Job mit
Tests, `golangci-lint` und Cross-Build für Windows und Raspberry Pi.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP01.1 | `SP01.1-client-lint.md` | Umsetzung | autonom | offen |
| SP01.2 | `SP01.2-go-geruest.md` | Umsetzung | autonom | offen |
| SP01.3 | `SP01.3-regel-tests.md` | Umsetzung | autonom | offen |
| SP01.4 | `SP01.4-review.md` | Review | autonom | offen |

## Nicht im Sprint

- Bestandscode verkleinern (B-018, B-034): hier nur Ausnahmen mit heutigem Wert (Ratsche).
- Go-Engine-Code (ab SP03/SP04). Hier nur Modul, Daten-Einbettung, Lint und CI.

## Abnahme

–
