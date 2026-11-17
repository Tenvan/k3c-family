# RU4.2 · Canvas Teil 2: Transform, Dreiecksnetz, Formen, Filter, Freigeben

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.1
- **Tickets:** B-1014
- **Kriterien:** AC-01

## Ziel

`Canvas` kann alles zeichnen, was das Go-Spiel über `DrawImage` mit `GeoM`, `DrawTriangles` und `vector.*` zeichnet, und Texturen mit Filter anlegen und wieder freigeben.

## Kontext

Vorgaben: RU4-README › Vorentscheidungen › Canvas Teil 2. Bestehend: `rust/crates/k3c-platform/src/lib.rs` (`Canvas`), `batch.rs` (`Frame`, Instanzen, `INST_BYTES`), `backend/gpu.rs`, `backend/shader.wgsl` (Instanz = Ziel-Rechteck + Quell-Rechteck + Farbe), `headless.rs` (`NullCanvas`). Go-Aufrufer zur Orientierung: `grep -rn "vector\.\|DrawTriangles\|GeoM\.Rotate\|FilterNearest" native/k3c-game native/k3c-rig --include=*.go` (genutzt: `FillRect`, `StrokeRect`, `FillCircle`, `StrokeCircle`, `StrokeLine`, `FillPath` mit Geraden, `DrawTriangles` in `k3c-rig/paint/painter.go` und `world/lightdraw.go`, Filter linear/nächster Nachbar). Die Instanz bekommt eine 2×3-Matrix statt des Ziel-Rechtecks; `sprite(dst)` wird zu einer Matrix aus Skalierung und Verschiebung. Netze und Formen gehen als Dreiecke in einen eigenen Vertex-Puffer (eine zweite Pipeline), Kreise als Polygon mit fester Segmentzahl.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/crates/k3c-game/src/demo/**`

## Nicht-Ziele

Shader und Blend-Modi (Fluss, Licht). Kurven in Pfaden (`ArcTo`, `QuadTo`): braucht ein Aufrufer sie, Ticket. Schrift (RU4.3).

## Schritte

1. `src/affine.rs`: `Affine { a, b, c, d, tx, ty }` mit `IDENTITY`, `translate`, `scale`, `rotate`, `concat` in der Reihenfolge von `ebiten.GeoM` (Go-Doku lesen: `GeoM.Translate` hängt hinten an).
2. `Canvas`: `sprite_tf`, `mesh(tex, &[Vertex], &[u16])` mit `Vertex { pos: [f32; 2], uv: [f32; 2], color: [f32; 4] }`, `fill_rect`, `stroke_rect`, `fill_circle`, `stroke_circle`, `stroke_line`, `fill_path(&[[f32; 2]], color)`, `texture_with(rgba, w, h, Filter)`, `free_texture(TexId)`. `rect` bleibt als Kurzform von `fill_rect`.
3. `src/shape.rs`: Formen → Dreiecke (Linie als Viereck mit Breite, Kreis mit fester Segmentzahl, Polygon per Ohren-Schnitt); reiner Code, ohne GPU.
4. `batch.rs` und `backend/gpu.rs`: Instanz mit Matrix; Dreiecks-Pipeline mit wiederverwendetem Vertex- und Index-Puffer; Sampler je Filter; `free_texture` gibt die Textur am Bildende frei. Zeichenreihenfolge bleibt Ebene → Ausschnitt → Textur → Aufruf.
5. `NullCanvas` zählt die neuen Aufrufe.
6. Tests: `src/affine_tests.rs` (gleiche Matrix wie GeoM für Verschieben-Drehen-Skalieren), `src/shape_tests.rs` (Dreieckszahl und Fläche je Form, konkaves Polygon), `tests/alloc.rs` erweitert (0 Allokationen mit allen neuen Aufrufen).

## Fertig, wenn

- [ ] AC-01: `cargo test -p k3c-platform` grün, inklusive `affine_tests`, `shape_tests` und `tests/alloc.rs` mit allen neuen Aufrufen.
- [ ] AC-01: `cargo run --release -p k3c-platform --example probe` läuft weiter mit 60 Bildern/s (Titel).
- [ ] Demo `--headless --steps 600` Code 0; `task rust:lint` grün.

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
