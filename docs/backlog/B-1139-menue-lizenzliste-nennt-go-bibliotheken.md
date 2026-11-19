# B-1139 · Die Lizenz-Seite des Rust-Spiels nennt Go-Bibliotheken statt der Rust-Crates

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU58
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19 (Ralf: ausdrücklich)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`rust/crates/k3c-game/src/menu/licenses.rs` ist ein wörtlicher Port der Go-Liste (Ebitengine, Oto, go-text, x/image, go-mp3, oggvorbis). Das Rust-Spiel linkt keines davon, sondern u. a. wgpu/naga, winit-Verwandte, fontdue, cpal, gilrs, png, flate2. Die Menü-Seite und der Kopf von `LIZENZEN.txt` nennen damit falsche Bibliotheken. Der Bildvergleich `testdata/rust/bild/menu/licenses.go.png` ist gegen das Go-Spiel golden.

## Ziel

Die Menü-Liste nennt die tatsächlichen Rust-Abhängigkeiten (Gruppen Engine, Schrift, Grafik, Ton) plus Werke (Schrift, Klänge), der Bildvergleich wird nach dem Go-Rückbau neu aufgenommen.

## Beteiligte und Zielgruppen

Entwickler; 🧑 Ralf prüft die Seite im Spiel.

## Anforderungen

- Liste aus `cargo metadata` ableiten (nur direkte Abhängigkeiten der Spiel-Crates), Texte DE/EN unverändert in der Struktur.
- Ralf 2026-11-19: nicht mehr nach RU37, sondern in RU58.6 (vor dem Löschen des Go-Spiels); der Bildvergleich der Lizenz-Seite gegen Go als Golden wird aufgegeben, die Baseline neu festgelegt (`licenses.png`).

## Nicht-Ziele

Volltexte (stehen in `LIZENZEN.txt`, B-1121).

## Regeln und Einschränkungen

Änderung an Menü-Texten und Golden (einzige erlaubte Golden-Ausnahme, Ralf 2026-11-19).

- **Bereich:** `rust/crates/k3c-game/src/menu/licenses.rs`, `rust/crates/k3c-game/src/menu/image_tests.rs`, `testdata/rust/bild/menu/**`, `tools/rustpaket/**`
- **Abhängig von:** RU58.4
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Lizenz-Seite zeigt „Engine: wgpu, naga …“ statt „Ebitengine“.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Die Liste nennt nur Werke, die das Rust-Spiel nutzt, und deckt sich mit den direkten Abhängigkeiten in `Cargo.toml`.
- **AC-02** Bildvergleich und Golden der Lizenz-Seite sind neu aufgenommen und begründet.

## Offene Fragen

Bis dahin steht in `LIZENZEN.txt` der alte Kopf plus die vollständige echte Crate-Liste (B-1121); 🧑 entscheidet, ob das für die Weitergabe vorläufig genügt.

## Notizen

Gefunden bei der Planung von RU57 (2026-11-19).
