# RU44 · CLI · Bild-Pfad ohne Allokationen

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1077, B-1078, B-1069
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

RU42 (B-1075) misst in `k3c-game/tests/alloc_je_bild.rs` im Maximum 805 Allokationen je Bild der Wiedergabe `nacht-300` nach dem Aufwärmen (`step` 788, `draw` 29; 179 von 179 Bildern allokieren). Rust-Standard § 11 verlangt 0. Dazu fehlt die Fenstergröße aus `Config` (B-1069). Der Sprint ist für den Second Worker (Lane `zweit`) geplant.

## Ziel

Der Bild-Pfad (`step` und `draw` der Wiedergabe) allokiert nach dem Aufwärmen nicht mehr (Ziel 0, sonst begründete Reste mit `PERF`-Marke), die Obergrenze in `alloc_je_bild` ist auf den neuen Messwert gesenkt, und `--size`/`--novsync` der Wiedergabe wirken.

## Beteiligte und Zielgruppen

Der Second Worker setzt um; Ralf prüft die Wirkung am Messlauf (Ruckler, Gate G2). Optimierung nach Messung (Ralf 2026-11-18).

## Anforderungen

Tickets B-1077 (step), B-1078 (draw), B-1069 (Fenster). Kein Eingriff in `src/menu/**`, `src/hud/**`, `src/devtools/**` (RU9, RU23, RU29), `k3c-sim` und `k3c-level` (RU11, RU6). Messung nur mit dem Test `alloc_je_bild` (mit `K3C_GRAFIK_DIR` auf eine vorhandene Grafik, sonst übersprungen), keine Sim-, Balance- oder Gen-Läufe.

## Nicht-Ziele

Menus und HUD (RU9, RU23), Sim und Level (RU11, RU6), neue Messwerkzeuge, `native/measure.ps1` (Folgeticket des Hauptrechners), Draw-Call-Bündelung.

## Regeln und Einschränkungen

Rust-Standard § 11; Sessions laufen nacheinander (RU44.1 → .2 → .3 → .4 → .5), weil sie sich `replay/**` teilen. Vor dem Push `git merge origin/main`.

- **Bereich:** `rust/crates/k3c-game/src/replay/**`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/view/**`, `rust/crates/k3c-game/src/demo/mod.rs`, `rust/crates/k3c-rig/src/**`, `rust/crates/k3c-platform/src/backend/**`, `rust/crates/k3c-platform/examples/probe/main.rs`, `rust/crates/k3c-game/tests/alloc_je_bild.rs`
- **Abhängig von:** RU42
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`cargo test --release -p k3c-game --test alloc_je_bild -- --nocapture` nennt vorher Maximum 805, nachher höchstens den neuen Wert (Ziel 0).

## Ausnahme- und Fehlerfälle

Eine Allokation, die nicht ohne Regeländerung fällt, bekommt `// PERF(B-xxxx): Grund`; die Review-Session nimmt sie in den Messwert auf und meldet sie als Befund für den Hauptrechner.

## Akzeptanzkriterien

- **AC-01** `step` der Wiedergabe allokiert nach dem Aufwärmen nicht mehr (B-1077) oder höchstens begründete Reste.
- **AC-02** `draw` allokiert nach dem Aufwärmen nicht mehr (B-1078) oder höchstens begründete Reste.
- **AC-03** `--size` und `--novsync` der Wiedergabe wirken (B-1069).
- **AC-04** `MAX_ALLOCS_JE_BILD` in `alloc_je_bild.rs` steht auf dem neuen Messwert (Ziel 0), der Test ist grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU44.1 | `RU44.1-blend-ohne-allokation.md` | Umsetzung | autonom | fertig |
| RU44.2 | `RU44.2-takt-zustand-ohne-allokation.md` | Umsetzung | autonom | offen |
| RU44.3 | `RU44.3-draw-ohne-allokation.md` | Umsetzung | autonom | offen |
| RU44.4 | `RU44.4-fenstergroesse-vsync.md` | Umsetzung | autonom | offen |
| RU44.5 | `RU44.5-review-obergrenze.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
