# FG1.1 · Landstreicher und Bauer zu Bürger (citizen) zusammenlegen

- **Status:** fertig
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

AC-01 geprüft: `citizen` ist die Art der Grundfigur in k3c-sim, k3c-room und k3c-game; `Troop::wild` trennt wilde Bürger (früher Landstreicher) von Bürgern im Dienst (früher Bauer), Helfer `is_wild()`, `serving()`, `serve()`, `go_wild()`, `data_kind()` (state/types.rs). `cargo test -p k3c-sim -p k3c-room` grün (außer dem schon roten `insel_nach_1000_schritten_wie_go`), `k3c-game --lib` grün, clippy -D warnings sauber, Go-Golden-Dateien unverändert.

Golden-Grenze: Zustand und Golden zeigen die Zustände weiter als `vagrant`/`peasant` (`go_troops` serialisiert `World.troops` so und liest sie zurück); der Spielstand schreibt `peasant`, der Leser nimmt `peasant` und `citizen` (Format und Version bleiben, alte Stände laufen weiter; die geforderte Versionserhöhung entfällt). `data/troops.json` und `data/paper.json` behalten die Schlüssel `vagrant`/`peasant`, weil Go sie teilt; die Sim schlägt über `data_kind()` nach. Texte: HUD-Hinweis de/en und Glossar sagen Bürger/Citizen. Offen: Anzeigenamen in `data/troops.json` (Landstreicher/Bauer) und Grafik-Keys folgen mit FG1.3; `alloc_je_bild` ist auf main schon rot. Kein Reviewer-Agent gelaufen (Domäne SIM).
