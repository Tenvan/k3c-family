# BD4.3 · Austreten im Spielmenü

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd4
- **Abhängig von:** –
- **Tickets:** B-411
- **Kriterien:** AC-05

## Ziel

„Spieler abmelden“ trifft den Öffner des Menüs (bei zwei Tastatur-Spielern den gewählten Slot); der letzte lokale Spieler verlässt wie „Spiel verlassen“.

## Kontext

Stand: `LocalSlots.removeLast` (höchster Slot), `REMOVE_PLAYER_KEY` als bool, `GameScene.wantsOptions` merkt sich den Öffner nicht, `OptionsScene` kennt keinen Öffner. Server kann `removeSlot` schon (kein Protokoll-Ticket nötig). Tastatur 2 hat keine Menü-Taste.

## Erlaubte Dateien

- `src/scenes/localSlots.ts`, `localSlots.test.ts`
- `src/scenes/optionsLogic.ts`, `optionsLogic.test.ts`
- `src/scenes/menuOpener.ts`, `menuOpener.test.ts` (neu, rein)
- `src/scenes/GameScene.ts`, `OptionsScene.ts`
- `src/core/texts.de.ts`, `texts.en.ts`

## Nicht-Ziele

Server- oder Protokolländerungen; Platz reservieren; neue Halte-Geste.

## Schritte

1. Tests zuerst (Vitest): `removeSlot(slot)` nur für den Slot, letzter Spieler -> `leave`; Öffner Pad/Tastatur 1/Touch, Eingabe ohne Slot -> ausgegraut; zwei Tastatur-Spieler -> Auswahl.
2. `removeLast` durch `removeSlot` mit Slot ersetzen; `menuOpener.ts` (rein).
3. `GameScene` merkt den Öffner (ein `MenuPress` je Pad), übergibt `targets` an `options`; `REMOVE_PLAYER_KEY` trägt den Slot.
4. `OptionsScene`: Eintrag ausgegraut ohne Ziel, Auswahl links/rechts bei mehreren Zielen.

## Fertig, wenn

- [ ] AC-05: Vitest für Öffner, Slot-genaues Abmelden, letzter Spieler, Tastatur-Auswahl grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

AC-05 (B-411/AC-01 bis AC-04) umgesetzt und geprüft: `LocalSlots.removeAt(slot)` ersetzt `removeLast` (letzter Spieler -> `leave`), `menuOpener.ts` (Öffner, Ziele), je Pad ein `MenuPress`, `OptionsScene` mit Ziel-Auswahl und Ausgrauen; Vitest in `localSlots`, `menuOpener`, `optionsLogic`. Im Browser mit zwei Tastatur-Spielern bestätigt: Auswahl, nur der gewählte Slot geht, danach Lobby beim letzten Spieler. Keine Protokoll-Änderung. Entscheidung B-527. Pad-/Touch-Öffner und AC-06 am Gerät: BD4.4.
