# B-1116 · replay/report.rs nutzt Report, FrameStats und FrameLog aus game/measure.rs

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
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

`replay/report.rs` (RU38.3) hat eigene `Report`, `FrameStats`, `percentile` und `replay/frames.rs` ein eigenes `FrameLog`. Es fehlen das Feld `bilder` (`FrameReport`) und die Kennwerte je Bild. RU30.4 hat die vollständige Fassung in `game/measure.rs`, `game/measure_frames.rs` angelegt, `replay/` blieb unberührt (Nicht-Ziel).

## Ziel

Ein Bericht, ein Satz Rechenfunktionen: `replay/` benutzt per `pub use` die Typen und `percentile`/`round1` aus `game/measure*.rs`; die Doppelungen entfallen.

## Beteiligte und Zielgruppen

Entwickler; Second Worker liest die Berichte.

## Anforderungen

Ein Bericht, ein Satz Rechenfunktionen; Berichtsformat der Wiedergabe bleibt lesbar (Felder nur ergänzt).

## Nicht-Ziele

Neue Messläufe (Hauptrechner führt keine Messungen aus).

## Regeln und Einschränkungen

RU42; Dateien <= 400 Zeilen.

Planungsangaben:

- **Bereich:** `rust/crates/k3c-game/src/replay/report.rs`, `rust/crates/k3c-game/src/replay/frames.rs`, `rust/crates/k3c-game/src/replay/mod.rs`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`replay_headless` schreibt den Bericht mit `bilder`, wenn `--measure-frames` gesetzt ist.

## Ausnahme- und Fehlerfälle

nicht relevant, reine Zusammenführung.

## Akzeptanzkriterien

- **AC-01** `replay/report.rs` und `replay/frames.rs` enthalten keine eigene Fassung von `Report`, `FrameStats`, `percentile`, `FrameLog` mehr.
- **AC-02** Der Bericht der Wiedergabe trägt `bilder` mit `--measure-frames`-Kennwerten wie im Spiel.
- **AC-03** `cargo test -p k3c-game` und `replay_headless`/`replay_trace` bleiben grün.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
