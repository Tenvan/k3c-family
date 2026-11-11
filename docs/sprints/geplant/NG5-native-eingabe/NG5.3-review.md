# NG5.3 · Review Eingabe

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PLAT
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng5/3-review
- **Abhängig von:** NG5.2
- **Tickets:** B-798
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint NG5 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Schwerpunkt: Regeln B und View + Menu, zwei Spieler, keine Eingabe ohne Prüfung. `view` und `game` unverändert.

## Erlaubte Dateien

- `native/k3c-game/input/**`
- Planung NG5, B-798 (über MCP)

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03: Nachweise geprüft.
- [ ] PR offen, Auto-Merge an.

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
