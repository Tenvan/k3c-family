# RU1.4 · Review Rust-Workspace

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru1
- **Abhängig von:** RU1.3
- **Tickets:** B-1014, B-1015, B-1018
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Diff des Sprints: `git diff origin/main...sprint/ru1`. Zusätzlich prüfen: Nichts außerhalb von `rust/`, `docs/standards/rust-port.md`, `tools/checkdiff`, `Taskfile.yml`, `ci.yml`, `tests/projectRules.test.ts`, `.gitignore` ist geändert; kein Go-Test ist angefasst.

## Erlaubte Dateien

- `rust/**`
- `docs/standards/rust-port.md`
- `tools/checkdiff/**`
- `Taskfile.yml`
- `.github/workflows/ci.yml`
- `tests/projectRules.test.ts`

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium AC-01 bis AC-06 mit dem Befehl aus der jeweiligen Session nachprüfen.
2. `task check` und `task check:go` nacheinander.
3. Ist-Werte für B-1015/AC-02 notieren: Dauer und Token je Session (aus den Ergebnissen der Sessions).
4. Abschnitt Abnahme der Sprint-README füllen, PR öffnen.

## Fertig, wenn

- [ ] Jedes Kriterium ist im Abschnitt Abnahme mit Nachweis genannt.
- [ ] `task check` und `task check:go` sind grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
