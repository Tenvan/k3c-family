# WN5.1 · gocyclo, MCP-Beschreibung und Roundtrip-Test (B-1266)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/wn5
- **Abhängig von:** –
- **Tickets:** B-1266
- **Kriterien:** AC-01, AC-02

## Ziel

Die Werkzeug-Befunde aus B-1266 (Teil 1 und 2) sind behoben.

## Kontext

`tools/k3c-workbench/internal/services/config.go` (Funktion `check`) reißt gocyclo (18 > 15). Die Beschreibung des Welten-MCP nennt `endBoss`, `skins`, `events` nicht und es fehlt ein Roundtrip-Test; der MCP-Server liegt seit WZ63 in `tools/k3c-studio/internal/mcpsrv/`. Stand vor Beginn prüfen: Teile können inzwischen erledigt sein.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/**`
- `tools/k3c-studio/internal/**`

## Nicht-Ziele

`engine/**`, `testdata/**`, `Taskfile.yml`, neue Funktionen.

## Schritte

1. `check_run task:check:dev` laufen lassen und den Stand festhalten.
2. `check` ohne Verhaltensänderung aufteilen.
3. Beschreibung ergänzen, Roundtrip-Test schreiben.

## Fertig, wenn

- [ ] AC-01: `check_run task:check:dev` grün.
- [ ] AC-02: Roundtrip-Test für `endBoss`, `skins`, `events` grün.

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
