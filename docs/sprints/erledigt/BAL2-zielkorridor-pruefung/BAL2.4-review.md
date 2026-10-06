# BAL2.4 · Review und Abnahme des Sprints BAL2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** bal2/4-review
- **Abhängig von:** BAL2.3
- **Tickets:** B-157
- **Kriterien:** alle

## Ziel

Der Sprint BAL2 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-157 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `data/balance-targets.json`, das Balance-Paket, `testdata/balance/`, `Taskfile.yml` und `.github/workflows/ci.yml`. Besonders prüfen: Grenzen in den Daten entsprechen den bestätigten Zahlen in `docs/rules/zielkorridore.md` (Test vorhanden); keine Werte in `data/` außer den Korridoren geändert; Bewertung deterministisch; die CI bricht bei verletzten Zielen nicht, wohl aber bei Werkzeug-Fehlern; Baseline-Update mit Begründung im Commit; nicht messbare Ziele sind als Liste im Ergebnis von BAL2.1 geführt (ggf. Tickets).

## Erlaubte Dateien

- Balance-Paket, `data/balance-targets.json`, `.github/workflows/ci.yml` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Änderung der Korridore (REG).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check:go` und `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/bal2` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-07 aus den Ergebnissen von BAL2.1 bis BAL2.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-157 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); in B-099 notieren, dass AC-03 bis AC-05 mit BAL2 erfüllt sind.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-07 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check:go
task check
```

## Ergebnis

Review leicht: Diff gegen `origin/develop` gelesen, keine schweren Befunde, keine Änderung an Code. AC-01 bis AC-07 nachgewiesen (siehe Abnahme in der Sprint-README). CI-Nachweis AC-06: Lauf 37204084366 grün, Artefakt `k3c-balance`. Checks grün (`task check`, `task check:go`, `task check:dev`).
