# B-1078 · PERF: draw allokiert bis zu 29 mal je Bild, Ziel 0

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU44
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c-game/tests/alloc_je_bild.rs` (RU42, B-1075) misst im Maximum 29 Allokationen in `draw` je Bild. Fundstellen vermutet: `Scene::build` einmalig (nach Aufwärmen nicht), je Bild `Rc`-Klone in `k3c-rig` `Actor::draw_mount` (`mount.clone()`, `skin.clone()`) und kurzlebige `Vec` in den Zeichnern.

## Ziel

`draw` allokiert nach dem Aufwärmen nicht mehr.

## Beteiligte und Zielgruppen

Agenten setzen um; Messung gilt (Ralf 2026-11-18).

## Anforderungen

- Puffer in den Zeichnern wiederverwenden, `Rc`-Klone im Bildpfad vermeiden.
- Obergrenze im Test absenken.

## Nicht-Ziele

Optimierung des Takts (B-1077).

## Regeln und Einschränkungen

Rust-Standard § 11 (`docs/standards/rust-port.md`): kein Allokieren in `draw`, kein `format!`/`to_string()`/`clone()` von `String`/`Vec` je Bild. Nicht angefasst werden `src/menu/**`, `src/hud/**`, `src/devtools/**` (RU9, RU23, RU29) sowie `k3c-sim` und `k3c-level`. Gemessen wird nur mit `k3c-game/tests/alloc_je_bild.rs` (gerenderte Grafik nötig, `K3C_GRAFIK_DIR` oder Test übersprungen).

- **Bereich:** `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/replay/**`, `rust/crates/k3c-rig/src/**`, `rust/crates/k3c-game/tests/alloc_je_bild.rs`
- **Abhängig von:** B-1077
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Messaufgabe.

## Ausnahme- und Fehlerfälle

Nicht vermeidbare Allokation: als unkritisch begründen.

## Akzeptanzkriterien

- **AC-01** Die Zeichner in `world`, `props`, `actors`, `fx`, `view` und `replay` (z. B. `notes.clone()` in `replay/app.rs`) halten ihre `Vec`-Puffer und rufen `clear()` statt neue anzulegen.
- **AC-02** `k3c-rig` `Actor::draw_mount` (`paint/actor.rs`) klont `Rc` für Mount und Skin nicht mehr je Bild (Borrow oder Index statt Klon).
- **AC-03** `alloc_je_bild` meldet für `draw` nach dem Aufwärmen höchstens die neue Obergrenze (Ziel 0); Reste stehen als `// PERF(B-1078): Grund` im Code und im Ergebnis der Session.

## Offene Fragen

keine

## Notizen

Eine Session (RU44.3), läuft nach RU44.2, weil beide `replay/**` berühren. Messwert vor RU44: draw max 29.
