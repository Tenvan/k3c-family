# ECO1.7 · Ausrüstung von oben oder vom Boden holen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/7-ausruestung-von-oben
- **Abhängig von:** ECO1.6
- **Tickets:** B-1197
- **Kriterien:** AC-07

## Ziel

Unten ohne Regal holt ein Bürger liegende Ausrüstung oder wandert nach oben, um sie zu holen.

## Kontext

Ticket B-1197, Gebäude nach Tiefe aus ECO1.6. Fallen gelassene Stücke gibt es seit B-929. Weg über die Treppen, deterministisch, 2+ Spieler. Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/unit/**`
- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/island/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wie Bürger heute Waffe und Hammer aus dem Regal holen (`unit/jobs.rs`, `fetch_weapon`).
2. Ohne Regal in der Stufe: liegende Ausrüstung suchen, sonst zur Oberfläche laufen.
3. Tests.

## Fertig, wenn

- [ ] AC-07: Unten ohne Regal holt ein Bürger liegende Ausrüstung oder wandert nach oben.
- [ ] `task check:diff` grün.

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
