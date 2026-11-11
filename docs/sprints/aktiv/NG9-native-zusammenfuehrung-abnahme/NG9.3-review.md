# NG9.3 · Review Zusammenführung und Abnahme-Ticket des Projekts

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng9/3-review
- **Abhängig von:** NG9.2
- **Tickets:** B-802
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint NG9 ist abgenommen, das Abnahme-Ticket des Projekts NAT ist angelegt, der PR offen; die Mensch-Sessions NG9.4 und NG9.5 bleiben offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer`. Hardware entkoppelt: AC-04 und AC-05 als „angenommen, Validierung offen (NG9.4, NG9.5)“ führen; der Sprint bleibt aktiv. Abnahme-Ticket des Projekts NAT nach B-742 (nimmt offene Abnahme-Tickets der NG-Sprints auf); Sim und Balance `verschoben (Sperre 2026-11-11)`.

## Erlaubte Dateien

- `native/k3c-game/game/**`
- `native/k3c-game/e2e/**`
- `native/Taskfile.yml`
- `native/measure.ps1`
- Planung NG9, B-802, B-786, NAT (über MCP)

## Nicht-Ziele

Neue Funktionen; Abschalten von Server und Browser-Client.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket des Projekts, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03: Nachweise geprüft.
- [ ] Abnahme-Ticket NAT angelegt; PR offen, Auto-Merge an.

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
