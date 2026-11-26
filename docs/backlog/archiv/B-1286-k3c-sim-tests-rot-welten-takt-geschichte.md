# B-1286 · Vier k3c-sim-Tests sind auf main rot (eingebettete Welten, Takt wie Go, Geschichte)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU41
- **Projekt:** RSU
- **Erstellt:** 2026-11-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-25 (Takt-Test: Test-Welt aus testdata/, Default bestätigt)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`cargo test --workspace` ist auf main rot in `k3c-sim` (lokal 2026-11-25, unabhängig von RU41: die Branch ändert unter rust/, data/, testdata/ nur zwei Test-Hilfen). Die CI auf main ist im Rust-Job schon vorher rot (`plan_objects_stage_tests`, fehlende Grafik).
- `island::worlds_tests::eingebettete_welten_gueltig`: erwartet nur `standard.json` in `data/worlds/`, seit WL2 (#837) liegen dort auch britannien, nil, sengoku.
- `island::worlds_tests::standard_welt_gleich_bisherigen_inseln`: Hash der `islands` von standard.json, seit WL2 (#821, B-1215, fünf Inseln) bewusst geändert.
- `island::takt_tests::insel_nach_1000_schritten_wie_go` und `story::mod_tests::standard_welt_erzaehlt_eine_geschichte`: vergleichen mit Go-Fixtures bzw. Erwartungen aus der alten Standard-Welt.

## Ziel

`cargo test --workspace` ist wieder grün, ohne die Absicht der Tests aufzugeben.

## Beteiligte und Zielgruppen

Agent setzt um; 🧑 entscheidet, ob die Go-Fixture des Takt-Tests durch eine Rust-Referenz ersetzt wird (Go-Engine seit RU41 entfernt).

## Anforderungen

- Welten-Tests auf alle Dateien in `data/worlds/` und den neuen Stand der Standard-Welt umstellen (Hash bewusst neu, Grund im Commit).
- Takt- und Geschichten-Test: Ursache belegen (Datenänderung WL2 oder Abweichung von Rust). Bei Datenänderung eine feste Test-Welt aus `testdata/` statt der Standard-Welt nehmen, damit die Go-Fixture gültig bleibt.

## Nicht-Ziele

Golden unter `testdata/golden` neu schreiben; Regel- oder Wertänderungen.

## Regeln und Einschränkungen

Golden werden nie zugunsten von Rust neu geschrieben (Entscheidung 012: Orakel ist das Verhalten).

- **Bereich:** `rust/crates/k3c-sim/src/island/**`, `rust/crates/k3c-sim/src/story/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
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

Takt-Test: Test-Welt aus testdata/ statt Standard-Welt (Default) oder neue Rust-Referenz (🧑)?

## Notizen

Gefunden in RU41.1.
