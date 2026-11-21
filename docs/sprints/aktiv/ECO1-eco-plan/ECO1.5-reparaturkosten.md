# ECO1.5 · Reparatur kostet 1 Grundmaterial und 2 Gold

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/5-reparaturkosten
- **Abhängig von:** ECO1.4
- **Tickets:** B-1195
- **Kriterien:** AC-05

## Ziel

Reparatur kostet immer genau 1 Grundmaterial der Ausbaustufe und 2 Gold.

## Kontext

Ticket B-1195, Handwerker aus ECO1.4. Heute reparieren Bürger kostenlos zwischen den Wellen. Default: je Vorgang bis volle HP. Zahlstelle am Gebäude. Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/economy/**`
- `data/buildings.json`
- `data/economy.json`
- `docs/rules/materialien-gebaeude.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Reparatur-Logik finden (grep repair).
2. Kosten buchen: 1 Material der Stufe, 2 Gold; ohne Mittel keine Reparatur.
3. Regeltext.
4. Tests.

## Fertig, wenn

- [ ] AC-05: Reparatur bucht 1 Grundmaterial der Stufe und 2 Gold.
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
