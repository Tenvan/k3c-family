# NG3.5 · Review Figuren

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** figuren
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng3/5-review
- **Abhängig von:** NG3.4
- **Tickets:** B-796, B-791, B-780
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Sprint NG3 ist abgenommen, PR offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Rig-Mathematik, Allokationen je Bild, Batching). Prüfen: `view` und `game` unverändert, Importregel.

## Erlaubte Dateien

- `native/k3c-game/actors/**`
- `native/k3c-rig/rig/**`
- Planung NG3, B-796, B-791, B-780 (über MCP)

## Nicht-Ziele

Neue Funktionen; Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. `task check`, `task check:go`, `go test ./...` in `native/k3c-game` und `native/k3c-rig`.
2. Diff lesen, schwere Befunde beheben.
3. Abnahme, Abnahme-Ticket, PR mit Auto-Merge; B-780 bleibt offen bis NG9.5.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03, AC-04: Nachweise geprüft.
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
