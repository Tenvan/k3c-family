# B-009 · Komplexitäts-Budget wird automatisch geprüft

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP01
- **Erstellt:** 2026-09-29

## Beschreibung

Oxlint (TypeScript) und `golangci-lint` (Go) prüfen Datei- und Funktionslänge, Verschachtelung und Komplexität; Regel-Tests prüfen Schichtgrenzen und die Ratsche für Bestandscode.

## Warum

Niedrige Komplexität ist Pflicht in jeder Session und soll nicht vom Gedächtnis des Reviews abhängen.

## Akzeptanz

`npm run check` und die Go-Prüfungen scheitern bei Verstößen, lokal und in der CI.

## Notizen

TS 7 hat keine JS-API, deshalb Oxlint statt `typescript-eslint`.
