# B-1066 · Rust lädt die Seekarte aus data/paper.json relativ zum Arbeitsordner

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU20
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`Hud::new` ruft `SeaMap::from_data()` (`rust/crates/k3c-game/src/art/seamap.rs`), das über `art/datafile.rs::read` die Datei `data/paper.json` relativ zum Arbeitsordner liest (Meldung „❌ Assets der Seekarte nicht lesbar … data\paper.json“ in jedem Test aus `rust/crates/k3c-game`). Go nimmt die eingebetteten Daten (`art.DataSeaMap`, `data.Files`). Dasselbe gilt für `scales.rs`, `voyage.rs` (native.json) und `worldimages.rs`, die ebenfalls `datafile::read` nutzen.

## Ziel

`datafile::read` wählt die Datenquelle wie Go (`data.Files`): `K3C_DATA_DIR`, sonst `data/` neben der EXE, sonst eingebettet (`k3c_data::Source::from_env`). Die Seekarte und die übrigen Zuordnungen der Grafik laden dann unabhängig vom Arbeitsordner, auch ohne Ordner `data/`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `datafile::read` nutzt `k3c_data::Source::from_env()`; `read_in` (ausdrücklicher Ordner) bleibt.
- Keine Verhaltensänderung bei gesetztem `K3C_DATA_DIR` oder vorhandenem Ordner neben der EXE.
- Fehlertext nennt wie bisher den Pfad bzw. `data/<name>`.

## Nicht-Ziele

Neue Dateien in die Einbettung von k3c-data aufnehmen (Go bettet genau `*.json biomes/*.json worlds/standard.json`); Seekarte selbst (RU23).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung (B-1014); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; keine neue Abhängigkeit (k3c-data ist schon Abhängigkeit von k3c-game).

- **Bereich:** `rust/crates/k3c-game/src/art/datafile.rs`, `rust/crates/k3c-game/src/art/seamap_tests.rs`
- **Abhängig von:** RU20
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Test läuft im Ordner `rust/crates/k3c-game` (ohne `data/`) → `SeaMap::from_data()` liefert `Some`, keine Fehlermeldung im Log.

## Ausnahme- und Fehlerfälle

Datei nicht in der Einbettung (z. B. `worlds/x.json`) und kein Ordner → Fehler `data/<name>: open <name>: file does not exist` wie in Go.

## Akzeptanzkriterien

- **AC-01** `datafile::read("paper.json")` liefert die Daten auch bei Arbeitsordner ohne `data/` (Test in `art/`).
- **AC-02** `SeaMap::from_data()` ist im Testlauf `Some`; `cargo test -p k3c-game` und `task rust:lint` grün.

## Offene Fragen

keine

## Notizen

Ergebnis (2026-11-18, Claude): `art/datafile.rs::read` nutzt `k3c_data::Source::from_env()` (K3C_DATA_DIR, `data/` neben der EXE, sonst eingebettet); `read_in` ist nur noch für Tests da, neu `read_from`. Test `sea_map_aus_eingebetteten_daten` (Arbeitsordner ohne `data/`). `cargo test -p k3c-game` (390 + Integrationstests) und `task rust:lint` grün; die Meldung „Assets der Seekarte nicht lesbar“ erscheint im Testlauf nicht mehr.
