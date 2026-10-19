# src/scenes/hints/

## Responsibility

Hinweise mit Tasten-Glyphen über den Spielern: Aktions-Overlay und geführte erste Nacht. Der Client rechnet keine Regel; Aktionen und Zustände kommen aus `actions` bzw. dem Snapshot des Servers.

## Design

- **Glyphen:** `glyphs.ts` (`glyphOf`: Aktion + Gerät → Glyph-Schlüssel `pad:`/`key:`/`touch:`, B und View nie als Bild; `slotLabels`, `HintDevice`) + `glyphView.ts` (`drawGlyph`, `GlyphRow`: Zeile aus Text und Glyphen).
- **Aktionen:** `actionHints.ts` (`playerHint`/`hintView`: Aktionen aus `actions` des Servers, Bauen/Zahlen wie das Preisschild, `HINT_FONT`) + `actionOverlay.ts` (`ActionOverlay`, `screenAt`: Weltposition → Bildschirm je Zelle).
- **Geführte erste Nacht:** `guideHints.ts` (`nextGuide`/`stepGuide`/`guideDone`: `coin`→`pay`→`dusk`→`recruit` aus dem Snapshot, `guideImage`) + `guideOverlay.ts` (`GuideOverlay`; gesehen-Stand über `core/guideSeen`).
- Tests: `actionHints`, `glyphs`, `guideHints` (`*.test.ts`).

## Flow

1. `HudScene` ruft je Frame `ActionOverlay` und `GuideOverlay` auf, getrennt nach lokalem Spieler und Zelle.
2. `playerHint` wählt Hinweis (Schlag, Skills, Lernen, Respec, Bauen/Zahlen) aus `actions` und dem nächsten Preisschild; `glyphOf` liefert die Taste je Gerät, `GlyphRow` zeichnet Text und Glyph.
3. `stepGuide` hält den Hinweis, bis seine Handlung im Snapshot sichtbar ist, und markiert ihn über `guideSeen` als gesehen; `OptionsScene` setzt den Stand per `guideSeen.reset()` zurück.

## Integration

- Konsumenten: `../HudScene.ts` (`actionOverlay`, `guideOverlay`, `glyphs`), `../hud/skillMenuView.ts` (`glyphs`).
- Abhängigkeiten: `../render/` (`fontRules`, `priceTagRules`, `viewRules`), `../hud/` (`skillMenuLogic`, `radarView`), `../layout` (`Cell`), `src/input/slotBindings` (Tasten-Belegung, `Device`), `src/core/` (`guideSeen`, `settings`, `texts`, `constants`), `src/model/types`, Phaser.
