# Rust-Sim-Abgleich: Golden mit gleichem Verhalten (Gate G3, Sprint RU22)

Stand 2026-11-18, Branch `sprint/ru22`. Maßstab ist Entscheidung 012: dieselben Ereignisse in derselben Reihenfolge, dieselben
Ganzzahl-Werte, Fließkommawerte innerhalb 1e-4 relativ (absolut 1e-9 nahe 0). Zweck ist nur, Port-Fehler zu finden. Kein
Golden wurde geschrieben oder geändert.

## Die 15 Golden

Vergleich: `cargo test -p k3c-sim --test golden_forest --test golden_biome --test golden_campaign` (Debug-Bau, zusammen unter
2 s). Werkzeug: `k3c_det::golden::diff_tol` (neu, `SIM_TOL`), `tests/common/` (Lauf, Meldung mit erstem Snapshot und Pfad,
Leser der Go-Spur `engine/sim/golden_trace_test.go`).

| Datei | Takte | Spieler | Snapshots | gleich | größte Fließkomma-Abweichung (relativ) | Ticket |
|---|---|---|---|---|---|---|
| sim-forest-ohne-spieler | 1800 | 0 | 61 | ja | 0 (alle 25010 Werte bitgleich) | – |
| sim-forest-tag | 1800 | 2 | 61 | ja | 0 (26356) | – |
| sim-forest-nacht | 2700 | 2 | 91 | ja | 0 (49582) | – |
| sim-forest-aufbau | 6000 | 2 | 101 | ja | 0 (60137) | – |
| sim-forest-raub | 1800 | 2 | 31 | ja | 0 (26578) | – |
| sim-forest-sturm | 4500 | 2 | 76 | ja | 0 (77962) | – |
| sim-forest-monarch | 2430 | 2 | 41 | ja | 0 (25320) | – |
| sim-cave-aggression | 2700 | 1 | 91 | ja | 0 (33841) | – |
| sim-cave-aufbau | 6000 | 2 | 101 | ja | 0 (83570) | – |
| sim-cave-belagerung | 6000 | 1 | 101 | ja | 0 (62855) | – |
| sim-mine-welle | 3000 | 2 | 51 | ja | 0 (33238) | – |
| sim-crystal-welle | 1500 | 2 | 16 | ja | 0 (8270) | – |
| sim-ironhold-welle | 1500 | 2 | 16 | ja | 0 (9978) | – |
| campaign-abstieg | 3319 | 2 | 111 | ja (Tiefe 2, Spielstand, Laden, Reise) | innerhalb Toleranz | – |
| campaign-ausbau | 7809 | 2 | 27 | ja (Tiefe 0, Inhaltsprüfung) | innerhalb Toleranz | – |

Die Abweichung der Sim-Golden misst `cargo test -p k3c-sim --test golden_abweichung -- --nocapture` (jeder Fließkommawert
jedes Snapshots; die Rust-Sim rechnet in den 13 Läufen bitgleich zu den Go-Golden, die Toleranz wird nie gebraucht). Die
Kampagnen-Golden bestehen mit der Toleranz; ihre Abweichung wird nicht eigens gezählt.

Zusätzlich läuft `tests/golden_zusatzplatz.rs` (aus dem ignorierten Test `golden_zusatzplatz_nur_ids`, B-605/AC-02): Alle 13
Sim-Läufe mit und ohne zusätzlichen Hub-Platz unterscheiden sich nur in IDs und dem neuen Platz.

## Behobene Abweichungen

Gegen die Golden gab es keine Abweichung: Alle 15 Läufe waren schon beim ersten Versuch gleich. Beim Entfernen der
Platzhalter (`kein_platzhalter`) fielen drei Lücken auf, die kein Golden berührt (Inseln fehlen in den Golden) und die in
einer Insel abgestürzt wären (`unimplemented!`):

| Ort | Ursache | Behebung |
|---|---|---|
| `boss::boss_defeated_with` (Boss-Belohnung) | rief den Platzhalter `add_stock_capped` statt `island::storage::add_stock_capped` | ruft die echte Funktion, Go `ok && taken > 0` |
| `island::travel` (Banner beim Reisen) | rief den Platzhalter `return_banner`; die echte Funktion in `combat/squad.rs` war privat | `return_banner` ist `pub`, Platzhalter entfernt |
| `state::common::hit_site` (geschützte Nacht) | rief den Platzhalter `protected_night(w)` ohne Insel | `apply_damage_by` und die Aufrufer `hit_splash`, `step_projectiles`, `step_hazards` bekommen `Option<&IslandRef>` und rufen `island_ref::protected_night(w, isl)` |

Dazu: Die Import-Stellen `pending.rs` (vier Dateien, fast nur `pub use`) heißen jetzt `links.rs`; es stehen weder
`unimplemented!`/`todo!` noch `panic!("RU…")` im Quelltext außerhalb der Testdateien (`tests/kein_platzhalter.rs`). Offen
bleiben die Testhelfer von B-1103 in `*_tests.rs` und `testutil.rs` (70 `#[ignore]`, `unimplemented!` in Testdateien).

## Durchsatz

Gemessen am 2026-11-19 (RU51.5). Eine Messung braucht einen ruhigen Rechner im Release-Bau; auf dem Hauptrechner arbeitet Ralf, die
Sim-Sperre gilt. Messwerkzeug steht bereit und läuft im Abnahme-Ticket (`Lane: zweit`, der Second Worker fährt sie):

- Rust: `cargo test --release -p k3c-sim --test golden_durchsatz -- --ignored --nocapture` (5 Messungen à 10 Läufe von
  `sim-forest-aufbau`, 6000 Takte, nur `step`).
- Go: `go test ./engine/sim/ -run TestGoldenDurchsatz -durchsatz -count 5 -v` (derselbe Lauf, `testing.Benchmark`).

| Messgröße | Rust | Go | Rechner | Datum |
|---|---|---|---|---|
| Schritte je Sekunde, Median (Spanne) | 243 304 (240 368 bis 245 864; 5 Messungen) | 476 884 (470 778 bis 482 759; 5 Läufe von `-count 5`) | AMD Ryzen 7 5800X (8 Kerne), 32 GB RAM, Windows 11, Release-Bau | 2026-11-19 |

Rust erreicht damit etwa 51 % des Go-Durchsatzes am selben Lauf (`sim-forest-aufbau`, 6000 Takte, 2 Spieler). Nur berichtet,
keine Schwelle bewertet; Optimierung gehört zu RU43. Die Messgrößen sind nicht streng gleich (Rust: 5 Messungen à 10 Läufe;
Go: `testing.Benchmark` je Zählung, 96 bis 98 Läufe), die Größenordnung ist trotzdem belastbar (Streuung unter 3 %).

## Empfehlung für G3

Die Sim ist abgeglichen: alle 15 Golden gleich (13 Sim-Läufe sogar bitgleich), keine Abweichung, kein `#[ignore]` in den
Golden-Tests, kein Platzhalter im Spielpfad. Empfehlung: **bestanden**; offen sind nur der Durchsatz (Abnahme-Ticket) und die
70 ignorierten Testhelfer-Tests aus B-1103, die keine Golden betreffen. Die Go-Spur (`TestGoldenTrace`) ist für den Fall
bereit, dass spätere Änderungen ein Golden brechen.

Entscheidung G3 (🧑): offen
