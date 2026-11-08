# src/scenes/render/

## Responsibility

Zeichnet die `World` des Servers je Stufe: ein Phaser-Objekt je Entity, Grafik-Zuordnung, Reiter, Bürger, Hintergründe und Schrift-Katalog. Keine Spielregeln; Auswahl und Darstellungsregeln liegen in reinen Dateien ohne Phaser.

## Design

- **Zeichnen:** `worldRenderer.ts` (`WorldRenderer`, ein Phaser-Objekt je Entity-id; reicht `resourceName`/`siteName` aus `siteView` durch), `stageView.ts` (`StageView` je Stufe, Platzhalter-Layer für ungeladene Zellen, `showOnly` ordnet Kameras zu, Hintergrund über `backgroundLayers`, hält `StageEffects`), `viewRules.ts` (Darstellungsregeln ohne Sim-Import: `canAfford`, `hasDepth`, `isOnTower`, `daylight`, `PRICE_TAG_RANGE`).
- **Papierschnitt-Laden:** `paperAssets.ts` (Zuordnung aus `data/paper.json`, Ursprung und Maßstab aus dem Manifest), `mountGait.ts` (Gangart schritt/trab/galopp nach Tempo), `sprites.ts` (lädt Manifeste und Sheets der Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json`: `preloadSprites`, `createSpriteAnims`, `makeSprite`, `playPaper`; Reittiere `MOUNTS`, `RIDER_WAIST`).
- **Grafik-Zuordnung (rein, `null` = Platzhalter):** `buildingSprites.ts` (`siteTexture`, `hubTexture`, `backWallTexture`, `BUILDING_TEXTURES`; Gebäude aus `gebaeude-paper/`, Burg-Ausbau als Überlage `burg-ausbau`, `castleOverlay`), `plantation.ts` (`plantationPlots`, `isPlantationTree`: Setzling und Pappel der Baumschule, GEB5), `worldSprites.ts` (`objectSprite`, `biomeBackground`: Hintergründe aus `hintergruende/` im Maßstab 1, drei Schichten je Biom, GEB7; `WORLD_TEXTURES`), `natureSprites.ts` (`treeTexture`, `natureLook`, `veinSlot`, `nodeSprite`: Baumform aus der Position, außerhalb des Walds Wurzeln und Höhlenbusch, GEB6/GEB10), `pennant.ts` (`pennantColor`, `noteMarkers`: Wimpel in Spielerfarbe aus `coinGive`-Ereignissen, GEB10).
- **Gebäude und Welt-Objekte:** `siteView.ts` (`createSiteView`/`updateSiteView`, `createCastleView`/`updateCastleView` je Hub-Stufe, `preloadBuildings`/`prepareBuildings`, Preisschild, `TEXT`-Stil), `objectView.ts` (`preloadWorld`/`prepareWorld`, `objectImage`, `backgroundLayers`).
- **Regeln dazu (rein):** `backWallRules.ts` (`backWalls`, `BackWall`) + `backWallView.ts` (`createBackWallView`/`updateBackWallView`: gekachelte Hintergrundmauer, B-442), `priceTagRules.ts` (`priceTagSites`, `offersPrice`: nur das nächste Preisschild je Spieler, B-395), `siteStatusView.ts` (`siteStatus`, `statusLine`, `drawSiteStatus`: Wartegrund am Bauplatz, von `siteView.ts` genutzt).
- **Reittier:** `mountView.ts` (`createRider`/`updateRider`, Container je Monarch; Monarch samt Pferd aus dem Monarch-Asset).
- **Bürger:** `citizenView.ts` (`BuergerView`: Berufssymbol, Händler, Heilreichweite; Texte aus `hud/citizenLabels.ts`).
- **Kamera und Schrift:** `cameraFollow.ts` (`refollow`, `FollowCam`: Kamera-Nachführung), `fontRules.ts` (`FONTS`-Katalog, `fontStyle`, Mindestgrößen `minFontPx`, `contrastRatio`).
- **Gebäude-Darstellung:** `buildingSize.ts` (Textur-Schlüssel → Datei aus `BUILDING_TEXTURES`, Papier: Maßstab 1, weich; das alte Slot-Register `data/assets.json` und das Zielmaß B-685 entfielen mit CU1.2), `targetFit.ts` (`fitTarget`, Typen), `buildingLook.ts` (Phaser: laden, Maßstab und Filter einmal je Grafik, `showBuildingLook`).
- Tests: `backWallRules`, `buildingSize`, `buildingSprites`, `cameraFollow`, `fontRules`, `natureSprites`, `pennant`, `plantation`, `priceTagRules`, `siteStatusView`, `worldSprites` (`*.test.ts`).

## Flow

1. `LoadScene` ruft `preloadSprites`, `preloadWorld`, `preloadBuildings`, danach `createSpriteAnims`, `prepareWorld`, `prepareBuildings`.
2. `GameScene` legt je Stufe einen `StageView` an; `StageView` besitzt einen `WorldRenderer` und `StageEffects`, ordnet Zellen-Kameras zu (`showOnly`) und setzt den Hintergrund (`backgroundLayers`).
3. `WorldRenderer` zeichnet `timeline.sample(now)` je Frame: Gebäude über `siteView` (Tor-, Mauer-, Preisschild-Regeln), Welt-Objekte über `objectView`, Reiter über `mountView`/Bürger über `BuergerView`.
4. `cameraFollow.refollow` führt die Zellen-Kamera dem Monarchen nach; `HudScene` liest Schriftgrößen aus `fontRules`.

## Integration

- Konsumenten: `GameScene.ts` (`StageView`, `viewRules`, `refollow`), `HudScene.ts` (`WorldRenderer`, `fontRules`), `LoadScene.ts` (`objectView`, `siteView`, `sprites`), `../hud/`, `../hints/`, `../effects/effectsView.ts` (`fontRules`).
- Abhängigkeiten: `../hud/citizenLabels`, `../hud/economyLabels`, `../effects/` (`StageView`), `../layout` (`fontRules`), `src/model/` (`types`, `data`, `biome`), `src/core/` (`constants`, `texts`), `data/paper.json` (› `figuren`), Grafiken unter `public/grafik/`, Phaser.
- `sprites.ts` lädt die Manifeste und Sheets der Figuren-Gruppen (`preloadSprites`, `createSpriteAnims`); die alten Pixel-Sheets (`preloadSheet`, `preloadMount`) entfielen mit CU1.2; `createRider(scene, p, mountKey?)` nimmt das Reittier als Parameter.
