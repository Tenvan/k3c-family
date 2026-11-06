# MG6.6 · Sprint-Review MG6

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg6/6-review
- **Abhängig von:** MG6.1, MG6.2, MG6.3, MG6.4, MG6.5
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG6 ist reviewt (frischer Reviewer, Opus), Befunde behoben oder als D-Zeile/Ticket, Abnahme in der README.

## Kontext

Diff `origin/feature/pap..sprint/mg6`: nur `public/grafik/gebaeude/` (neue `*.paper.json`, Ausgabe-Ordner, `index.paper.json`, `golden.json`) und Planung. Kriterien AC-01–AC-03 der README.

## Erlaubte Dateien

- `public/grafik/gebaeude/*.paper.json` der Sprint-Gebäude, ihre Ausgabe-Ordner, `index.paper.json`, `golden.json`
- `docs/sprints/aktiv/MG6-papier-gebaeude/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine Sichtprüfung durch 🧑 (PS1).

## Schritte

1. Reviewer-Agent über den Sprint-Diff (Specs, Stufen-Ausgabe, Golden).
2. Befunde beheben (über MCP) oder D-Zeile/Ticket.
3. `task check`, Abnahme-Abschnitt füllen, Commit.

## Fertig, wenn

- [ ] AC-01–AC-03 mit Nachweis aus MG6.1–MG6.5 bestätigt.
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
