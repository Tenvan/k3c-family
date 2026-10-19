# src/scenes/effects/

## Responsibility

Feedback und Juice: aus `GameEvent`s werden Ring-Effekte mit Partikeln, Kamera-Shake und Controller-Rumble. Nur Optik und Haptik, kein Spielzustand; Barrierefreiheits-Grenzen (Blitzrate, Einstellungen) stehen in den Regeln.

## Design

- `effects.ts` (`effectFor`: Event → `Effect`, `EFFECT_CONFIG` je `EffectKind`, Limits `MAX_FLASHES_PER_SECOND`, `FLASH_MIN_GAP_MS`, `SHAKE`, `RUMBLE`, `MAX_LIVE_EFFECTS`; `BuildSpots`).
- `effectRules.ts` (reine Regeln: `flashAllowed`/`runEffect` Blitz-Sperrfenster und Einstellung, `hurtSeat`, `shakeCell` Shake-Auswahl je Seat/Zelle, `rumblePad`).
- `effectsView.ts` (`StageEffects`: zeichnet Ring + Partikel einer Stufe in deren Ebene, zerstört nach Ablauf, höchstens `MAX_LIVE_EFFECTS` zugleich).
- Tests: `effects`, `effectRules`, `effectsView` (`*.test.ts`).

## Flow

1. `GameScene.spawnEffects()` ruft je `GameEvent` `feedback()` (Shake über `shakeCell`/`hurtSeat`, Rumble über `rumblePad`) und `effectFor`.
2. `runEffect` lässt einen Blitz nur zu, wenn Einstellung und Sperrfenster es erlauben; `StageEffects.spawn` zeichnet den Effekt in die Ebene der Stufe (`StageView`).

## Integration

- Konsumenten: `../GameScene.ts` (`effects`, `effectRules`), `../render/stageView.ts` (`StageEffects`, `effects`); außerhalb von `src/scenes`: Asset-Studio `tools/k3c-studio/frontend` (`EFFECT_CONFIG`, `EffectKind` über `@game/scenes/effects/effects`).
- Abhängigkeiten: `../render/fontRules` (`effectsView`), `../layout` (`Cell`, `effectRules`), `src/online/clientProtocol` (`SlotSeat`), `src/model/types` (`GameEvent`), `src/core/constants`, Phaser.
