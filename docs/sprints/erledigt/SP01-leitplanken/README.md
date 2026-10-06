# SP01 · INF · Leitplanken + Go-Gerüst

- **Status:** erledigt
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-009, B-033, B-050, B-052
- **Start-Commit:** 918f385
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-09-30 🧑 Chat-Anweisung von Ralf (Revision 3, mit B-052; Revision 2 mit B-050 ebenfalls freigegeben)

## Ausgangslage

Das Komplexitäts-Budget steht nur in `docs/arbeitsweise.md`, kein Werkzeug prüft es. `tsconfig.json` schließt `tests/` aus. Es gibt kein Go-Modul; die Balancing-Daten liegen in `src/data/`.

## Ziel

Das Komplexitäts-Budget wird für TypeScript und Go automatisch geprüft, und das Repo hat ein Go-Modul, in das die
Engine ab SP03 einzieht. Am Ende sichtbar: `npm run check` lokal und in der CI grün; die CI hat einen Go-Job mit
Tests, `golangci-lint` und Cross-Build für Windows und Raspberry Pi.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten (jede künftige Session); die Review-Session.

## Anforderungen

B-009 › Anforderungen, B-033 › Anforderungen, B-050 › Anforderungen (seit Revision 2), B-052 › Anforderungen (seit Revision 3). Sprint-eigen: Go-Modul `k3c` im Root; die Daten liegen in `data/` als einzige Quelle für Client (Import) und Server (`go:embed`); die CI hat einen Go-Job mit Tests, Lint und Cross-Build für `windows/amd64` und `linux/arm64`.

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
- **AC-08** `requirements.md` im Root listet alle vorausgesetzten Installationen mit Version und Prüfbefehl, passend zu `.nvmrc`, `go.mod` und `ci.yml` (B-052/AC-01, B-052/AC-02).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP01.1 | `SP01.1-client-lint.md` | Umsetzung | autonom | fertig |
| SP01.2 | `SP01.2-go-geruest.md` | Umsetzung | autonom | fertig |
| SP01.3 | `SP01.3-regel-tests.md` | Umsetzung | autonom | fertig |
| SP01.4 | `SP01.4-review.md` | Review | autonom | fertig |

## Abnahme

**2026-09-30, SP01.4 (autonomer Agent, nur lokal).** Geprüft: 42 Dateien aus `git diff --stat 918f385..main`
(lokales `main` 9cc82fd, 7 Commits vor `origin/main`; kein `git fetch`, 🧑: nur lokal). Jede Datei vollständig gelesen;
die 11 JSON-Dateien in `data/` als reine Umbenennungen (`git diff -M`: 100 %, keine Wertänderung), `package-lock.json`
über den Diff (nur `oxlint` 1.86.0 mit Plattform-Bindings, `@types/node` 26.6.3 → 22.20.4, `undici-types`).
Lokal: Node 26.7, Go 1.27.0, golangci-lint 2.14.0.

- **AC-01 geprüft:** Messung mit den Zielwerten ohne Ausnahmen: alle 9 Ausnahmen in `.oxlintrc.json` sitzen genau auf
  dem gemessenen Höchstwert (Komplexität landing 29, units 25, worldRenderer 23, world 21, gamepadTest 20, enemies 20,
  levelGenerator 19, HudScene 17, economy 16; Funktionslänge units 62); keine Grenze für neuen Code gelockert.
  Gegenprobe in `src/world/` und `tests/`: Funktion mit 63 Zeilen, Komplexität 16 und Tiefe 5 → `oxlint` Exit-Code 1;
  Komplexität 15 und Tiefe 4 grün. Einschränkung: `server` steht in `ignorePatterns` (Abweichung von SP01.1 › Schritt 3/4,
  dort begründet) → B-055.
- **AC-02 geprüft (lokal), CI verschoben, B-053:** `npm run check` Exit-Code 0 (Lint, Typecheck, 192 Tests auf 9cc82fd),
  `npm run build` Exit-Code 0; `ci.yml` führt `npm run lint` vor dem Typecheck aus.
- **AC-03 geprüft:** Daten in `data/`, Importe in den fünf Client-Dateien zeigen dorthin, `grep src/data` findet außer
  den Planungsordnern nur noch `aufstellung.html` (behoben). `data/embed.go` bettet `*.json` und `biomes/*.json` ein;
  Probe `{ kaputt` an `data/troops.json` → `go test ./...` rot („troops.json ist kein gültiges JSON“), zurückgenommen.
- **AC-04 geprüft (lokal), CI verschoben, B-053:** `go test ./...`, `golangci-lint run` (0 Befunde),
  `golangci-lint config verify`, `GOOS=windows GOARCH=amd64 go build ./...` und `GOOS=linux GOARCH=arm64 go build ./...`
  Exit-Code 0. Gegenprobe mit Wegwerf-Paketen `engine/sim` und `engine/room`: `depguard` (Import `k3c/engine/room`),
  `gocyclo` 16 und `funlen` 61 gemeldet, 15 und 60 grün. `nestif` misst keine Tiefe (4 verschachtelte `if` scheitern,
  Schleifen zählen nicht) → B-054.
- **AC-05 geprüft:** Baseline = `wc -l` der fünf Dateien über 300 Zeilen; Probe spriteReference 330 → 329 → `npm test`
  rot, zurückgenommen. Schichtregel durch Lesen des Tests und SP01.3 › Probe A.
- **AC-06 geprüft:** `npm run typecheck` = `tsc --noEmit && tsc --noEmit -p tests`; Probe `tests/_probe.ts` mit
  Typfehler → TS2322, Exit-Code 1, gelöscht (B-033/AC-01).
- **AC-07 geprüft:** Probe `data/probe.go` mit 301 Zeilen → `npm test` rot, gelöscht; harte Grenze 400 und überflüssige
  Einträge durch Lesen des Tests und SP01.3 › Probe D/E (B-050/AC-01, B-050/AC-02, B-050/AC-03).
- **AC-08 geprüft:** `requirements.md` nennt Git, Node.js 22, Go 1.27, golangci-lint 2.14 mit Zweck, Quelle und
  Prüfbefehl; stimmt mit `.nvmrc` (22), `go.mod` (`go 1.27.0`) und `ci.yml` (`node-version-file`, `go-version-file`,
  `version: v2.14`) überein (B-052/AC-01, B-052/AC-02).

**Behobene Befunde:** `aufstellung.html` nannte im Anzeigetext `src/data/sprites.json` → `data/sprites.json`
(Kleinstes, PLAT). `README.md` › CI/CD beschrieb die CI ohne Lint und Go-Job → ergänzt. B-009, B-033, B-050, B-052
standen noch auf `eingeplant` → `erledigt` (auch Index); B-009 verweist für den CI-Teil auf B-053.

**Neue Abhängigkeiten:** `oxlint` und `@types/node` stehen in der freigegebenen Spec (Regeln und Einschränkungen);
im Review geprüft: beide nur `devDependencies`, sonst kommen nur Oxlint-Plattform-Bindings und `undici-types` dazu.

**Neue Tickets:** B-053 (CI-Lauf für SP01), B-054 (Go-Verschachtelung als Tiefe prüfen), B-055 (`server/*.mjs` ohne
Budget-Prüfung; die Korrektur in `.oxlintrc.json` wurde im Review nicht freigegeben), B-056 (Ratsche zieht gesunkene
Werte nicht nach). Belassen: B-051 (Oxlint-Warnungen im Bestand) unverändert offen.
