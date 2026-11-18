# B-1073 · Fluss-Shader in world anbinden (draw_river, ready, Rückfall auf Kacheln)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`world/water.rs` (`RiverWater::ready`, `build`, `set`) rechnet die Uniforms, `ready` liefert aber immer `false`; `draw_river` zeichnet darum immer Kacheln. `tests/still.rs` vergleicht den Fluss nur als Kacheln (Kommentar „der Shader gehört nicht zum Vergleich, B-1055“). Teil 4 von 4 aus B-1055.

## Ziel

`draw_river` nutzt in der Fassung „shader“ den Canvas-Aufruf `river` (B-1070) und fällt auf Kacheln zurück, wenn der Canvas `false` meldet; der Standbild-Vergleich deckt den Fluss mit Shader ab.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `RiverWater` hält die vier Farben aus Go (`rgb`) und baut daraus `RiverUniforms`; `ready` ist `true`, sobald die Fassung „shader“ gewählt ist; der Rückfall entscheidet das Ergebnis von `Canvas::river`.
- `draw_river` ruft bei Shader-Fassung `river(rect, &uniforms)` mit dem Rechteck aus `water_layout` (`x`, `top`, `w`, `h`); bei `false` die Kachel-Fassung.
- Die Logzeile „Fluss-Fassung“ meldet „aktiv“ nach tatsächlicher Wahl; Tests in `water_tests.rs` (`river_shader_lazy` u. a.) werden angepasst.
- `tests/still.rs` zeichnet den Fluss mit Shader und vergleicht gegen Go-Golden in den Schwellen von RU13.6; wo Go-Golden fehlen, nur gegen die Software-Referenz.

## Nicht-Ziele

Neuer Go-Golden-Lauf oder `gen:paper` (nur im Abnahme-Ticket B-1059), Sichtprüfung auf dem TV (Ralf).

## Regeln und Einschränkungen

`docs/standards/rust-port.md`; `rust/crates/k3c-game/src/replay/**` bleibt unberührt. Kein Sim, kein Balancing.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/world/water.rs`, `rust/crates/k3c-game/src/world/river.rs`, `rust/crates/k3c-game/src/world/water_tests.rs`, `rust/crates/k3c-game/src/world/mod.rs`, `rust/crates/k3c-game/tests/still.rs`
- **Abhängig von:** B-1070, B-1071, B-1072
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Einstellung `shader` mit GPU-Canvas: Fluss per Shader. Einstellung `shader` mit Canvas, der `false` liefert: Kacheln. Einstellung `tiles`: immer Kacheln.

## Ausnahme- und Fehlerfälle

Zelle ohne sichtbare Fläche (`h` höchstens 0): nichts zeichnen. Canvas lehnt ab: Kacheln, kein Fehler.

## Akzeptanzkriterien

- **AC-01** In Shader-Fassung ruft `draw_river` `Canvas::river` genau einmal je Zelle mit den Werten aus `water_layout` (Test mit Aufzeichnungs-Canvas in `water_tests.rs`).
- **AC-02** Meldet der Canvas `false`, entstehen dieselben Kachel-Befehle wie in der Kachel-Fassung (Test).
- **AC-03** `tests/still.rs` besteht mit Shader-Fluss in den Schwellen von RU13.6; `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` sind grün.

## Offene Fragen

keine

## Notizen

Aus B-1055 zerlegt. Sichtvergleich Shader auf GPU: Ralf im Spiel, Teil von B-1059.
