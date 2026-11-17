# B-1038 · read_png ist doppelt in k3c-rig und k3c-game und wird in RU7.6 zusammengeführt

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU7
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
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

- **AC-01** `grep -rn "fn read_png" rust` findet genau eine Definition; `cargo test` der betroffenen Crates grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
