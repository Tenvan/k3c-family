# BAL1.4 · Review und Abnahme des Sprints BAL1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** bal1/4-review
- **Abhängig von:** BAL1.3
- **Tickets:** B-099, B-159
- **Kriterien:** alle

## Ziel

Der Sprint BAL1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-159 ist archiviert, B-099 bleibt offen für BAL2 und BAL3.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt das Balance-Paket und den Befehl (Ort aus BAL1.1), `testdata/replay/`, `Taskfile.yml` und k3c-dev. Besonders prüfen: Bots greifen nur über `PlayerCommand` ein (kein Schreiben in die Welt); kein `math/rand`, keine Wanduhr, keine unsortierte Map-Iteration; Determinismus-Test nicht gelockert; `engine/sim` und `data/` unverändert; das k3c-dev-Werkzeug liest nur erlaubte Pfade; keine neue Abhängigkeit ohne Zustimmung.

## Erlaubte Dateien

- Balance-Paket, Befehl, `tools/k3c-dev/internal/mcpsrv/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Werte in `data/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check:go`, `task check` und `task check:dev` grün.
2. `git fetch && git diff origin/develop...origin/sprint/bal1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-07 aus den Ergebnissen von BAL1.1 bis BAL1.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-159 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); B-099 bleibt `eingeplant` (BAL2, BAL3), Notiz zum Stand.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-07 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check:go
task check
task check:dev
```

## Ergebnis

–
