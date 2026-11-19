# Abnahme RU18: Wirtschaft, Bau, Monarch gegen Go-Golden (B-1101, RU51.3)

Stand: 2026-11-19, Second Worker, Branch `sprint/ru51` auf `origin/main` (25f46a4a).

| AC | Lauf | Ergebnis |
|---|---|---|
| AC-01 | `sim_test start` | nicht gefahren: von der Workbench abgelehnt (Sperre 🧑 2026-11-11, `K3C_SIM_ENTSPERRT` nicht gesetzt); nur 🧑 hebt auf |
| AC-02 | Balancing-Lauf | nicht gefahren, gleicher Grund (und RU18 ändert keine Werte in `data/`) |
| AC-03 | `check_run task:test` | 2005 grün, 2 rot, beide nicht RU18 (siehe `abnahme-ru21.md`) |
| AC-04 | Race | `go:race` grün, kein DATA RACE (siehe `abnahme-ru14.md`) |
| AC-05 | Golden-Abgleich Rust gegen `testdata/golden` | `cargo test --release -p k3c-sim`: 911 grün, 70 ignoriert (auf main), 1 rot (`eingebettete_welten_gueltig`, rom.json); Golden-Tests grün: `golden_kampagne`, `golden_kampagne_ausbau`, `groesste_fliesskomma_abweichung_je_golden_unter_der_toleranz` (Toleranz 1e-4), `alle_sim_golden_zustaende_laufen_im_kreis`, `golden_zusatzplatz_nur_ids` |

Je Teil (economy, building, hub, merchant, monarch, skills): kein Golden-Test rot, keine Abweichung zu Go beziffert. Die 70 ignorierten Tests von main sind auf `sprint/ru52` (noch nicht gemergt) zu 977 grün, 5 ignoriert abgebaut; ihre Golden-Erwartungen liefen dort grün, ohne dass ein Golden geändert wurde.
