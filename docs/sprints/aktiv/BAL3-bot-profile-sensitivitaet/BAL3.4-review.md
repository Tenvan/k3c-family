# BAL3.4 · Review und Abnahme des Sprints BAL3

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** bal3/4-review
- **Abhängig von:** BAL3.3
- **Tickets:** B-158
- **Kriterien:** alle

## Ziel

Der Sprint BAL3 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-158 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt das Balance-Paket (Profile, Sensitivität, Kurven), `cmd/k3c-balance/`, `Taskfile.yml` und die Tester-Dokumentation. Besonders prüfen: Bots greifen nur über `PlayerCommand` zu; Zufall nur über `engine/rng`, kein `math/rand`, keine Wanduhr; kein Kind-Bot und keine Fehler-Daten (B-158/AC-02); der Sensitivitäts-Lauf ändert keine Datei in `data/`; unbekannter Pfad und zu wenige Spieler ergeben Fehler; mit 2 und 4 Spielern kein falsches Verhalten; kein Kriterium wurde umformuliert.

## Erlaubte Dateien

- Balance-Paket, `cmd/k3c-balance/`, `Taskfile.yml`, neue Daten in `data/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Änderung von Beschlüssen oder Werten.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check:go` und `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/bal3` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von BAL3.1 bis BAL3.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag (Minor: Sprint mit Wirkung im Werkzeug).
5. B-158 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check:go
task check
```

## Ergebnis

–
