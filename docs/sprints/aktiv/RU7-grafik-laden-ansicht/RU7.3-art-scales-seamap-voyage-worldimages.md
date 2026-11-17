# RU7.3 · art Teil 3: scales, seamap, voyage, worldimages

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** RU7.2
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

Die Bild-Zuordnungen aus `data/paper.json` und `data/native.json` (Skalen, Seekarte, Reise, Weltbilder) laden in Rust wie in Go.

## Kontext

Quellen: `native/k3c-game/art/scales.go` (36), `seamap.go` (108, T), `voyage.go` (74, T), `worldimages.go` (89, T). Go liest über `data.Files`; Rust liest über `art/datafile.rs` (neu: `read(name) -> Result<Vec<u8>, Fehler>`, Ordner aus `K3C_DATA_DIR`, sonst `data/` relativ zum Arbeitsordner, wie `data/embed.go` ohne Einbetten). Nicht `k3c-data` benutzen (RU3 läuft parallel). Tests lesen das echte `data/` ab `CARGO_MANIFEST_DIR` (`../../../data`).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`

## Nicht-Ziele

Typisierte Daten in `k3c-data`. Zeichnen der Seekarte (HUD, späterer Sprint).

## Schritte

1. Go-Dateien und Tests ganz lesen.
2. `art/datafile.rs`, dann `scales.rs`, `seamap.rs`, `voyage.rs` (Go `sync.Once` → `std::sync::OnceLock`), `worldimages.rs`.
3. Tests `seamap_tests.rs`, `voyage_tests.rs`, `worldimages_tests.rs` portieren.

## Fertig, wenn

- [x] AC-01: `cargo test -p k3c-game art` grün, inklusive der drei portierten Testdateien gegen das echte `data/`.
- [x] `task rust:lint` grün.

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

- **AC-01 geprüft:** `cargo test -p k3c-game art` grün (36 Tests, darunter portierte `seamap`, `voyage`, `worldimages` gegen das echte `data/` über `CARGO_MANIFEST_DIR/../../../data`, plus `data_scales`-Parser und `Library::usable`). `task rust:lint` und `task check:diff` grün; Demo `--headless --steps 600` Code 0.
- **Umgesetzt:** `art/datafile.rs` (`K3C_DATA_DIR`, sonst `data/`), `seamap.rs`, `voyage.rs` (`OnceLock`), `worldimages.rs` (`STANDARD_WORLD`), `scales::data_scales`/`parse_scales`; `default_config()` setzt `scales` jetzt aus `data/paper.json`.
- **Abweichungen:** `apply_scales` stand schon seit RU7.2 (vorgezogen). `Library::usable` (Go `worldimages.go`) ergänzt. Go-`nil`-Zuordnung (`*SeaMap == nil`) entfällt: `SeaMap::from_data` liefert `Option`; Go-Test `AllocsPerRun` entfällt (kein Allokationszähler in Rust, die Zugriffe liefern `&str`). `voyage()` liest `data/` relativ zum Arbeitsordner; Tests lesen daher das echte `data/` direkt.
- Neue Tickets: keine.
