# B-1077 · PERF: Replay-Takt (step) allokiert bis zu 788 mal je Bild, Ziel 0

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

`k3c-game/tests/alloc_je_bild.rs` (RU42, B-1075) misst in der Wiedergabe `nacht-300` nach 120 Aufwärmbildern im Maximum 788 Allokationen in `step` (Mittel des Bildes gesamt 478). Fundstellen vermutet: `ReplayApp::step` in `replay/app.rs` mit `blend.capture` (Welt kopieren), `next_world` und `install` (Zustand je Aufzeichnungs-Takt bauen).

## Ziel

`step` allokiert nach dem Aufwärmen nicht mehr; die Obergrenze im Test sinkt entsprechend.

## Beteiligte und Zielgruppen

Agenten setzen um; Messung gilt (Ralf 2026-11-18: Optimierung am Ende, nach Messung).

## Anforderungen

- Puffer in `blend.capture`, `next_world`, `install` wiederverwenden.
- `MAX_ALLOCS_JE_BILD` im Test absenken.

## Nicht-Ziele

Optimierung des Zeichenpfads (B-1078).

## Regeln und Einschränkungen

Rust-Standard § 11 (`docs/standards/rust-port.md`): kein Allokieren im Takt-Pfad, Puffer mit `clear()` wiederverwenden; kein `unwrap`/`expect`/`panic!` im Nicht-Testcode. Gemessen wird nur mit `k3c-game/tests/alloc_je_bild.rs` (setzt die gerenderte Grafik voraus, `K3C_GRAFIK_DIR` oder Test übersprungen).

- **Bereich:** `rust/crates/k3c-game/src/replay/**`, `rust/crates/k3c-game/tests/alloc_je_bild.rs`
- **Abhängig von:** RU42
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

nicht relevant: Messaufgabe.

## Ausnahme- und Fehlerfälle

Ist ein Teil nur im Replay (nicht im echten Spiel) heiß, genügt die Begründung als unkritisch.

## Akzeptanzkriterien

- **AC-01** `ReplayApp::step` (`replay/app.rs`, `replay/blend.rs`) wiederverwendet seine Puffer: `blend.capture`, `apply` und `restore` legen nach dem Aufwärmen keine neuen `Vec`/`BTreeMap`-Knoten an.
- **AC-02** `next_world` und `install` bauen den Zustand je Aufzeichnungs-Takt ohne Allokation, soweit das ohne Regeländerung geht; Reste stehen mit Begründung als `// PERF(B-1077): Grund` im Code und im Ergebnis der Session.
- **AC-03** `alloc_je_bild` meldet für `step` nach dem Aufwärmen höchstens die neue Obergrenze (Ziel 0), die Obergrenze `MAX_ALLOCS_JE_BILD` ist gesenkt (macht die Review-Session des Sprints für den Gesamtwert).

## Offene Fragen

keine

## Notizen

Zwei Sessions: RU44.1 (Blend: capture/apply/restore), RU44.2 (next_world, install, Kameras, Rest). Messwert vor RU44: step max 788.
