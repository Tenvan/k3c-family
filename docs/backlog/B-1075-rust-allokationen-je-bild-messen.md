# B-1075 · Allokationen je Bild im Rust-Zeichenpfad sind nicht gemessen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU36
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die vorläufige Messung (`docs/messung/rust-ruckler.md`, RU39) kann Allokationen je Bild im Rust-Zeichenpfad nicht zählen: ein zählender `GlobalAlloc` braucht `unsafe`, der Workspace setzt `unsafe_code = "forbid"`. Allokationen im Bildpfad sind die häufigste Quelle von Ausreißern; Ziel ist absolut performanter und stabiler Code.

## Ziel

Die Zahl und Größe der Allokationen je Bild (Mittel, Maximum, Bilder mit Allokation) für `step` und `draw` der Wiedergabe liegen vor; Treffer im Bildpfad sind als Tickets erfasst.

## Beteiligte und Zielgruppen

Agenten setzen um; Ralf (🧑) entscheidet, falls ein Weg `unsafe` oder eine neue Abhängigkeit braucht.

## Anforderungen

- Messweg ohne `unsafe` im Spiel-Crate: z. B. ein zählender Allokator in einem eigenen Mess-Crate außerhalb des Lint-Verbots, das nur Tests bauen (Freigabe 🧑 für Crate oder Ausnahme).
- Ergebnis je Bild in `docs/messung/rust-ruckler.md`.

## Nicht-Ziele

Optimierungen selbst (eigene Tickets je Treffer).

## Regeln und Einschränkungen

Neue Crates und `unsafe` nur mit Freigabe 🧑 (Port-Leitfaden, `unsafe_code = forbid`).

- **Bereich:** `rust/crates/k3c-game/tests/messung_cpu.rs`, `docs/messung/rust-ruckler.md`
- **Abhängig von:** RU39
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** deps

## Beispiele

nicht relevant: Messaufgabe.

## Ausnahme- und Fehlerfälle

Gibt 🧑 weder `unsafe` noch eine Abhängigkeit frei, bleibt der Wert „nicht messbar“ und der Bildpfad wird per Code-Durchsicht auf Allokationen geprüft.

## Akzeptanzkriterien

- **AC-01** `cargo test --release -p k3c-game --test messung_cpu -- --ignored --nocapture` (oder Nachfolger) nennt Allokationen und Bytes je Bild für `step` und `draw`.
- **AC-02** Für jeden Treffer im Bildpfad existiert ein Ticket oder der Befund ist als unkritisch begründet.

## Offene Fragen

Freigabe für Messweg (eigenes Mess-Crate oder Ausnahme vom Verbot) bei Ralf.

## Notizen

Aus RU39.3 (autonom angelegt, Ralf 2026-11-18: Messungen zum Schluss).
