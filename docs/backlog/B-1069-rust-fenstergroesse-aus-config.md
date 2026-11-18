# B-1069 · k3c-platform öffnet das Fenster fest mit 1280x720, --size der Wiedergabe wirkt nicht

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
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

`k3c_platform::run_window` öffnet das Fenster fest mit `LogicalSize(1280, 720)` und `PresentMode::Fifo` (`rust/crates/k3c-platform/src/backend/mod.rs`, `gpu.rs`); `Config` hat weder Größe noch VSync. Die Wiedergabe (RU39.1) liest `--size BxH` und schreibt den Wert nur in den Bericht (`fenster`); ohne VSync lässt sich Rust nicht messen.

## Ziel

Fenstergröße und VSync kommen aus `Config`, `k3c-game` (Start, Wiedergabe) gibt `--size` und `--novsync` durch wie das Go-Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um; Ralf misst mit `--size` und `--novsync` (Gate G2).

## Anforderungen

Port ohne Regeländerung (B-1014). `Config` bekommt `size: Option<(u32, u32)>` (logisch, `None` = 1280x720) und `novsync: bool` (Standard `false` = `PresentMode::Fifo`, `true` = Modus ohne Warten, z. B. `Immediate`, sonst `Mailbox`, sonst `Fifo`). Bestehende Aufrufer behalten ihr Verhalten.

## Nicht-Ziele

`native/measure.ps1` (Go-Messskript, außerhalb des Sprints): die Weitergabe von `-Vsync` an `replay` übernimmt der Hauptrechner als eigenes Ticket. `--size` im Spiel-Start außerhalb der Wiedergabe.

## Regeln und Einschränkungen

Rust-Standard `docs/standards/rust-port.md`; Fenster-Setup nur in `k3c-platform`, keine Regeländerung in `k3c-sim` und `k3c-level`.

- **Bereich:** `rust/crates/k3c-platform/src/backend/mod.rs`, `rust/crates/k3c-platform/src/backend/gpu.rs`, `rust/crates/k3c-platform/examples/probe/main.rs`, `rust/crates/k3c-game/src/replay/cli.rs`, `rust/crates/k3c-game/src/replay/run.rs`, `rust/crates/k3c-game/src/demo/mod.rs`
- **Abhängig von:** B-1078
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`k3c-game replay nacht-300.jsonl.gz --size 1920x1080` → Fenster mit 1920x1080 logisch, Bericht nennt 1920x1080. `--novsync` → Fenster ohne Warten auf die Bildwiederholung.

## Ausnahme- und Fehlerfälle

Größe außerhalb Breite 640 bis 3840 lehnt `cli.rs` bereits ab (unverändert). Unterstützt die Grafikkarte keinen Modus ohne Warten, fällt `novsync` auf `Fifo` zurück und loggt es.

## Akzeptanzkriterien

- **AC-01** `k3c-game replay <datei> --size 1920x1080` öffnet ein Fenster mit 1920x1080 (logisch); der Bericht nennt dieselbe Größe, die Warnung „nur im Bericht“ in `run.rs` entfällt.
- **AC-02** `--novsync` setzt einen Present-Modus ohne Warten (Test auf die Modus-Auswahl als reine Funktion); ohne den Schalter bleibt `Fifo`. Die Weitergabe in `native/measure.ps1` ist Folgeticket des Hauptrechners.

## Offene Fragen

keine

## Notizen

Gefunden in RU39.1 (Messwerkzeug). Für G2 unkritisch: Messanleitung nimmt `-Size 1280x720` für beide Programme und `-Vsync`. Die Struktur-Literale in `demo/mod.rs` und `examples/probe/main.rs` brauchen nur `..Default::default()`.
