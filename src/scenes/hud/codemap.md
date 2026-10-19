# src/scenes/hud/

## Responsibility

Bildschirmfeste Anzeigen der `HudScene`: Layout der HUD-Elemente, Meldungen, Gold, Radar, Skill-Menü, Wirtschafts- und Bürger-Texte, Steuer-, Beitritts- und Pause-Hinweise. Zustand und Texte sind rein, die `*View`-Dateien zeichnen nur.

## Design

- **Layout:** `hudLayout.ts` (`hudLayout`, `anchorRect`, `overlaps`, `RANK`, `GAP`, `BAR`, `DEBUG_AREA`: reine Rechnung, Elemente je Zelle/Bildschirm ohne Überschneidung nach Rang) + `hudElements.ts` (`HudBox`, `BOX`: Kasten-Stil, `freeAreas`); genutzt von `HudScene.ts` und `skillMenuView.ts`.
- **HUD-Zustand (rein):** `bannerQueue.ts` (`enqueueBanner`, `fitBanner`, `BANNER_MAX_CHARS`: einzeilige Meldungsleiste), `goldFlash.ts` (`GoldFlash`, `stepGoldFlash`: Goldzahl kurz hervorheben), `controlsHint.ts` (`ControlsTrack`, `stepControlsHint`: Steuerhinweis nach Beitritt/Gerätewechsel), `joinHint.ts` (`joinHintDevices`: beitrittsfähige Geräte), `pauseBanner.ts` (`pauseShown`, `PauseBanner`: „Pausiert“-Band bei `devPaused`).
- **Wirtschaft/Bürger-Texte (rein):** `economyLabels.ts` (`waitInfo`, `siteWait`, `hubWait`, `stockTexts`, `hubText`, `WaitInfo`: Wartegrund, Lagerstand, Hub-Stufe aus Server-Feldern; von `render/siteStatusView.ts` genutzt), `citizenLabels.ts` (`buergerLines`, `professionLabel`, `merchantText`, `healerRanges`, `limitText`: Texte je Bürger; von `render/citizenView.ts` genutzt).
- **Radar:** `radar.ts` (reine Marker-Berechnung `radarMarkers`/`cellRadar`, `radarRect`) + `radarView.ts` (`RadarLayer`, `RadarCell`).
- **Skill-Menü:** `skillMenuLogic.ts` (`menuEntries`, `slotViews`, `SkillMenus.route`: lernen/Respec aus `points`/`actions` des Servers, Befehle statt Spielerkommandos) + `skillMenuView.ts` (`SkillMenuLayer`).
- **Touch:** `pauseButton.ts` (`PauseButton`, `pauseButton()`: Schaltfläche „Optionen“, ein Knopf pro Seite, `take()` einmal je Druck).
- Tests: `bannerQueue`, `citizenLabels`, `controlsHint`, `economyLabels`, `goldFlash`, `hudLayout`, `joinHint`, `pauseBanner`, `radar`, `skillMenuLogic` (`*.test.ts`).

## Flow

1. `HudScene` liest `GameScene.hudCells()` und `pendingEvents`, bildet `HudItem`s und lässt `hudLayout` sie über `freeAreas` ohne Überschneidung anordnen.
2. Gold (`stepGoldFlash`), Vorrat (`stockTexts`, `hubText`), Meldungen (`enqueueBanner`/`fitBanner`), Bürger-Texte, Steuer-/Beitrittshinweise (`stepControlsHint`, `joinHintDevices`), `PauseBanner`, `RadarLayer` und `SkillMenuLayer` (Zustand aus `GameScene.skillMenus`) werden je Frame aktualisiert.
3. `GameScene` hält `SkillMenus` (`route`: das Menü fängt Befehle ab) und zeigt/liest `pauseButton()`; `OptionsScene` hält ihn währenddessen an (`suspend`).

## Integration

- Konsumenten: `../HudScene.ts` (alle HUD-Dateien), `../GameScene.ts` (`radarView`, `joinHint`, `pauseButton`, `skillMenuLogic`), `../OptionsScene.ts` (`pauseButton`), `../hints/` (`radarView`, `skillMenuLogic`), `../render/` (`economyLabels`, `citizenLabels`).
- Abhängigkeiten: `../layout` (`hudLayout`, `radar`, `radarView`), `../localSlots` (`skillMenuLogic`), `../render/fontRules` (`pauseBanner`, `skillMenuView`), `../hints/glyphs` (`skillMenuView`), `src/model/types`, `src/core/` (`constants`, `texts`), Phaser.
