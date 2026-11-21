# FG1.1 · Landstreicher und Bauer zu Bürger (citizen) zusammenlegen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** fg1/1-buerger-citizen
- **Abhängig von:** ECO1
- **Tickets:** B-1199
- **Kriterien:** AC-01

## Ziel

`vagrant` (Landstreicher) und `peasant` (Bauer) sind in Sim, Daten, Spiel, Texten und Tests ein Typ: `citizen` (Bürger).

## Kontext

Ticket B-1199, Entscheidung Ralf 2026-11-21. Erst nach ECO1 (Handwerker). Go-Parität: Go-Golden-Läufe laufen mit `native_rules` aus und kennen nur `vagrant`/`peasant`; die Golden-Dateien dürfen sich nicht ändern, daher Abbildung `citizen` ↔ Go-Namen an der Golden-Grenze oder Golden begründet neu schreiben (`go test ./internal/detvec/ -write`, Begründung im Commit). Spielstand-Version erhöhen, alte Stände egal.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-data/**`
- `rust/crates/k3c-game/**`
- `data/troops.json`
- `data/economy.json`
- `testdata/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Alle Vorkommen von `vagrant` und `peasant` per grep erfassen.
2. Zu `citizen` zusammenlegen, Texte „Bürger“/„Citizen“.
3. Golden-Grenze lösen, Tests anpassen.
4. `cargo test` der betroffenen Crates.

## Fertig, wenn

- [ ] AC-01: `citizen` ersetzt `vagrant` und `peasant`; Texte de/en.
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
