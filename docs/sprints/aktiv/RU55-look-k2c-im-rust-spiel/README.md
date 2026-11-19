# RU55 · CLI · Look und Maßstab nach K2C-Vorbild im Rust-Spiel

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1043, B-1045, B-1044, B-1046, B-1047, B-1130
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Der Standard `docs/standards/bildaufbau.md` (K2C-Auswertung 2026-11-17) verlangt drei Bänder (Welt 58 %, Ufer 7 %, Fluss als Spiegel 35 %), ≈ 5 M sichtbare Welt, Tönung und Licht je Tageszeit, Dunst zwischen den Ebenen und einen Wald, der mit dem Roden offener wird. Das Rust-Spiel zeichnet heute wie das Go-Spiel: Gehlinie bei 2/3 (`view/cell.rs` `WORLD_SHARE`), Ufer 20 px (`world/ground.rs`), Welt 420/32 Units hoch (`VIEW_UNITS` an fünf Stellen), Wellen-Fluss ohne Spiegelung (`world/river.rs`, `river.wgsl`), Lichtkarte nur als Helligkeit (`world/lightdraw.rs`, B-1056). Die Sprints BB1 (Assets) und BB2 (Spiel, noch auf `develop` und Go bezogen) sind nicht umgesetzt; BB2 ist durch diesen Sprint ersetzt, BB1 (Asset-Größen, Waldwand-Varianten, Leitmotive) bleibt als eigener Papierschnitt-Sprint bestehen.

## Ziel

Das Rust-Spiel hat Aufbau, Maßstab und Licht des K2C-Vorbilds: drei Bänder 58/7/35 %, M≈ 12 % der Ansicht, ≈ 5 M Welt, Spiegel-Fluss, Tönung, Lichthöfe und Dunst je Tageszeit, Wald nach Rodung. Am Ende sichtbar: am TV in einer und zwei Ansichten, plus Bericht mit Referenz-Aufnahmen (Abnahme B-1130).

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-hud`, `k3c-grafik`, `k3c-welten`. Aufnahmen, Messung und Bildvergleich: Second Worker (Lane zweit, B-1130). Abnahme am TV und Wahl der Weite: 🧑.

## Anforderungen

Siehe `B-1043 › Anforderungen`, `B-1045`, `B-1044`, `B-1046`, `B-1047` und `B-1130`. Sprint-eigen: **Profil `go` statt Bruch mit der Go-Abnahme.** Die neuen Werte stehen in `data/look.json` (`profiles.k2c` Standard, `profiles.go` alter Stand); das Profil wählt `K3C_LOOK`. Bildvergleiche gegen Go (RU32, Gate G4 in RU36) laufen im Profil `go` und bleiben gültig. Frame-Budget: 60 Hz mit zwei Ansichten, schlechtester Frame zählt; RU42-Leitplanken (kein `unwrap`/`expect`/`panic` im Produktivcode) und die Allokations-Obergrenze in `tests/alloc_je_bild.rs` gelten unverändert.

## Nicht-Ziele

Asset-Größen, neue Waldwand-Varianten und Leitmotive (BB1, B-1038 bis B-1042); Engine-Regeln; neue Wetter-Mechanik; Partikel (Schnee, Regen); Umschalter des Profils im Cheat-Fenster (Folgeticket nach RU56).

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Das Spiel zeichnet nur; Werte nach `data/` (Go liest `data/` mit; neue Dateien und Felder mit Standard, Go-Golden unverändert). Golden werden nie zugunsten von Rust neu geschrieben. Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742). Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen, keine neuen Abhängigkeiten. Die Sessions laufen der Reihe nach (alle berühren `data/look.json`).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/view/cell.rs`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/hud/layout.rs`, `rust/crates/k3c-game/src/game/layout.rs`, `rust/crates/k3c-game/src/game/draw.rs`, `rust/crates/k3c-game/src/replay/layout.rs`, `rust/crates/k3c-game/src/still/state.rs`, `rust/crates/k3c-game/src/audio/mix.rs`, `rust/crates/k3c-game/src/props/decor.rs`, `rust/crates/k3c-game/src/fx/paint.rs`, `rust/crates/k3c-platform/src/river.rs`, `rust/crates/k3c-platform/src/backend/river.wgsl`, `rust/crates/k3c-platform/src/backend/gpu.rs`, `rust/crates/k3c-data/src/look.rs`, `rust/crates/k3c-data/src/lib.rs`, `data/look.json`, `data/light.json`, `data/paper.json`
- **Abhängig von:** RU54
- **Braucht 🧑:** Entscheidung, Abnahme
- **Sperre:** –

Hinweis Kollision: RU54 (Farben im Bildpfad) und RU36.3 (Befunde aus der Abnahme am PC, `rust/crates/**`) können dieselben Dateien in `world/**` berühren; RU55 startet nach RU54 und rebased vor jeder Session auf `main`.

## Beispiele

Nacht am Hub, 2 Ansichten: Gehlinie je Hälfte bei 58 %, Lagerfeuer und Mond hell im Spiegel, Waldrand dunkel, alles dunkelblau getönt, Fackel des Monarchen mit warmem Hof.

## Ausnahme- und Fehlerfälle

Profil `go` oder unbekannter Wert in `K3C_LOOK` → `go` bzw. Standard `k2c` mit Log-Zeile; ohne Fluss-Shader bleibt die Wellen-Darstellung (B-1044); Höhle und Mine ohne Himmels-Tönung (B-1046); sehr flache Fenster: Ufer höchstens ein Drittel des Flusses (B-1043).

## Akzeptanzkriterien

- **AC-01** Gehlinie 58 %, Ufer 7 %, Look-Werte aus `data/look.json` mit Profilen `k2c` und `go` (B-1043/AC-01).
- **AC-02** Sichtbare Welt ≈ 5 M aus `data/`, M ≈ 12 % der Ansicht, Split-Screen ≈ 16 % je Hälfte (B-1045/AC-01).
- **AC-03** Fluss spiegelt die Welt im Frame-Budget (B-1044/AC-01).
- **AC-04** Tönung je Tageszeit und Ereignis, Dunst zwischen den Ebenen (B-1046/AC-01).
- **AC-05** Lichthöfe an allen Lichtquellen (B-1046/AC-02).
- **AC-06** Wald und Parallax je Welt, Wald nach Rodung (B-1047/AC-01).
- **AC-07** Abnahme: Aufnahmen, Pixelmessung, Bildvergleich Go im Profil `go`, Frame-Zeit (B-1130, Lane zweit) und am TV (🧑, B-1043/AC-02, B-1044/AC-03, B-1045/AC-02, B-1046/AC-03).
- **AC-08** Review: `task check:diff` und Fachprüfung grün, Abnahme-Ticket B-1130 mit Szenenliste vollständig.

## Offene Fragen

Weite 4, 4,5 oder 5 M (B-1045): Default 5 M, 🧑 entscheidet am TV nach B-1130 (ein Wert in `data/look.json`, Änderung ohne Code). Nicht blockierend.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU55.1 | `RU55.1-baender-look-profil.md` | Umsetzung | autonom | fertig |
| RU55.2 | `RU55.2-masstab-weite-monarchenhoehen.md` | Umsetzung | autonom | fertig |
| RU55.3 | `RU55.3-spiegel-fluss.md` | Umsetzung | autonom | fertig |
| RU55.4 | `RU55.4-toenung-dunst.md` | Umsetzung | autonom | fertig |
| RU55.5 | `RU55.5-lichthoefe.md` | Umsetzung | autonom | fertig |
| RU55.6 | `RU55.6-wald-parallax-je-welt.md` | Umsetzung | autonom | offen |
| RU55.7 | `RU55.7-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
