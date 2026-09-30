# B-057 · Die Go-Verschachtelung zählt `for range` und `else if` wie TypeScript

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** L2
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit L2)

## Ausgangslage

Seit L1 prüft `.golangci.yml` die Go-Verschachtelung mit `revive` › `max-control-nesting` 4 (B-054). Gegenproben im
Review L1.2 (golangci-lint 2.14) zeigen zwei Abweichungen vom Budget in `docs/arbeitsweise.md` und von Oxlint
`max-depth`: `for range` zählt keine Ebene (`range` → 4× `if`, Tiefe 5, bleibt grün), und jedes `else if` zählt eine
Ebene (flache Kette mit 5 Zweigen, Tiefe 1, scheitert). Fünf direkt verschachtelte `select` melden ebenfalls nichts.
Go ist damit bei der häufigsten Schleife lockerer und bei `else if`-Ketten strenger als das Budget.

## Ziel

Die harte Grenze Verschachtelung 4 bedeutet in Go dasselbe wie in TypeScript, auch für `for range` und `else if`.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die ab SP03 Go-Code schreiben; Review-Session.

## Anforderungen

- Eine eigene Prüfung als Go-Test (nur Standardbibliothek: `go/parser`, `go/ast`) misst die Verschachtelungstiefe
  jeder Funktion in allen `.go`-Dateien unter `data/`, `engine/`, `cmd/`, auch in `*_test.go`.
- Zählweise wie Oxlint/ESLint `max-depth`: `if`, `for`, `for range`, `switch`, Typ-`switch` und `select` erhöhen die
  Tiefe um 1; ein `if` im `else`-Zweig eines `if` (`else if`) erhöht sie nicht; `case`-Klauseln zählen nicht;
  ein Funktionsliteral beginnt wieder bei 0.
- Tiefe über 4 lässt `go test ./...` und damit `npm run check:go` scheitern, die Meldung nennt Datei, Zeile und Tiefe.
- `revive` › `max-control-nesting` ist danach entfernt; Kommentar in `.golangci.yml` und `docs/arbeitsweise.md` ›
  Werkzeuge nennen die neue Prüfung.

## Nicht-Ziele

Andere Budget-Grenzen ändern; Oxlint oder TypeScript-Regeln anfassen; einen eigenen golangci-lint-Linter bauen.

## Regeln und Einschränkungen

Keine neue Abhängigkeit (nur Standardbibliothek); Komplexitäts-Budget gilt auch für die Prüfung selbst;
Grenzen nur aus `docs/arbeitsweise.md` › Komplexitäts-Budget. Domäne INF.

## Beispiele

- `for _, v := range xs` → `if` → `if` → `if` → `if` (Tiefe 5) → `npm run check:go` scheitert (heute grün).
- `if` / `else if` / `else if` / `else if` / `else if` / `else if` ohne innere Struktur (Tiefe 1) → grün (heute rot).
- Funktionsliteral in Tiefe 3, darin 4× `if` → grün (das Literal beginnt bei 0, wie bei Oxlint).

## Ausnahme- und Fehlerfälle

- Datei lässt sich nicht parsen → der Test scheitert mit Datei und Parser-Fehler (statt sie zu überspringen).
- Noch kein Go-Code unter `engine/` oder `cmd/` → Ordner überspringen, Test grün.

## Akzeptanzkriterien

- **AC-01** Eine Go-Probe `range` → 4× `if` lässt `npm run check:go` scheitern, `range` → 3× `if` nicht (Proben zurücknehmen).
- **AC-02** Eine Go-Probe mit flacher `else if`-Kette aus 6 Zweigen bleibt grün.
- **AC-03** `for` → 4× `if`, `switch` → `for` → 3× `if` und `select` → 4× `if` scheitern; ein Funktionsliteral beginnt bei 0 (Proben).
- **AC-04** `revive` ist aus `.golangci.yml` entfernt; `.golangci.yml` und `docs/arbeitsweise.md` › Werkzeuge nennen die neue Prüfung.

## Offene Fragen

keine. Entschieden (🧑, 2026-09-30): eigene Tiefen-Prüfung mit `go/ast` (Weg c), `revive` wird entfernt.

## Notizen

Proben und Zählweise: B-054 › Notizen (Review L1.2). Ursache vermutet, ungeprüft: `max-control-nesting` wertet
`*ast.RangeStmt` nicht aus und läuft über `IfStmt.Else` mit erhöhter Ebene. Stolperfalle bei Proben: golangci-lint
zeigt standardmäßig nur 3 gleiche Meldungen (`max-same-issues`); für Proben `--max-same-issues 0` setzen.
