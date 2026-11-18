# B-1051 · read_png ist doppelt in k3c-rig und k3c-game und wird in RU7.6 zusammengeführt

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU7
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`read_png` gibt es doppelt: `rust/crates/k3c-rig/src/paint/assets.rs` (liefert Bytes, Breite, Höhe) und `rust/crates/k3c-game/src/art/png.rs` (liefert `Rgba`). Aufgefallen im Review RU5.5.

## Ziel

Eine gemeinsame Funktion statt zwei Kopien, damit PNG-Laden nur an einer Stelle gepflegt wird.

## Beteiligte und Zielgruppen

Entwickler; Umsetzung in RU7.6 (Brücke `art/rigs.rs`).

## Anforderungen

- Ein einziges `read_png` im Rust-Workspace; beide Aufrufer nutzen es.
- Ergebnis der Bilder unverändert.

## Nicht-Ziele

Neue Formate, neue Abhängigkeiten ohne Freigabe 🧑.

## Regeln und Einschränkungen

Port ohne Verhaltensänderung (B-1014); Datei ≤ 400 Zeilen.

- **Bereich:** `rust/crates/k3c-rig/src/paint/assets.rs`, `rust/crates/k3c-game/src/art/png.rs`
- **Abhängig von:** RU5
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: reine Zusammenführung.

## Ausnahme- und Fehlerfälle

nicht relevant: Fehlerverhalten bleibt wie bisher.

## Akzeptanzkriterien

- **AC-01** Genau eine Implementierung des vormultiplizierenden PNG-Dekoders: `k3c_rig::paint::read_png`; `k3c-game/src/art/png.rs` ist nur eine dünne Hülle (Typ `Rgba`), `cargo test` von k3c-rig und k3c-game grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Ergebnis (2026-11-18, Claude): Schon in RU7.6 umgesetzt, auf main geprüft: `art/png.rs::read_png` ruft `k3c_rig::paint::read_png` und baut nur `Rgba` daraus; Test `vormultiplizieren_wie_go` grün. Abweichung vom Wortlaut von AC-01: `grep -rn "fn read_png" rust` findet drei Namen, aber nur eine Dekodier-Implementierung. `k3c-game/src/still/pngio.rs::read_png` (RU13.6, Standbild) ist eine andere Funktion: gerades RGBA mit RGB-Unterstützung, ohne Vormultiplikation, für den Pixelvergleich mit Go; sie ist keine Kopie.
