# B-1056 · Canvas: Multiply-Blend für die Lichtkarte

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c-platform::Canvas` kennt keinen Multiply-Blend. `native/k3c-game/world/lightdraw.go` (`drawLightMap`, `ebiten.Blend` multiply) braucht ihn für die Lichtkarte; `rust/crates/k3c-game/src/world/lightdraw.rs` rechnete die `LightMesh` nur, zeichnete sie nicht (RU13.2).

## Ziel

`k3c-platform::Canvas` kennt keinen Multiply-Blend (Ziel × Quelle, Deckkraft des Ziels bleibt), den `lightdraw.go` (`drawLightMap`, `ebiten.Blend multiply`) für die Lichtkarte braucht. Der Canvas bekommt einen Blend-Modus für `mesh` (oder eine eigene Multiply-Fläche); danach zeichnet `World::draw_light_map` in `world/lightdraw.rs` die schon gerechnete `LightMesh` (Aufruf entfällt in RU13.2). Ohne das fehlt in Rust die Nacht-Abdunklung der Zelle.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Neuer Zustand `Canvas::blend(Blend)` (`Over` = Standard, `Multiply` = Ziel × Quelle je Farbkanal, Deckkraft des Ziels bleibt), gilt wie `layer`/`clip` für die folgenden Aufrufe, je Bild zurückgesetzt.
- Aufrufe mit anderer Mischung bilden eigene Draw-Calls (`Run::blend`), Reihenfolge bleibt die der Aufrufe.
- GPU-Backend: je (Art, Mischung) eine Pipeline; `Multiply` mit Quelle-Faktor `Dst` (Farbe) bzw. `Zero` (Alpha), Ziel-Faktor `Zero` bzw. `One` wie `multiply` in `lightdraw.go`.
- Der Software-Rasterer des Standbilds (`SoftCanvas`) rechnet Multiply nach.
- `World::draw_light_map` zeichnet die Lichtkarte (Multiply, `mesh` auf einem weißen 1×1-Bild) wie Go.

## Nicht-Ziele

Weitere Mischmodi (add, screen); Bildvergleich der Nacht-Zustände mit Go (benötigt `gen:paper`, in diesem Lauf gesperrt); Messung im Fenster (RU39.2).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung gegenüber Go (B-1014); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; keine neue Abhängigkeit. Das GPU-Backend ist ohne Fenster nicht testbar und wurde nur gebaut und mit clippy geprüft.

- **Bereich:** `rust/crates/k3c-platform/src/lib.rs`, `rust/crates/k3c-platform/src/batch.rs`, `rust/crates/k3c-platform/src/batch_tests.rs`, `rust/crates/k3c-platform/src/frame_canvas.rs`, `rust/crates/k3c-platform/src/backend/gpu.rs`, `rust/crates/k3c-platform/src/backend/setup.rs`, `rust/crates/k3c-game/src/still/raster.rs`, `rust/crates/k3c-game/src/still/raster_tests.rs`, `rust/crates/k3c-game/src/world/lightdraw.rs`, `rust/crates/k3c-game/src/world/mod.rs`
- **Abhängig von:** B-1058
- **Braucht 🧑:** Gerät (Sichtprüfung der Nacht im Fenster, G2)
- **Größe:** M
- **Sperre:** –

## Beispiele

Ziel (255, 204, 102), Netz mit Farbe (0,5, 0,5, 0,5) im Multiply → (128, 102, 51), Deckkraft bleibt 255.

## Ausnahme- und Fehlerfälle

Mischung steht nach dem Bildende wieder auf `Over`; ein Canvas ohne Multiply-Unterstützung (Standard-Methode, `NullCanvas`) ignoriert den Aufruf.

## Akzeptanzkriterien

- **AC-01** Test `mischung_trennt_buendel_und_gilt_nur_ein_bild` (k3c-platform): andere Mischung gibt eigene Draw-Calls in Aufrufreihenfolge, Standard je Bild `Over`.
- **AC-02** Test `multiply_blend_rechnet_ziel_mal_quelle` (Standbild-Rasterer): Ziel × Quelle, Deckkraft bleibt, danach wieder Over.
- **AC-03** `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` grün; `draw_light_map` ruft `blend(Multiply)` und `mesh`.

## Offene Fragen

keine

## Notizen

Umgesetzt 2026-11-18 (Claude): AC-01 bis AC-03 grün. Nicht geprüft: Sichtprüfung der Nacht im Fenster und Bildvergleich der Nacht-Zustände gegen Go (Grafik nicht erzeugt, `gen:paper` gesperrt; GPU-Pfad nur gebaut). Beides gehört zu Ralfs G2-Messung (RU39.2); darum den PR erst danach mergen.
