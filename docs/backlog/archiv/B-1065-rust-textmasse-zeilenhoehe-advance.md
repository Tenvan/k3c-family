# B-1065 · Rust misst Textzeilen niedriger und kleine Schrift schmaler als Go

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU20
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Gemessen im Bildvergleich RU20.7 (tests/still_hud.rs): Rust misst die Höhe einer Textzeile als Schriftgröße (24,0 px), Go als Zeilenhöhe (26,9 px bei derselben Größe); die Boxen der Textelemente sind in Rust 2,5 bis 2,9 px niedriger, darunter gestapelte Elemente liegen entsprechend höher. Der Beitritts-Hinweis (kleine Schrift) ist in Rust 6 % schmaler (183 statt 194 px). x und Breite der Boxen stimmen (Boxbreite in 32-px-Stufen). Zusammenhang mit B-1063 (Text 2 px tiefer), vermutet gleiche Ursache in k3c-platform (Ascent, Zeilenabstand, Advance-Rundung).

## Ziel

Rust misst Textzeilen wie Go (`text.Measure`): Zeilenhöhe = Ober- plus Unterlänge der Schrift, Schriftgrößen bleiben gebrochen. Boxen der Textelemente haben dieselbe Höhe wie in Go, kleine Schrift dieselbe Breite.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `Canvas::text_height(font, px)` liefert Ober- plus Unterlänge (`k3c-platform`, Frame, NullCanvas, SoftCanvas); `art/font.rs::measure` nutzt sie statt der Schriftgröße und liefert für leeren Text (0, 0) wie Go.
- Die Plattform rundet `px` nicht mehr auf ganze Pixel, sondern quantelt auf 1/64 Pixel (Go: `GoTextFace.Size` gebrochen); Breite, Oberlänge, Glyphen-Raster und Cache-Schlüssel nutzen dieselbe Größe.
- Werte gegen Go belegt (Go `Metrics()`/`text.Measure` mit `public/schrift/k3c.ttf`, Go v2.10.6).

## Nicht-Ziele

Pixelgleiche Glyphen wie Go (Hinting, Unterschneidung per GPOS); Ursache der 2 px in B-1063 (dort offen); Verengen der Toleranzen in `tests/still_hud.rs` (braucht erzeugte Grafik, `gen:paper` in diesem Lauf gesperrt).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung gegenüber Go (B-1014); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; keine neue Abhängigkeit; `Canvas::text_height` hat eine Standard-Methode (Größe), damit Stubs unverändert bleiben.

- **Bereich:** `rust/crates/k3c-platform/src/ttf.rs`, `rust/crates/k3c-platform/src/ttf_tests.rs`, `rust/crates/k3c-platform/src/lib.rs`, `rust/crates/k3c-platform/src/frame_canvas.rs`, `rust/crates/k3c-platform/src/headless.rs`, `rust/crates/k3c-game/src/art/font.rs`, `rust/crates/k3c-game/src/art/font_tests.rs`, `rust/crates/k3c-game/src/still/raster.rs`
- **Abhängig von:** RU20
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Schrift `k3c.ttf`, Größe 24: Zeilenhöhe 27,738 (vorher 24,0), Oberlänge 22,676; „Hallo Welt“ bei 12 px: Breite 56,37 (Go 56,39).

## Ausnahme- und Fehlerfälle

Unbekannte Schrift: Höhe 0. Größe unter 1 px: auf 1 geklemmt.

## Akzeptanzkriterien

- **AC-01** Test `ttf_masse_wie_go` (k3c-platform): Oberlänge und Zeilenhöhe bei 12, 16 und 24 px stimmen mit den Go-Werten auf 0,01 px, die Breite auf 0,1 px.
- **AC-02** Test `font_measure_hoehe_wie_go` (k3c-game): `measure` liefert Go-Höhe, zwei Zeilen im Abstand 1,25 × Größe, leerer Text (0, 0).
- **AC-03** `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` grün.

## Offene Fragen

keine

## Notizen

Umgesetzt 2026-11-18 (Claude). Ursache der 6 % schmaleren Schrift: `round_px` rundete jede Größe auf ganze Pixel, Go rechnet gebrochen. Ursache der niedrigeren Zeilen: `art/font.rs` nahm die Schriftgröße als Zeilenhöhe, die Plattform bot keine Unterlänge. Nicht geprüft: der Bildvergleich `tests/still_hud.rs` (ohne Grafik übersprungen, `gen:paper` gesperrt); die Toleranzen `TEXT_LINE_TOL`, `TEXT_TOL_W` und `MAX_MEAN_WITH_TEXT` bleiben unverändert und sind im Abnahme-Ticket mit erzeugter Grafik zu verengen. Frame-Budget: gebrochene Größen legen je Größe eigene Glyphen im Atlas an (vorher je ganze Pixelgröße); Messung im Fenster gehört zu G2 (RU39.2).
