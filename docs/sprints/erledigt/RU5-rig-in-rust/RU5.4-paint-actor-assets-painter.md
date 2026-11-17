# RU5.4 · paint: actor, assets, painter als Dreiecksnetz

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru5
- **Abhängig von:** RU5.3, RU4.2
- **Tickets:** B-1014
- **Kriterien:** AC-02, AC-03

## Ziel

Die Rig-Figur wird in Rust als Dreiecksnetz über `k3c_platform::Canvas::mesh` gezeichnet, mit Plätzen für Rüstung und Waffe; Vergleichsbilder gegen Go.

## Kontext

Leitfaden `docs/standards/rust-port.md`. Quellen: `native/k3c-rig/paint/actor.go` (258), `assets.go` (167), `painter.go` (148, Test `painter_test.go`). `ebiten.DrawTriangles` → `Canvas::mesh` (RU4.2). Vergleichsbilder: Go erzeugt Referenz-Netze (Vertices, Indizes) je Figur und Pose über einen Test `native/k3c-rig/paint/rustref_test.go` nach `testdata/rust/rig/paint/`; Rust vergleicht Netze (Toleranz aus `native/k3c-rig/RIG.md`, sonst 1e-6 je Koordinate). Ein Pixelvergleich ist nicht nötig, wenn die Netze gleich sind. Nachzug NA1.6 (Rig-Export der Bosse) ist nicht Teil dieser Session; dafür ein Ticket im Sprint RU5 anlegen, falls die Boss-Specs fehlen.

## Erlaubte Dateien

- `rust/crates/k3c-rig/**`
- `native/k3c-rig/paint/rustref_test.go`
- `testdata/rust/rig/**`

## Nicht-Ziele

Änderungen an produktivem Go-Code. Brücke ins Spiel (RU7.6).

## Schritte

1. Go-Dateien und Test ganz lesen; `k3c-platform` Canvas-Trait ansehen.
2. Port actor, assets, painter; `painter_test.go` portieren.
3. Referenz-Netze Go → JSON, Rust-Vergleich je Figur und Pose.
4. `cargo test -p k3c-rig`, `task rust:lint`.

## Fertig, wenn

- [ ] AC-02: Figur als Dreiecksnetz über `Canvas::mesh`, Rüstung und Waffe haben Plätze.
- [ ] AC-03: Netze je Figur und Pose innerhalb der Schwelle zu Go.
- [ ] `task rust:lint` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 geprüft: `paint/actor.rs`, `assets.rs`, `painter.rs` in Rust; die Figur zeichnet über `k3c_platform::Canvas::mesh` (Bänder, Zügel) und `sprite_tf` (Teile, Elemente). Rüstung und Waffe haben Plätze (`element_at` für `ruestung` und `waffe`); Reiter am Platz `sattel`. `cargo test -p k3c-rig` grün (22 Tests), `task rust:lint` (workspace, -D warnings) grün.
AC-03 geprüft: `native/k3c-rig/paint/rustref_test.go` (`-update`) schreibt `testdata/rust/rig/paint/figures.json` (225 Figuren und Posen: zu Fuß, Vierbeiner allein, Monarch auf allen Reittieren in 4 Gangarten, Varianten, Rüstung; 2807 Bilder, 646 Netze). `figuren_gegen_go` vergleicht Textur, Matrix, Alpha, Ecken (Position, UV, Farbe) und Indizes. Schwelle: 1e-6 plus 2,5e-7 relativ je Koordinate (Go rundet auf float32), Indizes exakt; alle gleich. Pixelvergleich entfällt (Netze gleich). `painter_alpha` ist portiert, dazu `parse_color`.

Abweichungen: (1) Die Go-Referenz ruft die echten Actor-Funktionen für Zustand, Pose, Umhang-Physik und Painter-Netze auf, bildet aber die Verteilung der Zeichen-Aufrufe (`Actor.Draw`: drawMount, drawRider) im Test nach, weil sich das Ziel-Image von Ebitengine nicht abhören lässt, ohne Produktivcode zu ändern. (2) Neue Abhängigkeiten von k3c-rig: `k3c-platform` (Workspace) und `png` (Workspace-Eintrag, in k3c-game schon genutzt); `read_png` ist dort kopiert (RU7.6 kann es zusammenführen). (3) `WritePNG` aus assets.go ist nicht portiert (kein Aufrufer in Rust; gehört zu Sheets/Shots). (4) Painter hat kein `Dst`-Feld; der `Canvas` wird je Aufruf übergeben. (5) `Actor::start` ist neu (Zustand vor dem ersten Lösen setzen, für Tests und Sheets). Boss-Specs (NA1.6) fehlen nicht: alle 5 Boss-Rigs liegen unter assets/bosse und sind in den Vergleichen enthalten, also kein Ticket nötig.
