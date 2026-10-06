# M1 · SRV · k3c-dev I: MCP-Kern über HTTP

- **Status:** erledigt
- **Domäne:** SRV
- **Prio:** mittel
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-046
- **Start-Commit:** 7ff19d6
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-09-30 🧑 Chat (Revision 2, mit Abhängigkeit und Ausnahmen aus M1)

## Ausgangslage

Agenten prüfen über Shell-Befehle mit langer Rohausgabe und haben keine Logs. Ab SP01 gibt es ein Go-Modul mit
`golangci-lint` in der CI. M1 ist der erste von vier einschiebbaren Sprints für das Entwickler-Werkzeug `k3c-dev`
(M1 MCP-Kern, M2 Statistik und Spieldaten, M3 Oberfläche mit Logs-Seite, M4 MCP-Seite).

## Ziel

Agenten verbinden sich über HTTP mit `k3c-dev`, führen Prüfungen verdichtet aus und lesen Logs; jeder Aufruf wird
gezählt und protokolliert. Am Ende sichtbar: `go run .` in `tools/k3c-dev` läuft, Claude Code zeigt `k3c-dev` als
verbunden, `check_run npm:check` antwortet mit einer Zeile bzw. nur den Fehlerzeilen.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten am Entwickler-PC (Windows); 🧑 gibt Abhängigkeit und Ausnahmen frei.

## Anforderungen

B-046 › Anforderungen.

## Nicht-Ziele

B-046 › Nicht-Ziele. Insbesondere keine Oberfläche (M3, M4) und keine Statistik über Sitzungen (M2).

## Regeln und Einschränkungen

B-046 › Regeln und Einschränkungen. Dazu, mit der Freigabe dieser Revision genehmigt:

1. **Abhängigkeit:** `github.com/modelcontextprotocol/go-sdk` **v1.8.0** (aktuelle stabile Version, geprüft 2026-09-30),
   nur im Modul `tools/k3c-dev`.
2. **Ausnahmen außerhalb der Domäne** (INF), nur diese Stellen:
   - `package.json`: Script `check:dev` = `cd tools/k3c-dev && go test ./... && golangci-lint run`.
   - `.github/workflows/ci.yml`: neuer Job `k3c-dev` auf `windows-latest` (Go aus `tools/k3c-dev/go.mod`, `go test ./...`,
     `golangci-lint` v2.14 mit `working-directory: tools/k3c-dev`).
   - `tests/projectRules.test.ts` und `tests/nesting_test.go`: `tools` bzw. `../tools` in `goDirs` aufnehmen.
   - `.gitignore`: `.mcp.json`, `logs/`.
3. Das Hauptmodul bleibt unberührt: `go build ./...` im Repo baut `tools/k3c-dev` nicht mit (eigenes `go.mod`).

## Beispiele

B-046 › Beispiele.

## Ausnahme- und Fehlerfälle

B-046 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Modul, HTTP-Start und Roundtrip-Test mit Parameter-Hinweis (B-046/AC-01).
- **AC-02** `check_run` ist fail-closed und verdichtet, mit Zeitlimit und Konsolenpuffer (B-046/AC-02, B-046/AC-03).
- **AC-03** Zähler und Aufruf-Log (B-046/AC-04).
- **AC-04** Eigenes JSON-Log und Log-Tools mit Verdichtung (B-046/AC-05).
- **AC-05** Instructions, README, `.gitignore`, `check:dev` und CI-Job grün (B-046/AC-06).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M1.1 | `M1.1-geruest-zaehler.md` | Umsetzung | autonom | fertig |
| M1.2 | `M1.2-check-run.md` | Umsetzung | autonom | fertig |
| M1.3 | `M1.3-logs.md` | Umsetzung | autonom | fertig |
| M1.4 | `M1.4-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-09-30, leichtes Review über `7ff19d6..main` (61 Dateien) durch vier Reviewer (Sonnet) je Bereich plus Agent.
- Kriterien: AC-01 und AC-03 geprüft (M1.1), AC-02 geprüft (M1.2), AC-04 und AC-05 geprüft (M1.3); AC-05 zum Teil
  `verschoben`: CI-Job nie gelaufen, weil nicht gepusht → B-069. „Claude Code zeigt `k3c-dev` verbunden“ prüft 🧑.
- Behoben: Sessions ohne Leerlauf-Ende zählten nach Abbruch oder Neustart ewig als Clients (`SessionTimeout` 30 min,
  `Stop` schließt Sessions); Sperre je Ziel nach Panik dauerhaft gesetzt (`defer`); Testmuster mit führendem `-`
  (`--watch`) abgelehnt, strenger als die Positivliste in B-046.
- Neue Tickets: B-069. Hinweise in M3 (Job Object statt `taskkill /T`) und M4 (`node_modules` in den Regeltests).
