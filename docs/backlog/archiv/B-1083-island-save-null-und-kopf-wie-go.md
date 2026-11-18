# B-1083 · Rust liest Insel-Spielstaende mit null-Feldern und falschem Kopf-Typ strenger als Go

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

Review von RU10.2 (niedrig): (1) `parse_island_save` liest `version`/`island` mit `as_i64().unwrap_or(0)`; Go bricht bei einem Nicht-Integer mit Unmarshal-Fehler ab (nur anderer Fehlertext). (2) Ein ausdrückliches `null` in String-, Vec- oder Zahl-Feldern von `IslandSave`/`IslandPlayerSave` bricht serde ab, Go lässt den Wert stehen. Betrifft nur von Hand bearbeitete Stände; Go schreibt dort nie `null`.

## Ziel

Rust liest solche Stände wie Go (null bleibt Standard, falscher Kopf-Typ ist ein Fehler).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Go-Referenz: `engine/sim/island_save.go` (`ParseIslandSave`, Zeile 90) und `island_save_v3.go` (`decodeIsland`, Zeile 21); Rust: `rust/crates/k3c-sim/src/island/save_parse.rs` (`parse_island_save`, `decode_island`).
- (1) `version` und `island` im Kopf: Ist der Wert vorhanden und kein Integer (z. B. String, Fliesskommazahl, Objekt), meldet Rust einen Fehler wie Go (`spielstand: json: cannot unmarshal …`; Wortlaut aus dem Go-Test `TestParseIslandSaveRejectsBadInput` ableiten oder, wenn Go-Text nicht reproduzierbar, den vorhandenen Stil von `fail(...)` nutzen und im Ergebnis nennen). Fehlt das Feld, bleibt 0 (wie heute und wie Go).
- (2) Ein ausdrückliches `null` in String-, Vec- oder Zahl-Feldern von `IslandSave` und `IslandPlayerSave` (Typen in `save.rs`) lässt den Standardwert stehen (Go: `null` ist No-Op beim Unmarshal). Umsetzung z. B. mit `#[serde(default, deserialize_with = "null_default")]` oder einer Vorverarbeitung in `decode_island`, die `null`-Felder (außer den Pflichtfeldern `campaignId`, `seed`, `time`, `stock`, `stages`, `players`, die bleiben Fehler `Feld X fehlt`) aus dem `Value` entfernt.
- Verhalten sonst unverändert, deterministisch, 2+ Spieler (die Spielerliste bleibt wie gelesen).

## Nicht-Ziele

Neue Spielstand-Versionen, Änderung des Schreibformats (Go schreibt nie `null`), Migrationslogik, Go-Code ändern.

## Regeln und Einschränkungen

Spielstand-Format bleibt (Sperre `spielstand` gilt nur für Formatänderungen, hier keine). Keine Rücksicht auf alte Spielstände jenseits der Go-Verhalten-Gleichheit. Leitfaden `docs/standards/rust-port.md` § 6, § 11. Datei <= 400 Zeilen (`save_parse.rs` hat 265).

- **Bereich:** `rust/crates/k3c-sim/src/island/save_parse.rs`, `rust/crates/k3c-sim/src/island/save.rs`, `rust/crates/k3c-sim/src/island/save_tests.rs`, `rust/crates/k3c-sim/src/island/save_null_tests.rs`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`{"version":"9"}` -> Fehler. `"seed":null` -> `Feld seed fehlt`. `"worldId":null` oder `"skillPool":null` -> Standardwert, kein Fehler.

## Ausnahme- und Fehlerfälle

Pflichtfelder mit `null` bleiben Fehler (`Feld X fehlt`).

## Akzeptanzkriterien

- **AC-01** Test: `version` und `island` mit Nicht-Integer-Wert (String, 1.5, Objekt) ergeben einen Fehler; fehlend ergibt 0.
- **AC-02** Test: `null` in je einem String-, Vec- und Zahl-Feld von `IslandSave` und `IslandPlayerSave` wird wie ein fehlendes Feld gelesen (Standardwert), Pflichtfelder mit `null` bleiben Fehler.
- **AC-03** `cargo test --release -p k3c-sim island::save` grün (alle bisherigen Save-Tests unverändert), `cargo clippy -p k3c-sim --all-targets -- -D warnings` ohne Warnungen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
