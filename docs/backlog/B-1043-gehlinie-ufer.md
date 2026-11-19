# B-1043 · Gehlinie liegt bei 58 % der Ansicht, das Ufer ist 7 % hoch

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU55
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 1: In K2C liegt die Gehlinie bei 57–59 % der Ansicht, das Ufer ist ≈ 7 %, der Fluss ≈ 35 %. Bei uns liegt die Gehlinie bei 66,7 % (`view.WorldShare`), das Ufer bei ≈ 3 % (`world.bankUnits` 20 px), der Fluss bei ≈ 30 %. Das Spiel wird nach Rust portiert (Go-Stopp).

## Ziel

Das Bild hat die drei Bänder Welt, Ufer, Fluss im Verhältnis 58 : 7 : 35, in jeder Ansicht und jeder Split-Screen-Hälfte.

## Beteiligte und Zielgruppen

Spieler am TV, 1 oder 2 Ansichten. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-hud`. Abnahme: 🧑.

## Anforderungen

- Gehlinie bei 58 % der Zellenhöhe, ein fester Wert für alle Biome (B-526 bleibt gültig); heute `WORLD_SHARE = 2/3` in `rust/crates/k3c-game/src/view/cell.rs`.
- Ufer 7 % der Zellenhöhe (heute `BANK_UNITS` 20 px in `world/ground.rs`), Material und Farbe je Biom bzw. Welt (Erde mit Rissen, Steinkai), Grasbüschel über der Kante.
- HUD, Seekarte und Marken rücken mit; nichts liegt im Fluss außer Spiegelung und Glanz.
- **Werte aus `data/look.json` (neu):** Abschnitt `profiles` mit zwei Profilen. `k2c` (Standard des Spiels): `worldShare` 0,58, `bankShare` 0,07, übrige Werte der Folge-Tickets (Weite, Tönung). `go` (alt: 2/3, 20 px, 420/32 Units): nur für die Bildvergleiche gegen das Go-Spiel (RU32, Gate G4), damit die Abnahme am PC nicht an der neuen Optik scheitert. Laden über `rust/crates/k3c-data/src/look.rs` (neue Datei, ein `pub mod look;` in `lib.rs`); das Profil wählt `K3C_LOOK` (`k2c` | `go`), Standard `k2c`; Tests und `tests/e2e_*.rs` setzen `go` ausdrücklich.
- Alle Stellen, die `WORLD_SHARE` oder Ufer-Höhe kennen (`audio/mix.rs`, `game/layout.rs`, `replay/layout.rs`, `still/state.rs`, `props/decor.rs`), lesen den Wert aus dem Profil statt aus einer eigenen Konstante.

## Nicht-Ziele

Spiegelung (B-1044), Maßstab der Welt (B-1045).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/view/cell.rs`, `rust/crates/k3c-game/src/world/ground.rs`, `rust/crates/k3c-game/src/world/river.rs`, `rust/crates/k3c-game/src/hud/layout.rs`, `rust/crates/k3c-game/src/game/layout.rs`, `rust/crates/k3c-game/src/replay/layout.rs`, `rust/crates/k3c-game/src/still/state.rs`, `rust/crates/k3c-game/src/audio/mix.rs`, `rust/crates/k3c-game/src/props/decor.rs`, `rust/crates/k3c-data/src/look.rs`, `rust/crates/k3c-data/src/lib.rs`, `data/look.json`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
- Kein Änderungsbedarf in Engine und Sim; das Spiel zeichnet nur. Golden und `testdata/` bleiben unverändert (Profil `go` hält den alten Stand). RU42-Leitplanken: kein `unwrap`/`expect`/`panic` im Produktivcode, die Allokations-Obergrenze in `tests/alloc_je_bild.rs` wird nicht gelockert.

## Beispiele

Fenster 1920 × 1080, eine Ansicht: Gehlinie bei y ≈ 626, Ufer bis ≈ 702, darunter Fluss.

## Ausnahme- und Fehlerfälle

Sehr flache Fenster: Ufer höchstens ein Drittel des Flusses.

## Akzeptanzkriterien

- **AC-01** Test im Rust-Client: Gehlinie bei 58 % ± 1 px, Ufer 7 % in einer und zwei Ansichten.
- **AC-02** Am TV (🧑): HUD und Marken überdecken nichts Wichtiges.
- **AC-03** Prüfläufe des Rust-Clients grün.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung`.
