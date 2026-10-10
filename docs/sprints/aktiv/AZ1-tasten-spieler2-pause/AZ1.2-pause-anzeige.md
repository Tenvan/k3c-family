# AZ1.2 · Pause-Anzeige je Zelle und angehaltene Figuren-Animationen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** sprint/az1
- **Abhängig von:** AZ1.1
- **Tickets:** B-333
- **Kriterien:** AC-03, AC-04

## Ziel

Meldet der Zustand `devPaused`, steht in jeder Spieler-Zelle mittig „Pausiert“ auf einem halbtransparenten Band, und alle Figuren-Animationen stehen still; läuft der Raum, ist das Band weg und die Animationen laufen weiter.

## Kontext

- **Zustand:** `devPaused` kommt nur im Dev-Mode des Servers (`WorldState` in `src/online/clientProtocol.ts`); der Welt-Typ der Szene kennt das Feld nicht, `debugOverlay.ts` liest es als `{ devPaused?: boolean }`. Fehlt das Feld → keine Anzeige.
- **Aussehen** (Entscheidung 🧑 bei der Freigabe 2026-10-10): Text „Pausiert“ (`t()`, de/en) mittig je Spieler-Zelle auf halbtransparentem Band, passend zu den HUD-Elementen aus U6; Schrift mindestens laut `fontRules.ts` (B-136).
- **Zellen:** `HudScene.update()` bekommt `game.hudCells()` (`RadarCell` mit `cell.x/y/w/h`, `cell.kind` `player`/`partner`); nur `player`-Zellen bekommen das Band.
- **Vorrang:** Ist die Verbindung weg, zeigt `HudScene.showHints()` `gameNotice(client)` groß in der Mitte und kehrt früh zurück; das Band darf diesen Hinweis nicht verdecken (dann ausblenden).
- **Animationen:** Figuren spielen Phaser-Animationen über den globalen `AnimationManager` (`src/scenes/sprites.ts`, `mountView.ts`, `objectView.ts`); `scene.anims.pauseAll()` / `resumeAll()` hält alle an. Nur bei Wechsel des Zustands aufrufen, nicht jeden Frame.
- **Grenzen:** `GameScene.ts` hat 398 Zeilen, dort nichts hinzufügen; Band als eigenes Modul (z. B. `src/scenes/pauseBanner.ts`) mit reiner Funktion für „sichtbar?“ und Test.

## Erlaubte Dateien

- neues Modul `src/scenes/pauseBanner.ts` mit Test
- `src/scenes/HudScene.ts`, `src/scenes/hudLayout.ts` (nur falls das Band ein HUD-Element wird)
- `src/core/texts.de.ts`, `src/core/texts.en.ts` (nur neuer Schlüssel für „Pausiert“)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Pause durch den Server im Couch-Raum (B-214); Optionen-Szene; Änderungen an Server, Protokoll oder `src/online/`; Tweens und Effekte anhalten.

## Schritte

1. `Status: in Arbeit`.
2. Test zuerst: reine Funktion (z. B. `pauseShown(state, away)`) → `true` nur bei `devPaused === true` und ohne Verbindungs-Hinweis; fehlendes Feld → `false`.
3. Band je Spieler-Zelle in `pauseBanner.ts`, von `HudScene.update()` gezeichnet; Text über `t()`.
4. Bei Wechsel von `devPaused` `anims.pauseAll()` bzw. `resumeAll()` aufrufen.
5. Browser-Pane (Freigabe 🧑 für diesen Lauf): Raum mit Dev-Mode, zwei Spieler, `/dm` „⏸ Pause“ → Band in beiden Zellen, `anims.paused === true`; „▶ Weiter“ → weg.
6. `task check` grün, Ergebnis eintragen, committen.

## Fertig, wenn

- [ ] AC-03: Test der Anzeige-Funktion grün; Browser-Pane: Band in jeder Spieler-Zelle bei Pause, weg nach „Weiter“ (Beobachtung am PC folgt in AZ1.4).
- [ ] AC-04: Browser-Pane: Animationen während der Pause angehalten, danach wieder laufend (Beobachtung am PC folgt in AZ1.4).
- [ ] Verbindungs-Hinweis wird nicht verdeckt; `task check` grün.

## Prüfen

```bash
task check
```

Browser-Pane laut Schritt 5, nur mit Freigabe 🧑 für diesen Lauf.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
