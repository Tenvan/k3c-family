# B-1293 · Vier k3c-sim-Tests sind auf main rot (eingebettete Welten, Takt wie Go, Geschichte)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU41
- **Projekt:** RSU
- **Erstellt:** 2026-11-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-25 (Takt-Test: Test-Welt aus testdata/, Default bestätigt)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`cargo test --workspace` war auf main rot in `k3c-sim` (lokal 2026-11-25, unabhängig von RU41). Seit WL2 (#837, #821, B-1215) liegen in `data/worlds/` auch britannien, nil, sengoku, und standard.json hat fünf Inseln.
- `island::worlds_tests::eingebettete_welten_gueltig`: erwartete nur `standard.json`.
- `island::worlds_tests::standard_welt_gleich_bisherigen_inseln`: Hash der `islands` bewusst geändert.
- `island::takt_tests::insel_nach_1000_schritten_wie_go` und `story::mod_tests::standard_welt_erzaehlt_eine_geschichte`: vergleichen mit Go-Fixture bzw. Erwartungen aus der alten Standard-Welt.

Umnummeriert von B-1286 (Nummer doppelt vergeben, auf main ist B-1286 die Abnahme WZG).

## Ziel

`cargo test --workspace` ist wieder grün, ohne die Absicht der Tests aufzugeben.

## Beteiligte und Zielgruppen

Agent setzt um; 🧑 hat entschieden: Takt-Test mit fester Test-Welt aus `testdata/` (2026-11-25).

## Anforderungen

- Welten-Tests auf alle Dateien in `data/worlds/` und den neuen Stand der Standard-Welt umstellen (Hash bewusst neu, Grund im Commit).
- Takt- und Geschichten-Test: feste Test-Welt aus `testdata/` statt der Standard-Welt, Go-Fixture bleibt gültig.

## Nicht-Ziele

Golden unter `testdata/golden` neu schreiben; Regel- oder Wertänderungen.

## Regeln und Einschränkungen

Golden werden nie zugunsten von Rust neu geschrieben (Entscheidung 012).

- **Bereich:** `rust/crates/k3c-sim/src/island/**`, `rust/crates/k3c-sim/src/story/**`, `testdata/rust/worlds/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`cargo test -p k3c-sim --lib` → 0 failed.

## Ausnahme- und Fehlerfälle

Weicht Rust bei gleicher Welt von der Go-Fixture ab: Fehler in Rust beheben, Fixture bleibt.

## Akzeptanzkriterien

- **AC-01** `cargo test -p k3c-sim --lib` grün.
- **AC-02** `cargo test --workspace` grün (lokal mit `task gen:paper`).

## Offene Fragen

keine (🧑 2026-11-25: Test-Welt aus testdata/)

## Notizen

Gefunden in RU41.1, umgesetzt in RU41.4 (1197 passed, 0 failed); `cargo test --workspace` grün in RU41.3.
