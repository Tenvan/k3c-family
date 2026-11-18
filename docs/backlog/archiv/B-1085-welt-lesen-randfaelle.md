# B-1085 · Welt-Datei lesen: null- und Array-Randfälle weichen von Go ab

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU46
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beim strengen Lesen der Welt-Datei (`island/worlds.rs`, `worlds_decode.rs`) weichen Randfälle von Go ab (Review RU10.5): (1) `"skins": null` wird angenommen, Go lehnt ab (`WorldFile.skins` ist ein `Value`, fehlt und null sind nicht unterscheidbar; `k3c-data` liegt außerhalb des Bereichs); (2) `T::deserialize(&Value)` nimmt Structs auch aus JSON-Arrays an (Insel als Array, `depthScaling: [1,1,1]`, `nature: []`), Go meldet `kein gültiges JSON`; (3) `natureByDepth: {"0": null}` lehnt Rust ab, Go nimmt leere Werte; eine Insel `null` meldet Rust als JSON-Fehler statt `id fehlt`.

## Ziel

Die Randfälle verhalten sich wie in Go.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Go-Referenz: `engine/sim/island_worlds.go` (`decodeIslands` Zeile 186, Welt-Kopf) und `engine/sim/island_nature.go`; Rust: `rust/crates/k3c-sim/src/island/worlds.rs`, `worlds_decode.rs`, Kopf `rust/crates/k3c-data/src/world.rs` (`skins: serde_json::Value`).
- Fall 1: `"skins": null` lehnt Rust wie Go ab (Fehlertext wie Go: Pflichtfeld fehlt bzw. null nicht erlaubt; Wortlaut aus dem Go-Test `engine/sim/island_worlds_test.go` ableiten). Fehlt das Feld, bleibt der bisherige Fehler.
- Fall 2: Eine Insel, `depthScaling` oder `nature` als JSON-Array statt Objekt ist ein Fehler `kein gültiges JSON` (vor dem Lesen den Typ prüfen, z. B. `Value::is_object`, statt `T::deserialize(&Value)` Arrays als Struct anzunehmen).
- Fall 3: `natureByDepth: {"0": null}` wird wie in Go als leerer Wert angenommen; eine Insel `null` in `islands` meldet `id fehlt` statt eines JSON-Fehlers.
- Reihenfolge und Text aller übrigen Fehlermeldungen bleiben (bestehende Tests in `worlds_error_tests.rs`, `worlds_tests.rs` müssen unverändert grün bleiben).

## Nicht-Ziele

Neue Welt-Felder, Änderungen an `data/worlds/*.json`, Go-Code ändern (Go ist das Orakel, Entscheidung 012), `k3c-data`-Umbauten außer dem Feld `skins` in `world.rs`.

## Regeln und Einschränkungen

Port-Leitfaden `docs/standards/rust-port.md` (§ 6 Fehler, § 11). Golden nicht neu schreiben. Datei <= 400 Zeilen, Funktion <= 60 Zeilen: `worlds_decode.rs` hat 250 Zeilen, neue Prüfungen bei Bedarf in eine neue Datei `worlds_shape.rs` auslagern.

- **Bereich:** `rust/crates/k3c-sim/src/island/worlds.rs`, `rust/crates/k3c-sim/src/island/worlds_decode.rs`, `rust/crates/k3c-sim/src/island/worlds_shape.rs`, `rust/crates/k3c-sim/src/island/worlds_tests.rs`, `rust/crates/k3c-sim/src/island/worlds_error_tests.rs`, `rust/crates/k3c-data/src/world.rs`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`"skins": null` -> Fehler wie Go. Insel `[]` -> `kein gültiges JSON`. `"natureByDepth": {"0": null}` -> geladen, Wert leer.

## Ausnahme- und Fehlerfälle

Unbekannte Felder und fehlende Pflichtfelder bleiben Fehler wie bisher.

## Akzeptanzkriterien

- **AC-01** Die genannten Fälle (skins null, Insel/depthScaling/nature als Array, natureByDepth-null-Wert, Insel null) verhalten sich wie in Go: je ein Test in `worlds_tests.rs` bzw. `worlds_error_tests.rs`; Reihenfolge und Text der übrigen Meldungen bleiben.
- **AC-02** `cargo test --release -p k3c-sim island::worlds` und `cargo test --release -p k3c-data` grün; `cargo clippy -p k3c-sim -p k3c-data --all-targets -- -D warnings` ohne Warnungen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
