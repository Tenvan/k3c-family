# RU4.2 · Canvas Teil 2: Transform, Dreiecksnetz, Formen, Filter, Freigeben

- **Status:** fertig
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

- **AC-01 geprüft:** `cargo test -p k3c-platform` grün (42 Unit-Tests), darin `affine_tests` (Verschieben → Drehen → Skalieren → Verschieben Schritt für Schritt gegen die GeoM-Formeln, Concat-Reihenfolge), `shape_tests` (Kreis 48 Segmente mit Fläche πr², Ring, Quadrat beide Umlaufrichtungen, konkave L-Form, Stern, geschlossener Pfad, entartete Eingaben) und `batch_tests` (Matrix aus `sprite_tf`, Linie als gedrehtes Rechteck, Netze und Formen bündeln nach Aufruf, Freigabe, Filter). `tests/alloc.rs` zeichnet zusätzlich `sprite_tf`, `mesh`, `stroke_rect`, `fill_circle`, `stroke_circle`, `stroke_line`, `fill_path`: 0 Allokationen in 120 Bildern.
- **AC-01 geprüft (Lauf):** `cargo run --release -p k3c-platform --example probe -- n=20000 secs=3`: Vulkan Median 16,67 ms (60 Bilder/s), CPU Median 1,1 ms; mit allen Formen (`n=2000`) Vulkan und DX12 ohne Validierungsfehler. **Ralf prüft:** In der Probe stehen rechts ein gedrehtes Sprite, ein bunt verlaufendes Netz, ein grüner Rahmen, ein roter halbdurchsichtiger Kreis, ein weißer Ring, ein kreisender gelber Zeiger und eine blaue L-Form – alle sauber und ohne Löcher.
- **Lauf:** Demo `--headless --steps 600` Code 0; `cargo clippy --workspace --all-targets -D warnings` grün; `task check:diff` grün.
- **API:** `Affine { a, b, c, d, tx, ty }` (`IDENTITY`, `translate`, `scale`, `rotate`, `concat`, `apply`; Reihenfolge wie GeoM), `Vertex { pos, uv (Textur-Pixel), color }`, `Filter::{Linear, Nearest}`; `Canvas`: `texture` (Standard linear) → `texture_with`, `free_texture`, `sprite_tf`, `mesh`, `fill_rect` (= `rect`), `stroke_rect`, `fill_circle`, `stroke_circle`, `stroke_line`, `fill_path`. `NullCanvas` zählt `meshes`, `shapes`, `freed`. `Frame`: `vertices`, `indices`, `write_vertices`, `take_freed`, `Run.kind` (`Quads`/`Tris`).
- **Umsetzung:** Instanz = Einheitsquadrat mit 2×2-Matrix + Verschiebung (64 Byte); `sprite(dst)` ist die Achsen-Matrix. `stroke_line` ist ein gedrehtes Instanz-Rechteck, `stroke_rect` vier Rechtecke; Kreise, Ringe und Polygone gehen als Dreiecke über eine zweite Pipeline mit u32-Indexpuffer; `finish` kopiert die Indizes je Bündel zusammenhängend um. Sampler je Filter in der Bind-Group der Textur; `free_texture` entfernt eine noch nicht hochgeladene Textur aus der Warteschlange, sonst gibt das Backend die Bind-Group nach dem Bild frei. `batch.rs` aufgeteilt in `batch.rs`, `frame_canvas.rs`, `batch_tests.rs`; Backend in `gpu.rs` und `setup.rs`.
- **Review (Sonnet):** keine schweren Befunde. Behoben: Tönung und Netz-Farben mit Alpha < 1 werden jetzt vormultipliziert (vorher wurde ein halbdurchsichtiges Sprite heller statt blasser; galt schon seit RU2), geschlossener Pfad (letzter = erster Punkt) wird gefüllt. Offen gelassen: Vollfarbe liegt innerhalb von Ebene und Ausschnitt über Texturen (Vorentscheidung „gebündelt nach Ebene, Ausschnitt, Textur“; Aufrufer trennen über Ebenen), `TexId` werden nicht wiederverwendet.
- **Abweichungen:** (1) `textureSample` im Shader vor die Verzweigung gezogen (gleichförmiger Kontrollfluss). (2) Kreise mit fester Segmentzahl 48 statt adaptiver Bögen wie `vector`; Pfade nur aus Geraden (Kurven: Nicht-Ziel). (3) Formen ohne Kantenglättung (`antialias` von `vector.*` entfällt).
