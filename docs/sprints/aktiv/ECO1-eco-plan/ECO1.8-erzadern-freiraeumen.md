# ECO1.8 · Erzadern erst nach dem Freiräumen abbaubar

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** eco1/8-erzadern-freiraeumen
- **Abhängig von:** ECO1.4
- **Tickets:** B-1198
- **Kriterien:** AC-08

## Ziel

Erzadern in den Minen sind erst nach dem Freiräumen von Felsen und Steinen auf dem Weg zum Hub abbaubar.

## Kontext

Ticket B-1198, Handwerker mit Spitzhacke aus ECO1.4. Default: ein bis drei Hindernisse je Ader. Level-Generator `k3c-level`, Sim `k3c-sim`; deterministisch.

## Erlaubte Dateien

- `rust/crates/k3c-level/src/**`
- `rust/crates/k3c-sim/src/world/**`
- `rust/crates/k3c-sim/src/unit/**`
- `data/biomes/mine.json`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Adern und Felsen im Level (`k3c-level`) lesen.
2. 1 bis 3 Hindernisse je Ader auf dem Weg; Abbau erst, wenn sie weg sind.
3. Tests über viele Seeds.

## Fertig, wenn

- [ ] AC-08: Jede Ader hat freiräumbare Hindernisse; Abbau erst nach dem Freiräumen.
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
