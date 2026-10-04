# LT1.4 · Review und Abnahme des Sprints LT1

- **Status:** in Arbeit
- **Typ:** Review
- **Agent:** autonom
- **Branch:** lt1/4-review
- **Abhängig von:** LT1.2
- **Tickets:** B-175
- **Kriterien:** alle

## Ziel

Der Sprint LT1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-175 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `cmd/k3c-load/`, `engine/net/status.go` und `Taskfile.yml`. Besonders prüfen: Token erscheint nie in Ausgabe, Log oder Bericht; das Werkzeug legt nur `test-`-Räume an und löscht nichts anderes; `cmd/k3c-load` importiert nichts aus `engine/room` oder `engine/sim`; `cpu` verändert die Status-Antwort sonst nicht und braucht weiter das Token; keine neue Abhängigkeit in `go.mod`; Bewertung senkt das Ziel nicht. LT1.3 (Messlauf am Pi) ist keine Abhängigkeit: ist sie offen, führt die Abnahme AC-06 als `angenommen, Validierung offen (LT1.3)` und LT1.3 steht im Fahrplan unter „Offen am Gerät“.

## Erlaubte Dateien

- `cmd/k3c-load/`, `engine/net/`, `Taskfile.yml` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Messung am Pi.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/lt1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von LT1.1 bis LT1.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-175 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen (LT1.3 ggf. unter „Offen am Gerät“), PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-05 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] AC-06 ist nachgewiesen oder als `angenommen, Validierung offen (LT1.3)` geführt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt; `task check` und `task check:go` grün; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
