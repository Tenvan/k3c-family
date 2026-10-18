# CI1.3 · Review des Sprints CI1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** ci1/3-review
- **Abhängig von:** CI1.1, CI1.2
- **Tickets:** B-273, B-019
- **Kriterien:** AC-01, AC-02

## Ziel

Ein getrennter Reviewer hat Diff und Kriterien von CI1 geprüft; Befunde sind behoben oder als Ticket festgehalten, der Sprint ist abgeschlossen.

## Kontext

Review-Session nach `docs/arbeitsweise.md` (getrennt vom Schreiben): Diff von `sprint/ci1` gegen `origin/develop` lesen, jedes Kriterium AC-01 und AC-02 am Ergebnis von CI1.1 und CI1.2 prüfen, CI-Lauf des Sprint-Branchs (Jobs `docker` und `go`) einsehen. Sprint-Ergebnis: Version vorschlagen (keine, reines CI-Thema, Image-Standard ändert sich für den Pi: Minor oder Patch entscheidet 🧑).

## Erlaubte Dateien

- Befunde: nur kleine Korrekturen in den Dateien von CI1.1 und CI1.2 (`Dockerfile`, `compose.yaml`, `Taskfile.yml`, `.github/workflows/ci.yml`); größere Befunde als Ticket
- Sprint-Dateien von CI1 (Abnahme, Status, Ordner nach `erledigt/`), `docs/sprints/README.md`, `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Neue Funktionen, neue Prüfungen über die Abdeckung hinaus, Ändern von Kriterien.

## Schritte

1. Diff `origin/develop...sprint/ci1` lesen, Ergebnisse von CI1.1 und CI1.2 gegen die Kriterien halten.
2. CI-Lauf des Sprint-Branchs prüfen (`gh_status`): Docker-Job (`dev=false`, `dev=true`) und Go-Job (Abdeckung in der Zusammenfassung) grün.
3. Befunde beheben oder als Ticket anlegen; `task check` grün.
4. Abnahme-Abschnitt der Sprint-README füllen (höchstens fünf Zeilen), Sprint abschließen (`plan_set Status erledigt`), Tickets B-273 und B-019 `erledigt`.

## Fertig, wenn

- [ ] AC-01 und AC-02 sind am Diff und am CI-Lauf belegt (Nachweis im Ergebnis).
- [ ] Befunde sind behoben oder als Ticket festgehalten; `task check` grün.
- [ ] Abnahme der Sprint-README ist gefüllt, Sprint, B-273 und B-019 sind `erledigt`.

## Prüfen

```bash
task check
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
