# EQ1.4 · Bogner nur oben, Truppen-Limit für die ganze Insel

- **Status:** fertig
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

- AC-04 geprüft (B-1208 AC-01 bis AC-03): `data/hub.json`: Bogner `topOnly` (Taverne bleibt), unten nur Kampfgebäude. Das Truppen-Limit zählt die ganze Insel (nur native Regeln): `World::fighters_elsewhere`/`limit_elsewhere` (serde skip), gesetzt je Takt in `Island::stage_ref` (`island/errand.rs::share_limit`); Basis einmal, Zuschlag je gebauter Kaserne jeder Stufe; Waffen-Holer anderer Stufen zählen mit. `weapon_to_fetch`, Besorgung (`surface_gear`) und HUD-Wirtschaft nutzen `island_fighters`/`island_limit`; ohne native Regeln bleiben die Felder 0 (Go-Parität).
- Tests `island/limit_tests.rs` (4): Bogner nur oben, Limit über Stufen, Go-Lauf je Stufe, Holer zählen mit. Golden `testdata/rust/data-tree.json.gz` neu geschrieben (hub.json geändert), Go-Tests grün.
- Review (k3c-reviewer): mittlerer Befund (Holer anderer Stufen fehlten) behoben und getestet.
- Abweichung: außerhalb der erlaubten Dateien `building/barracks.rs`, `unit/warrior.rs`, `economy/view.rs`, `state/types_world.rs`, `links.rs`.
- Nur die bekannten Roten (B-1190 AC-04). Spielgefühl (zwei Stufen am Limit) prüft Ralf im Abnahme-Ticket.
