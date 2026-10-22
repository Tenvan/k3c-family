# SO6.5 · Review und Abnahme des Sprints SO6

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** so6/5-review
- **Abhängig von:** SO6.4
- **Tickets:** B-493, B-494, B-492
- **Kriterien:** alle

## Ziel

SO6 ist nach `docs/arbeitsweise.md` › Review-Session geprüft; Abnahme in der Sprint-README.

## Kontext

Reviewer-Agent getrennt (Diff berührt Pfad-Code im Go-Werkzeug und den Audio-Kern). Die Hörprobe SO6.6 und der Workshop SO6.3 sind keine Abhängigkeit (Hardware entkoppelt): offen → Kriterien `angenommen, Validierung offen`.

## Erlaubte Dateien

- Dateien des Sprints nur für schwere Befunde; `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Geschmack der Sounds.

## Schritte

1. `task check`, `task check:go` nacheinander, Diff lesen.
2. Nachweis je Kriterium aus SO6.1, SO6.2, SO6.4.
3. Abnahme schreiben; Sprint bleibt `aktiv`, solange SO6.3/SO6.6 offen sind.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis, AC-04 offen (SO6.6) oder belegt.
- [ ] Schwere Befunde behoben.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
