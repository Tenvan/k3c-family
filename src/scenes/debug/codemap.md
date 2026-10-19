# src/scenes/debug/

## Responsibility

Debug-Werkzeuge (an, außer mit `?dev=0`): Diagnose-Overlay und Cheat-Dialog samt Gesten und Dev-Aktionen. Keine Spielregel: Aktionen werden auf die Nachricht `dev` abgebildet, der Server entscheidet.

## Design

- `debugOverlay.ts` (`debugLines`: Textzeilen, u. a. `Puffer … ms` und `Latenz … ms (p95 … ms)`; `debugEnabled`, `VERSION_KEY`, `DebugClient`/`DebugWorld`) + `debugOverlayView.ts` (`DebugOverlay`: zeichnet die Zeilen, verbindet Gesten, Dialog und `RoomClient`).
- `debugOverlayPanel.ts` (`CheatDialog`: modal, `focusEdges`/`focusStep`/`diagGesture`; `muteFocused`: Controller-Fokus sperrt Spielkommandos; `PAD_FOCUS`, `KEY_FOCUS`, `DEV_FOCUS_KEY`).
- `debugActions.ts` (`DEV_ACTIONS` → `DevMessage` via `devMessage`, `nextToggles`, `pauseMessage`, `roomDevMode`; `DEV_GROUPS`, `UPGRADE_LEVELS`).
- `debugGestures.ts` (`holdStep`: LB+RB 3 s = Cheat-Dialog, RB 3 s = Diagnose; `tapStep`/`listenTaps`: Doppeltap mit zwei bzw. einem Finger).
- Tests: `debugActions`, `debugGestures`, `debugOverlay`, `debugOverlayPanel` (`*.test.ts`).

## Flow

1. `HudScene` erzeugt `DebugOverlay` (`debugEnabled`: aus nur mit `?dev=0`); `LobbyScene` zeigt die Version über `VERSION_KEY`.
2. Gesten (`holdStep` am Pad, `listenTaps` per Touch) öffnen Diagnose oder `CheatDialog`; bei offenem Dialog bedienen Controller/Tastatur ihn über `focusStep`, `GameScene` sperrt Spielkommandos (`muteFocused`).
3. Gewählte Aktion → `devMessage` → `RoomClient`; `debugLines` zeigt Puffer, Latenz und Raumzustand.

## Integration

- Konsumenten: `../HudScene.ts` (`debugOverlay`, `debugOverlayView`), `../GameScene.ts` (`debugOverlayPanel`), `../LobbyScene.ts` (`debugOverlay`), `src/main.ts` (`VERSION_KEY`).
- Abhängigkeiten: `../GameScene` (Typ, `debugOverlayView`), `../localSlots` (`SlotCommand`, `debugOverlayPanel`), `src/online/` (`clientConnection`, `clientProtocol`, `clientLatency`), `src/input/keyboardLayouts`, `src/model/types`, `src/core/` (`constants`, `texts`, `clientLog`), Phaser, DOM (CSS des Dialogs).
