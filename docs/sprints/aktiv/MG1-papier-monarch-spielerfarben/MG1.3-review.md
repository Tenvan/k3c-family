# MG1.3 · Sprint-Review MG1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** monarch
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg1/3-review
- **Abhängig von:** MG1.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG1 ist reviewt (frischer Reviewer), Befunde behoben oder als Ticket, Abnahme in der README.

## Kontext

Diff `origin/feature/pap..sprint/mg1`: nur `public/grafik/figuren/**` und Planung. Kriterien AC-01–AC-03 der README.

## Erlaubte Dateien

- `public/grafik/figuren/**`
- `docs/sprints/aktiv/MG1-papier-monarch-spielerfarben/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine Sichtprüfung durch 🧑 (PS1).

## Schritte

1. Reviewer-Agent über den Sprint-Diff.
2. Befunde beheben (über MCP) oder Ticket.
3. `task check`, Abnahme-Abschnitt füllen, Commit.

## Fertig, wenn

- [ ] AC-01–AC-03 mit Nachweis aus MG1.1/MG1.2 bestätigt.
- [ ] `task check` grün, Abnahme ausgefüllt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
