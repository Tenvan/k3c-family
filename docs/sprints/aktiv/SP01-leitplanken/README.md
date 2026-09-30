# SP01 · INF · Leitplanken + Go-Gerüst

- **Status:** aktiv
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-009, B-033, B-050
- **Start-Commit:** 918f385
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (Revision 2, mit B-050)

## Ausgangslage

Das Komplexitäts-Budget steht nur in `docs/arbeitsweise.md`, kein Werkzeug prüft es. `tsconfig.json` schließt `tests/` aus. Es gibt kein Go-Modul; die Balancing-Daten liegen in `src/data/`.

## Ziel

Das Komplexitäts-Budget wird für TypeScript und Go automatisch geprüft, und das Repo hat ein Go-Modul, in das die
Engine ab SP03 einzieht. Am Ende sichtbar: `npm run check` lokal und in der CI grün; die CI hat einen Go-Job mit
Tests, `golangci-lint` und Cross-Build für Windows und Raspberry Pi.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten (jede künftige Session); die Review-Session.

## Anforderungen

B-009 › Anforderungen, B-033 › Anforderungen, B-050 › Anforderungen (seit Revision 2). Sprint-eigen: Go-Modul `k3c` im Root; die Daten liegen in `data/` als einzige Quelle für Client (Import) und Server (`go:embed`); die CI hat einen Go-Job mit Tests, Lint und Cross-Build für `windows/amd64` und `linux/arm64`.

## Nicht-Ziele

- Bestandscode verkleinern (B-018, B-034): hier nur Ausnahmen mit heutigem Wert (Ratsche).
- Go-Engine-Code (ab SP03/SP04). Hier nur Modul, Daten-Einbettung, Lint und CI.

## Regeln und Einschränkungen

Grenzen aus `docs/arbeitsweise.md` › Komplexitäts-Budget: harte Grenzen im Linter, das Ziel mit Ratsche im Regel-Test. Keine Wertänderung in `data/*.json`. TS 7 hat keine JS-API, deshalb Oxlint. Die neuen Abhängigkeiten (Oxlint, `@types/node`) decken B-009 und B-033, Zustimmung im Review.

## Beispiele

Funktion mit 61 Zeilen → `npm run lint` scheitert. Import aus `src/scenes` in `src/world` → `npm test` scheitert. Ungültige JSON-Datei in `data/` → `go test ./...` scheitert.

## Ausnahme- und Fehlerfälle

Bestandsdatei über einer Grenze → Ausnahme mit gemessenem Wert (Ratsche), kein Umbau. Go lokal nicht installiert → maßgeblich ist die CI.

## Akzeptanzkriterien

- **AC-01** `npm run lint` (Oxlint) scheitert bei Verstößen gegen die harten Budget-Grenzen; der Bestand steht mit gemessenem Wert als Ratsche in `.oxlintrc.json` (B-009/AC-01).
- **AC-02** `npm run check` bündelt Lint, Typecheck und Tests und läuft in der CI grün.
- **AC-03** Die Balancing-Daten liegen in `data/`; der Client importiert von dort, `data/embed.go` bettet sie ein, `data/embed_test.go` prüft sie.
- **AC-04** CI-Job `go`: `go test ./...`, `golangci-lint` mit Budget- und Schichtregeln und Cross-Build für `windows/amd64` und `linux/arm64` sind grün (B-009/AC-02).
- **AC-05** `npm test` scheitert, wenn eine Datei ohne passende Ausnahme über 300 Zeilen liegt, eine Ausnahme überflüssig ist oder `src/world` aus `scenes/`, `online/` oder `input/` importiert.
- **AC-06** `npm run typecheck` prüft `tests/` (B-033/AC-01).
- **AC-07** Der Regel-Test prüft auch Go-Dateien: über 300 Zeilen nur mit Ausnahme, über 400 nie, überflüssige Ausnahmen scheitern (B-050/AC-01, B-050/AC-02, B-050/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP01.1 | `SP01.1-client-lint.md` | Umsetzung | autonom | in Arbeit |
| SP01.2 | `SP01.2-go-geruest.md` | Umsetzung | autonom | offen |
| SP01.3 | `SP01.3-regel-tests.md` | Umsetzung | autonom | offen |
| SP01.4 | `SP01.4-review.md` | Review | autonom | offen |

## Abnahme

–
