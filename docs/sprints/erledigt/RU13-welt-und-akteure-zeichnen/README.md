# RU13 · CLI · Welt und Akteure zeichnen

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** 983398f2b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

`world` (1255 Zeilen) zeichnet Boden, Hintergrund und Tageszeit, `actors` (1447 Zeilen) Monarch, Truppen und Gegner aus dem Sim-Zustand.

## Ziel

Ein Sim-Zustand wird im Rust-Spiel als Welt mit Figuren gezeichnet. Am Ende sichtbar: Standbild aus einem Go-Zustand, Bildvergleich gegen das Go-Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 5 von 26, 7 Sessions (B-1015; eine mehr als geplant: der Bildvergleich bekommt eine eigene Session, weil das Standbild-Werkzeug und die Go-Referenzbilder mehr sind als ein Nebenschritt).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `native/k3c-game/world/` → `rust/crates/k3c-game/src/world/`, `native/k3c-game/actors/` → `rust/crates/k3c-game/src/actors/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru13` (eine nach der anderen, ein Commit je Session). `measure_test.go` beider Pakete (Messwerkzeug) wird nicht portiert, die Messung übernimmt RU38. Der Fluss-Shader (`water/river.kage`) wird nicht portiert, Rust zeichnet den Fluss als Kacheln (Ticket aus RU13.2).

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `native/k3c-game/world/`: backdrop (196), camera (142, T), cull (54, T), ground (56), layers (98, T) | 546 |
| 2 | `native/k3c-game/world/`: light (150, T), lightdraw (128), river (134, T), water (191, T) | 603 |
| 3 | `native/k3c-game/world/`: world (106); `native/k3c-game/actors/`: actors (92), anim (168, T) | 366 |
| 4 | `native/k3c-game/actors/`: catalog (222, T), combatmarks (123, T), draw (130, T), figures (326, T) | 801 |
| 5 | `native/k3c-game/actors/`: levels (72, T), markdraw (153), marks (114, T), pose (53) | 392 |
| 6 | Standbild `k3c-game still` und Bildvergleich an fünf Ständen (Go-Test `rustref_test.go`) | – |
| 7 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2708 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, Session 6 liefert den Bildvergleich, das Review prüft alle.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Einzige Go-Änderung: die neue Testdatei `native/k3c-game/world/rustref_test.go` (Session 6).

- **Bereich:** `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/still/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/tests/**`, `native/k3c-game/world/rustref_test.go`, `testdata/rust/bild/**`
- **Abhängig von:** RU7, RU8
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Welt und Tageszeit eines geladenen Zustands sehen aus wie im Go-Spiel (Bildvergleich an fünf Ständen).
- **AC-02** Monarch, Truppen und Gegner stehen und laufen an den Positionen des Zustands; die Paket-Tests sind portiert.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Fluss-Shader:** nicht portiert, Kachel-Fassung in Rust, Ticket für einen WGSL-Shader (RU13.2).
- **Bildvergleich:** Hat `k3c-platform` kein Auslesen ohne Fenster, baut RU13.6 einen kleinen Software-Rasterer im Werkzeug `still`; der Pixelvergleich ersetzt nicht die Sichtprüfung von 🧑 im Abnahme-Ticket.
- **Zusatz-Session:** 7 statt 6 Sessions (Bildvergleich eigene Session).

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Das Go-Spiel wächst nicht weiter). Diese Arbeiten entstehen direkt im Rust-Client, sobald die Sim-Seite steht. Spec steht im Ticket:

- SI4.4 Gatter und Reittier je Monarch im Spiel (B-1000)
- WE6.1 Bau und Endboss in jedem Biom, Ersatz-Asset der Basisart (B-993)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU13.1 | `RU13.1-world-backdrop-camera-cull-ground-layers.md` | Umsetzung | autonom | fertig |
| RU13.2 | `RU13.2-world-light-river-water.md` | Umsetzung | autonom | fertig |
| RU13.3 | `RU13.3-world-actors-anim.md` | Umsetzung | autonom | fertig |
| RU13.4 | `RU13.4-actors-catalog-combatmarks-draw-figures.md` | Umsetzung | autonom | fertig |
| RU13.5 | `RU13.5-actors-levels-markdraw-marks-pose.md` | Umsetzung | autonom | fertig |
| RU13.6 | `RU13.6-standbild-bildvergleich.md` | Umsetzung | autonom | fertig |
| RU13.7 | `RU13.7-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-17: AC-01 und AC-02 geprüft, Nachweise in den Ergebnissen RU13.1 bis RU13.6; alle Go-Tests von `world` und `actors` (außer `measure_test.go`) sind in Rust portiert (Ausnahmen mit Grund: Shader-, Alloc-Tests, `LevelSheets(nil)`), kein `pending.rs`, `testdata/golden` und `native/`/`engine/` unverändert bis auf `world/rustref_test.go`. `task check` und `task check:go` grün; Reviewer `k3c-reviewer`: nur drei niedrige Befunde, keine Regelabweichung.
Bildtests laufen ohne Grafik übersprungen, Sichtprüfung durch Ralf offen. Abnahme-Ticket B-1059 (Gen mit `K3C_STILL_REQUIRE=1`, Race, Sichtprüfung). Weitere Tickets aus dem Sprint: B-1055 (Fluss-Shader), B-1056 (Multiply-Blend), B-1058 (Canvas-Sortierung).
