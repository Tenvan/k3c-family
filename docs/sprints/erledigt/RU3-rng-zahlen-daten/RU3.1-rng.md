# RU3.1 · rng in Rust, bitgleich zum Golden

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru3
- **Abhängig von:** –
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

`k3c-rng` liefert für jeden Seed dieselbe Zahlenfolge wie `engine/rng`.

## Kontext

Quelle: `engine/rng/rng.go` (85 Zeilen) und `rng_test.go`; Golden: `testdata/golden/rng.json`. Port-Leitfaden: `docs/standards/rust-port.md` (bis RU1.1 gemergt ist: `plan_get B-1018`, Abschnitt Anforderungen). Go rechnet Ganzzahlen umlaufend; Rust bricht im Debug-Bau bei Überlauf ab, deshalb jede Rechnung im Generator mit `wrapping_add`, `wrapping_mul` und Schieben auf festen Breiten (`u32`, `u64` wie in Go).

## Erlaubte Dateien

- `rust/crates/k3c-rng/**`

## Nicht-Ziele

Eine fremde rng-Crate. Neue Funktionen, die `engine/rng` nicht hat.

## Schritte

1. `engine/rng/rng.go` und `rng_test.go` ganz lesen; Aufbau von `rng.json` ansehen.
2. `src/lib.rs`: `pub struct Rng` mit denselben Feldern und Breiten; `Rng::new(seed: &str)` mit derselben Umrechnung vom Text zum Startwert; jede Methode von Go in `snake_case`, in derselben Reihenfolge.
3. `src/lib_tests.rs`: `rng.json` lesen (`serde_json` als Test-Abhängigkeit, Pfad ab `CARGO_MANIFEST_DIR`), jede Zahl vergleichen; die Tests aus `rng_test.go` portieren.
4. `cargo test -p k3c-rng`, `task rust:lint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-01: `cargo test -p k3c-rng` ist grün; der Golden-Test vergleicht jede Zahl aus `rng.json`.
- [ ] Im Quelltext steht kein `+` oder `*` auf dem Zustand ohne `wrapping_`.

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

- AC-01 geprüft: `cargo test -p k3c-rng` grün (5 Tests); `golden` vergleicht jede Zahl aus `rng.json` (`next` über `to_bits`, `int`, `weighted`, `shuffle`, `pick`, `hash`) für alle Seeds; `task check:diff` grün.
- Umlauf: `hash_seed` und `next` rechnen nur mit `wrapping_add`/`wrapping_mul` auf `u32`; Test `ueberlauf_rechnet_umlaufend` startet bei `u32::MAX`.
- Portierte Tests: `TestGolden` → `golden`, `TestUmlauteUndEmoji` → `umlaute_und_emoji`, `TestFehler` → `fehler_pick_leer` und `fehler_weighted_ohne_positives_gewicht` (`should_panic` mit Go-Text).
- Abweichungen: `shuffle` nimmt und liefert `Vec<T>` (Go: Slice in place, gleiche Reihenfolge). `serde_json` als Test-Abhängigkeit mit Feature `float_roundtrip`, damit Zahlen korrekt gerundet gelesen werden wie in Go (`rust/Cargo.lock` dadurch ergänzt).
- Neue Tickets: keine.
