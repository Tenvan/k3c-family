# RU4.3 · Schrift: TTF mit Umlauten, Breite messen, Rückfall Pixel-Schrift

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
