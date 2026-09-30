# L2 · INF · Go-Tiefe wie TypeScript zählen

- **Status:** aktiv
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
| L2.2 | `L2.2-review.md` | Review | autonom | offen |

## Abnahme

–
