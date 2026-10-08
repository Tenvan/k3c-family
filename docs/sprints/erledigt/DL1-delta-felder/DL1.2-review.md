# DL1.2 · Review und Abnahme des Sprints DL1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** dl1/2-review
- **Abhängig von:** DL1.1
- **Tickets:** B-297
- **Kriterien:** alle

## Ziel

DL1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, liegt in `docs/sprints/erledigt/`, B-297 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff). Besonders: `null` bleibt ein Wert, `events` nie in `unset`, Client und
Server verhalten sich gleich, der Delta-Test deckt weiter alle geprüften Felder ab.

## Erlaubte Dateien

- `engine/net/delta*.go`, `src/online/clientDelta*`, `docs/protocol.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung.

## Schritte

1. `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/dl1` lesen, Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-04 aus dem Ergebnis von DL1.1 prüfen.
4. Abnahme (höchstens fünf Zeilen) mit Versionsvorschlag in die Sprint-README.
5. B-297 auf `erledigt` und nach `docs/backlog/archiv/`; Sprint nach `docs/sprints/erledigt/`, `Status: erledigt`,
   Fahrplan anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-04 haben einen Nachweis im Ergebnis von DL1.1.
- [x] Schwere Befunde behoben oder als Ticket; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

- Leichter Review des Diffs `origin/develop...origin/sprint/dl1`: **keine schweren Befunde.** `null` bleibt Wert (`unsetOf` prüft nur das Fehlen des Schlüssels, Test `deltaOf(cur, cur)`), `events` nie in `unset` (`k != "events"`), Server (`apply` im Go-Test) und Client (`applyDelta`) entfernen dieselben Felder, `unset` ist sortiert und fehlt, wenn leer; der Delta-Test prüft alle bisherigen Felder weiter. Die Burgtreffer-Abweichung in AC-03 läuft weiter über `stateOf`/`deltaOf`/`apply`, `castle` bleibt echt abgedeckt.
- Geprüft: `tsc`, `oxlint` (0 Fehler), Vitest (1337), `go test ./...`, `golangci-lint` (0 issues) grün; `tests/planning.test.ts` grün.
- AC-01 bis AC-04 haben ihren Nachweis im Ergebnis von DL1.1. Abnahme und Versionsvorschlag in der Sprint-README; B-297 archiviert, Sprint nach `docs/sprints/erledigt/`.
