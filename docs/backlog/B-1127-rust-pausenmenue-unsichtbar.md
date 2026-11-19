# B-1127 · Rust-Spiel: Pausenmenü bleibt unsichtbar, obwohl das Spiel pausiert

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU54
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Smoke-Abnahme durch Ralf (RU48): Im Rust-Fenster erscheint nach Menü-Taste kein Pausenmenü, das Spiel steht aber (Go zeigt das Menü). Ursache (belegt im Code): `k3c_platform::Frame::finish` sortiert nach `(Ebene, Ausschnitt, Aufruf)`. Die Zellen-Drawer zeichnen mit Ausschnitt (Index >= 1) auf Ebene 0, das HUD auf Ebene 800/801; Overlay-Drawer und oberster Screen in `game/draw.rs` zeichnen ohne Ausschnitt (Index 0) und erben Ebene 0. Der Screen sortiert damit vor die Spielwelt und unter das HUD und liegt verdeckt. Das Hauptmenü ist sichtbar, weil dann keine Zellen zeichnen.

## Ziel

Der oberste Screen (Pause, Optionen, Tasten) und die Overlay-Drawer liegen im Rust-Fenster über Welt und HUD, wie in Go (dort gilt die reine Aufrufreihenfolge).

## Beteiligte und Zielgruppen

Ralf spielt am PC mit Pad, zwei Ansichten (Split-Screen); Abnahme im Fenster durch Ralf.

## Anforderungen

- Fenster-Ebene für Overlay und Screen über `hud::paint::Z_CONTENT`.
- Regressionstest mit der echten `k3c_platform::Frame`: letzter gezeichneter Aufruf nach `finish` gehört dem Screen.
- Keine Allokation je Bild hinzufügen (RU42-Ratsche bleibt).

## Nicht-Ziele

Umbau der Sortierung in `batch.rs`; Farben (B-1128); Bildvergleich (B-1129, B-1120).

## Regeln und Einschränkungen

Rust-Leitplanken RU42; Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen.

- **Bereich:** `rust/crates/k3c-game/src/game/draw.rs`, `rust/crates/k3c-game/src/game/draw_tests.rs`, `rust/crates/k3c-game/src/game/mod.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Pause im Spiel mit zwei Zellen: Pause-Karten stehen über beiden Zellen und dem HUD.

## Ausnahme- und Fehlerfälle

Schiffsfahrt, Ladeanzeige und Fehlerfenster zeichnen vor dem Spiel und sind nicht betroffen (nicht relevant).

## Akzeptanzkriterien

- **AC-01** Test `screen_liegt_ueber_zellen_und_hud` ist grün und war ohne den Fix rot.
- **AC-02** Ralf sieht im Fenster das Pausenmenü über dem stehenden Spiel (manuelle Abnahme).

## Offene Fragen

keine

## Notizen

Die Sortierung nach Ausschnitt gilt für Zellen-Drawer so gewollt (Bündelung); die Ebene ist der vorgesehene Weg (`Canvas::layer`).
