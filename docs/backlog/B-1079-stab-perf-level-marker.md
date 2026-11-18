# B-1079 · Level-Generator: STAB- und PERF-Markierungen in k3c-level abbauen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU46
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c-level` (RU6) trägt zwei bewusste Schwachstellen: `STAB(B-1079)` in `level_entities.rs` (Go bricht bei leerer Liste der Verstecke mit `panic` ab, Rust meldet einen Fehler mit neuem Text) und `PERF(B-1079)` in `nature.rs` (`go_float` baut je Entität einen Text für den Variantenstrom; einmal je Level). Der Variantenstrom `<biom>:<seed>:variant:<gruppe>:<go_float(x)>` hängt an bitgleichen Entity-x (Review RU6.6).

## Ziel

Beide Markierungen sind nach Messung entfernt oder begründet unkritisch; die Bitgleichheits-Abhängigkeit der Varianten steht in `docs/standards/rust-port.md`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Erst messen (Allokationen je Level), dann ändern. Messung: temporärer Zähler im Test (Allokator `k3c-alloc-probe` in `rust/crates/k3c-alloc-probe`, falls für ein Test-Binary nutzbar) oder Zeitmessung `cargo test --release -p k3c-level nature -- --nocapture`; Ergebnis in die Session.
- `PERF(B-1079)` in `nature.rs`: Der Strom-Name wird bereits in einem wiederverwendeten Puffer (`stream`) gebaut; übrig ist die Allokation in `go_float(e.x)`. Entweder `go_float` auf einen `fmt::Write`-Zielpuffer umstellen (kein eigener `String` je Aufruf) oder die Marke durch eine Begründung ersetzen („einmal je Level, gemessen N Allokationen, unkritisch“) ohne Marke. Ergebnis der Float-Formatierung bleibt bitgleich (Variantenstrom).
- `STAB(B-1079)` in `level_entities.rs` (leerer Versteck-Pool): Die Rust-Meldung bleibt ein `Err` (kein Panic, Entscheidung 012: Rust darf robuster sein als Go); der Text wird festgelegt, mit Test auf einem Biom mit lauter `hub`/`edge`/geplanten Chunks, und die Marke entfällt, die Abweichung zu Go steht als normaler Kommentar (`// Abweichung zu Go (Entscheidung 012): …`).
- `docs/standards/rust-port.md`: ein Absatz in der Toleranz-Tabelle bzw. dem Abschnitt zu Gleitkomma: Variantenströme `<biom>:<seed>:variant:<gruppe>:<go_float(x)>` hängen an bitgleichen Entity-x (Toleranz 0), Rust darf den Strom-Text nicht anders formatieren.

## Nicht-Ziele

Neue Level-Regeln, Änderung der rng-Reihenfolge.

## Regeln und Einschränkungen

Entscheidung 012 (Port-Orakel), Leitfaden `docs/standards/rust-port.md` § 11. Keine Änderung der rng-Reihenfolge, keine neuen Level-Regeln; Golden (`testdata/golden/`, `testdata/rust/level/**`) werden nicht neu geschrieben. Läuft auf dem Second Worker (Lane zweit), Sim-Sperre gilt dort nicht.

- **Bereich:** `rust/crates/k3c-level/**`, `docs/standards/rust-port.md`
- **Abhängig von:** RU6
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Kein `STAB(B-1079)` und kein `PERF(B-1079)` mehr im Code (`grep -rn "B-1079" rust/crates/k3c-level` ohne Marke), `cargo clippy -p k3c-level --all-targets -- -D warnings` und `cargo test --release -p k3c-level` grün, Golden unverändert (`git status --short testdata` leer).
- **AC-02** Test für den leeren Versteck-Pool (Fehlertext festgelegt, kein Panic).
- **AC-03** `docs/standards/rust-port.md` nennt die Bitgleichheits-Abhängigkeit der Variantenströme.
- **AC-04** Der Level-Vergleich Go gegen Rust (`K3C_LEVEL_VERGLEICH`, siehe RU45.2) meldet nach der Änderung weiter `abweichend 0` für mindestens 1000 Seeds je Biom.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
