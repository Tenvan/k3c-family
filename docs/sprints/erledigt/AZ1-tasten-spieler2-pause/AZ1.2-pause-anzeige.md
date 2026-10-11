# AZ1.2 · Pause-Anzeige je Zelle und angehaltene Figuren-Animationen

- **Status:** fertig
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

- [x] AC-03: Test der Anzeige-Funktion grün (`pauseBanner.test.ts`); Browser-Pane nicht durchgeführt (keine Freigabe für diesen Lauf), Beobachtung am PC folgt in AZ1.4.
- [ ] AC-04: Animationen halten bei Pause an und laufen danach weiter: umgesetzt (`anims.pauseAll()`/`resumeAll()` beim Wechsel), Beobachtung am PC in AZ1.4.
- [x] Verbindungs-Hinweis wird nicht verdeckt (Test); `task check` grün.

## Prüfen

```bash
task check
```

Browser-Pane laut Schritt 5, nur mit Freigabe 🧑 für diesen Lauf.

## Ergebnis

- **AC-03** umgesetzt und geprüft (B-333/AC-01): `src/scenes/pauseBanner.test.ts` › „nur bei devPaused === true“, „Zustand ohne das Feld … keine Anzeige“, „der Verbindungs-Hinweis hat Vorrang“, „Text Pausiert in de und en“. `task check` grün. Beobachtung am PC folgt in AZ1.4.
- **AC-04** umgesetzt, nicht beobachtet (B-333/AC-02): `HudScene.showPause` ruft `anims.pauseAll()` bzw. `resumeAll()` nur beim Wechsel von `devPaused` auf. Der Browser-Pane-Schritt 5 wurde nicht ausgeführt, weil 🧑 ihn für diesen Lauf nicht freigegeben hat; Beobachtung (Figuren stehen still, laufen nach „Weiter“) in AZ1.4. Zu prüfen dabei: Setzt `mountView.ts` mit `mount.play(key)` eine angehaltene Animation wieder in Gang?
- **Umsetzung:** neues Modul `src/scenes/pauseBanner.ts` (`pauseShown`, `PauseBanner`: halbtransparentes Band und „Pausiert“ in Schrift `banner` je `player`-Zelle), von `HudScene.update()` gezeichnet; Texte `hud.paused` in de und en. `GameScene.ts` unverändert.
- **Abweichung:** `hudLayout.ts` nicht angefasst, das Band ist kein HUD-Element der Layout-Berechnung, sondern liegt über der Zelle (Tiefe 40).
- **Neue Tickets:** keine.
