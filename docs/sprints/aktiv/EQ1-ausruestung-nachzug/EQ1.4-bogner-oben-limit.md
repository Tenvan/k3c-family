# EQ1.4 · Bogner nur oben, Truppen-Limit für die ganze Insel

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eq1/4-bogner-oben-limit
- **Abhängig von:** EQ1.3
- **Tickets:** B-1208
- **Kriterien:** AC-04

## Ziel

Der Bogner steht nur ganz oben; das Truppen-Limit zählt Kämpfer aller Stufen einer Insel.

## Kontext

Ticket B-1208. `topOnly` in `data/hub.json` (ECO1.6, `world/lines.rs`), Limit in `state/links.rs` (`troop_limit`, `fighters`). Nur mit nativen Regeln; Datenänderung: `testdata/rust/data-tree.json.gz` neu schreiben.

## Erlaubte Dateien

- `data/hub.json`
- `rust/crates/k3c-sim/src/world/**`
- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-sim/src/island/**`
- `testdata/rust/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `topOnly` am Bogner, Taverne bleibt.
2. Limit über alle Stufen zählen.
3. Tests AC-01, AC-02, Go-Golden.

## Fertig, wenn

- [ ] AC-04: Bogner nur in Tiefe 0, Limit zählt über Stufen, Go-Golden grün (B-1208 AC-01 bis AC-03).
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
