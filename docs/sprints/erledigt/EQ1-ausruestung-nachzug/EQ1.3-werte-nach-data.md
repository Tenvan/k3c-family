# EQ1.3 · Ausrüstungswerte nach data/buildings.json

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** ?
- **Branch:** eq1/3-werte-nach-data
- **Abhängig von:** EQ1.1
- **Tickets:** B-1206
- **Kriterien:** AC-03

## Ziel

Rüstungspreis, Regalgrößen, Herstellzeiten und das Schwert-Zahlziel stehen als optionale Felder in `data/buildings.json`; die Sim liest sie von dort.

## Kontext

Ticket B-1206. Konstanten in `unit/hammer.rs` und `unit/warrior.rs`. Go liest die Felder nur mit (`engine/sim/data.go`); `testdata/rust/data-tree.json.gz` mit `go test ./internal/detvec/ -write` im Ordner `engine` neu schreiben, Grund im Commit. Zahlen unverändert.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/unit/**`
- `rust/crates/k3c-data/**`
- `data/buildings.json`
- `engine/sim/data.go`
- `testdata/rust/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konstanten erfassen.
2. Optionale Felder in k3c-data und `buildings.json`, Go-Struct.
3. Sim liest die Werte, Rückfall auf die alten Zahlen.
4. Golden neu schreiben, Tests AC-01 bis AC-03.

## Fertig, wenn

- [ ] AC-03: Werte kommen aus den Daten, ohne Feld gelten die alten Zahlen, Go-Golden grün (B-1206 AC-01 bis AC-03).
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

- AC-03 geprüft (B-1206 AC-01 bis AC-03): neue optionale Felder `armorRack` und `armorCost` an der Rüstkammer sowie `craftSeconds.sword` (10) an der Schmiede in `data/buildings.json`; `k3c-data` (`BuildingData`), Formtabelle und Go-Struct (`engine/sim/data.go`, nur mitgelesen) kennen sie. Die Sim liest sie (`unit/hammer.rs`: `armor_cost(v)`, `armor_rack(v)`, `sword_rack(v, kind)`; Herstellzeiten liefen schon über `craftSeconds`); ohne Feld gelten die alten Zahlen als Standardwert. Zahlen unverändert.
- Test `unit/equip_tests.rs::ausruestungswerte_kommen_aus_den_daten` (Wert aus den Daten wirkt, ohne Feld alter Wert).
- Golden `testdata/rust/data-tree.json.gz` neu geschrieben (`go test ./internal/detvec/ -write`), Grund: neue Datenfelder. `go test ./sim/` und `./internal/detvec/` grün.
- `cargo test -p k3c-sim`: nur die bekannten Roten (`insel_nach_1000_schritten_wie_go`, `eingebettete_welten_gueltig`, B-1190 AC-04); fmt, clippy sauber. `task check:diff`: Regeln, Go, fmt, lint grün, rust-test nur an den bekannten Roten.
- Abweichung: `k3c-sim/src/world/values_shape_buildings.rs` (Formtabelle) und `unit/mod.rs` außerhalb der erlaubten Dateien, nötig für die neuen Felder.
