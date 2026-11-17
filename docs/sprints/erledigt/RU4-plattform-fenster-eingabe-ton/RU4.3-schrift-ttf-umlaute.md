# RU4.3 · Schrift: TTF mit Umlauten, Breite messen, Rückfall Pixel-Schrift

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.2
- **Tickets:** B-1014
- **Kriterien:** AC-05

## Ziel

Die Plattform zeichnet Text aus `public/schrift/k3c.ttf` mit Umlauten in Größe und Farbe und misst seine Breite gleich im Fenster und headless.

## Kontext

Braucht die Freigabe der Crate `fontdue` durch 🧑 (RU4-README › Braucht 🧑); ohne Freigabe Status `blockiert`. Vorgaben: RU4-README › Vorentscheidungen › Schrift. Go-Vorbild: `native/k3c-game/art/font.go` (feste Datei `public/schrift/k3c.ttf`, Rückfall auf Debug-Schrift mit Warnung). Den Port von `art/font.go` macht RU7; hier nur die Plattform. Bestehend: Pixel-Schrift in `rust/crates/k3c-platform/src/font.rs` (`Canvas::text`), Instanz-Pipeline aus RU4.2.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/Cargo.toml`
- `rust/Cargo.lock`

## Nicht-Ziele

Port von `native/k3c-game/art/font.go` (RU7). Zeilenumbruch und Layout (HUD, Menüs).

## Schritte

1. `fontdue` mit fester Version (`=`) in `rust/Cargo.toml` › `[workspace.dependencies]`, in `k3c-platform/Cargo.toml` einbinden.
2. `src/ttf.rs`: Schrift laden, Glyphen je (Zeichen, Pixelgröße gerundet) rastern, Glyphen-Atlas (eine Textur, 2048², Zeilen-Packer), Cache als Feld mit fester Kapazität; Kerning wie fontdue liefert.
3. `Canvas::font(ttf) -> Result<FontId, String>`, `text_font(font, s, x, y, px, color)`, `text_width(font, s, px)`; Glyphen gehen als `sprite`-Instanzen an die Atlas-Textur. `NullCanvas` lädt die Schrift ebenfalls und misst gleich.
4. Tests: `src/ttf_tests.rs` mit `public/schrift/k3c.ttf` (Pfad ab `CARGO_MANIFEST_DIR`): „ÄÖÜäöüß“ hat für jedes Zeichen eine Glyphe ungleich der Ersatz-Glyphe; Breite wächst mit `px`; zweites Zeichnen desselben Textes legt keine neue Glyphe an; `tests/alloc.rs` mit `text_font` (nach dem ersten Bild 0 Allokationen).
5. `tests/layering.rs`: `fontdue` darf außerhalb von `backend/` stehen (reine Rechnung); keine Änderung nötig, wenn der Test nur winit/wgpu sucht.

## Fertig, wenn

- [ ] AC-05: `cargo test -p k3c-platform ttf` grün mit allen Umlauten.
- [ ] AC-05: `tests/alloc.rs` zählt nach dem ersten Bild 0 Allokationen mit `text_font`.
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

- **AC-05 geprüft:** `cargo test -p k3c-platform ttf` grün (8 Tests): „ÄÖÜäöüß“ hat in `public/schrift/k3c.ttf` je eine eigene Glyphe (Index ≠ 0); Breite wächst mit `px` und ist auf ganze Pixel gerundet; zweites Zeichnen legt keine Glyphe an, andere Größe schon; Glyphen stehen auf der Grundlinie (`y` + Oberlänge) und laufen um die gemessene Breite weiter; `Frame` und `NullCanvas` messen gleich (12, 24,5, 48 px); voller Atlas wird erst am Bildanfang geleert; zu große Glyphe behält ihren Vorschub; kaputte Datei liefert einen Fehler.
- **AC-05 geprüft:** `tests/alloc.rs` zeichnet zusätzlich `text_font` mit Umlauten und misst `text_width`: nach dem ersten Bild 0 Allokationen in 120 Bildern. Neue Glyphen (Cache-Fehlgriff) allozieren beim Rastern einmalig – erlaubt, weil sie nur im ersten Bild eines Textes vorkommen.
- **Lauf:** Probe zeigt „Grüße: ÄÖÜ äöü ß – Spieler 1“ in der TTF-Schrift, Vulkan ohne Validierungsfehler. **Ralf prüft:** Text in der Probe ist scharf, Umlaute und ß sind richtig, keine abgeschnittenen Ober- oder Unterlängen. `cargo clippy --workspace --all-targets -D warnings` grün; `task check:diff` grün.
- **API:** `Canvas::font(ttf) -> Result<FontId, String>`, `text_font(font, s, x, y, px, color)` (`(x, y)` = linke obere Ecke der Zeile wie `text/v2`), `text_width(font, s, px)`; `FontId`. `Frame::take_atlas_updates`. `fontdue = "=0.9.5"` ohne Standard-Features (nur `std`; Freigabe B-1014).
- **Umsetzung:** `src/ttf.rs`: `Fonts` (Laden, Breite mit Unterschneidung, Oberlänge), `Atlas` 2048² mit Zeilen-Packer und Cache `HashMap` mit fester Kapazität 4096 je (Schrift, Zeichen, px). Die Atlas-Textur legt `Frame` beim ersten Text an; neue Glyphen gehen als Teil-Updates (`write_texture` an der Stelle) ins Backend, das dafür jetzt Textur und Bind-Group hält.
- **Review (Sonnet):** keine schweren Befunde. Behoben: Atlas-Reset mitten im Bild hätte schon gelegte Glyphen überschrieben (jetzt erst am nächsten Bildanfang), Glyphen auf ganze Pixel gerundet (linearer Filter, sonst unscharf), zu große Glyphe verlor ihren Vorschub. Die Rückfall-Logik (Datei fehlt → Pixel-Schrift mit Warnung) liegt beim Aufrufer: Probe in RU4.9, Spiel in RU7 (`art/font.go`).
