# B-1139 · Die Lizenz-Seite des Rust-Spiels nennt Go-Bibliotheken statt der Rust-Crates

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU37
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
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
- Erst nach RU37 (Go-Spiel entfernt), weil der Bildvergleich der Seite gegen Go läuft.

## Nicht-Ziele

Volltexte (stehen in `LIZENZEN.txt`, B-1121).

## Regeln und Einschränkungen

Änderung an Menü-Texten und Golden.

- **Bereich:** `rust/crates/k3c-game/src/menu/licenses.rs`, `rust/crates/k3c-game/src/menu/image_tests.rs`, `testdata/rust/bild/menu/**`
- **Abhängig von:** RU37
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
