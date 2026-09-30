# L2 · INF · Go-Tiefe wie TypeScript zählen

- **Status:** erledigt
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-057
- **Start-Commit:** 27cf540
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (Revision 1)

## Ausgangslage

L1 hat die Go-Verschachtelung auf `revive` › `max-control-nesting` 4 umgestellt. Das Review L1.2 hat gezeigt, dass
`revive` `for range` nicht zählt und jedes `else if` als Ebene zählt (B-057). Die Lücke bei AC-01 von L1 ist nach
B-057 verschoben. 🧑 hat entschieden: eigene Tiefen-Prüfung mit `go/ast`, `revive` wird entfernt.

## Ziel

Die harte Grenze Verschachtelung 4 wird in Go genauso gezählt wie in TypeScript, bevor in SP03 der erste echte
Go-Code entsteht. Am Ende sichtbar: `npm run check:go` scheitert bei `range` → 4× `if` und bleibt bei einer flachen
`else if`-Kette grün.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die ab SP03 Go-Code schreiben; die Review-Session.

## Anforderungen

B-057 › Anforderungen. Sprint-eigen: `tests/projectRules.test.ts` zählt die Dateilänge auch für `.go`-Dateien in
`tests/`, damit die neue Prüfung selbst unter das Budget fällt.

## Nicht-Ziele

Andere Budget-Grenzen ändern; Oxlint anfassen; Ratsche nachziehen (B-056); eigener golangci-lint-Linter.

## Regeln und Einschränkungen

Einschiebbar, soll vor SP03 laufen. Nur Standardbibliothek. Die Prüfung liegt als `tests/nesting_test.go`
(Paket `tests`, nur Test-Datei); eine Probe in L2-Planung hat gezeigt, dass `go build ./...`, `go vet`, die Cross-Builds
und `golangci-lint` damit grün bleiben.

## Beispiele

B-057 › Beispiele.

## Ausnahme- und Fehlerfälle

B-057 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** `range` → 4× `if` scheitert, `range` → 3× `if` nicht (B-057/AC-01).
- **AC-02** Eine flache `else if`-Kette aus 6 Zweigen bleibt grün (B-057/AC-02).
- **AC-03** `for`, `switch` und `select` zählen je eine Ebene, ein Funktionsliteral beginnt bei 0 (B-057/AC-03).
- **AC-04** `revive` ist entfernt, `.golangci.yml` und `docs/arbeitsweise.md` › Werkzeuge nennen die neue Prüfung (B-057/AC-04).
- **AC-05** Die Dateilänge von `.go`-Dateien in `tests/` wird wie unter `data/`, `engine/`, `cmd/` geprüft.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| L2.1 | `L2.1-tiefen-pruefung.md` | Umsetzung | autonom | fertig |
| L2.2 | `L2.2-review.md` | Review | autonom | fertig |

## Abnahme

**2026-09-30, L2.2 (autonomer Agent, nur lokal).** Geprüft: 9 Dateien aus `git diff --stat 27cf540..main`
(lokales `main` 9b4e1ac; kein `git fetch`, 🧑: nur lokal), jede vollständig gelesen: `.golangci.yml`,
`docs/arbeitsweise.md`, `docs/sprints/README.md`, die drei Dateien dieses Sprints, `requirements.md`,
`tests/nesting_test.go`, `tests/projectRules.test.ts`. Lokal: Go 1.27.0, golangci-lint 2.14.0.
Gegenproben in `engine/probe/` (Go, `go test ./tests/`) und `src/` (TypeScript, `npx oxlint`), danach gelöscht.

- **AC-01 geprüft:** `for _, x := range` → 4× `if` → rot (Tiefe 5), `for range` → 3× `if` in derselben Probe nicht
  gemeldet; Methode mit Empfänger, `range` → 4× `if` → rot mit dem Namen der Methode.
- **AC-02 geprüft:** `else if`-Ketten zählen nicht: Rumpf eines `else if` mit 3× `if` grün, mit 4× `if` rot; Kette mit
  `else` am Ende und 4× `if` darin rot, wie bei Oxlint (das `if` im `else`-Block zählt).
- **AC-03 geprüft:** Typ-`switch` → 4× `if` rot, → 3× `if` grün; `select` → `for` → 3× `if` rot; `switch` mit Init →
  `switch` → 3× `if` rot; `for` mit Label + `continue` (4× `for` → `if`) rot. Funktionsliterale beginnen bei 0:
  3× `for` → Literal → Literal → 4× `if` grün, Literal im Literal mit 5× `if` rot, Literal in `if`-Bedingung
  (unter 4× `if`, darin 4× `if`) grün, Literal im `if`-Init mit 5× `if` rot, `go func` unter 4× `for` grün,
  Literal in `for`-Bedingung unter 3× `for` grün, Literal als Paketvariable mit 5× `if` rot. `else` ohne `if` zählt
  nicht (3× `if` darin grün, 4× rot), nackte Blöcke und `goto` zählen nicht. Dieselben strittigen Fälle in TypeScript
  (`else`-Block, Literale in Bedingung/Init, Label, nackte Blöcke, `else if`-Rumpf, `switch` → `switch`, Paketvariable):
  Oxlint meldet genau dieselben rot. Syntaxfehler in Unterordner → rot mit Datei und Parser-Fehler. Die Prüfung selbst:
  114 Zeilen, längste Funktion 24 Zeilen, `gocyclo` höchstens 9 (`walk`), Tiefe ≤ 4 (prüft sich selbst).
- **AC-04 geprüft:** `revive` steht in keiner `.yml`, `.go`, `.ts` oder `.json` mehr (nur noch in Sprint- und
  Ticket-Historie); `golangci-lint config verify` grün; `golangci-lint run --max-same-issues 0` auf den Proben meldet nur
  `staticcheck`/`unused`, keine Tiefe. Kopfkommentar `.golangci.yml` und `docs/arbeitsweise.md` › Werkzeuge nennen
  `tests/nesting_test.go`.
- **AC-05 geprüft:** Probe `tests/zz_probe_test.go` mit 401 Zeilen → `projectRules.test.ts` rot
  („keine Go-Datei über der harten Grenze“), gelöscht.
- **Weiter geprüft:** `requirements.md` › „Installation unter Windows“ (aus 986de25): `winget show` findet `Git.Git`,
  `OpenJS.NodeJS.22`, `GoLang.Go` (1.27.0), `GolangCI.golangci-lint` (2.14.0, `--version 2.14.0` verfügbar); passt zu
  `.nvmrc` (22), `go.mod` (1.27.0) und `ci.yml` (v2.14). `npm run check` und `npm run check:go` grün.
- **Behoben:** `requirements.md`: Mindestversion „golangci-lint 2.x“ → „2.14“ (passt zur Tabelle), Anführungszeichen
  „Add to PATH“; `tests/nesting_test.go`: Literal auf Paketebene heißt in der Meldung „Funktionsliteral in Paketebene“
  statt eines leeren Namens.
- **Neue Tickets:** B-058 (`Set-ExecutionPolicy` in `requirements.md` ist eine Sicherheitseinstellung, 🧑 entscheidet).
