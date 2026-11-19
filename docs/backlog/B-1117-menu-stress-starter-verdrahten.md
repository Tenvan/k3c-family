# B-1117 · Menü bekommt den Stress-Starter des Spiels (MenuSet und Verdrahtung)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU63
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU30.5 liefert `Game::stress_starter()` (`game/stress.rs`, `StarterLink`: erfüllt `view::stress::StressStarter`, die Schleife führt den Auftrag gleich nach dem `update` des Screens aus). Das Menü hält den Starter in `Menus::starter` (`set_stress_starter`), aber `menu::MenuSet` (Feld `shared` privat) bietet nach `menu::new_set` keinen Weg dorthin; `game/packages.rs` baut das Menü vor dem `Game`. Ohne Starter zeigt das Testing-Menü `TstNoRunner`, der Stresstest ist im Rust-Spiel nicht aus dem Menü startbar.

## Ziel

Der Starter des Spiels kommt ins Menü: `MenuSet::set_stress_starter` (menu), Weitergabe über die Rolle `Menus`/`Shared` und Aufruf nach `Game::new` (packages bzw. Client-Start), sodass das Testing-Menü den Stresstest startet.

## Beteiligte und Zielgruppen

Ralf spielt und prüft am PC; Entwickler.

## Anforderungen

Der Stresstest startet im Rust-Spiel aus dem Testing-Menü; keine Allokation je Bild.

## Nicht-Ziele

Cheat-Fenster und Entfernen der Cheat-Menüs (RU56, B-1136).

## Regeln und Einschränkungen

Rust-Leitplanken RU42; Dateien <= 400 Zeilen.

Planungsangaben:

- **Bereich:** `rust/crates/k3c-game/src/menu/mod.rs`, `rust/crates/k3c-game/src/menu/testing_tests.rs`, `rust/crates/k3c-game/src/game/packages.rs`, `rust/crates/k3c-game/src/game/stress.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Testing-Menü, Stresstest wählen: der Lauf startet statt der Meldung `TstNoRunner`.

## Ausnahme- und Fehlerfälle

Ohne Starter (Menü allein in Tests) bleibt `TstNoRunner` wie in Go.

## Akzeptanzkriterien

- **AC-01** `menu::MenuSet` bietet `set_stress_starter(Box<dyn StressStarter>)`; ein Test belegt, dass das Testing-Menü danach startet statt `TstNoRunner` zu zeigen.
- **AC-02** Nach `Game::new` setzt die Verdrahtung `game.stress_starter()` im Menü (Test mit `packages()` und `FakeSim` samt Dev-Rolle).
- **AC-03** `cargo clippy` der Crate grün, Dateien <= 400 Zeilen.
- **AC-04** Ralf startet den Stresstest aus dem Testing-Menü im Fenster (manuelle Abnahme).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
