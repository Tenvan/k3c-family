# GR5.2 · Abschalten über Optionen, Split-Screen-Kamera, Blitzgrenze, Vibration

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr5/2-abschalten-kamera-vibration
- **Abhängig von:** GR5.1, S5.1
- **Tickets:** B-164
- **Kriterien:** AC-02, AC-04, AC-05, AC-06, AC-07

## Ziel

Screenshake und Blitz lassen sich über die Einstellungen ausschalten; Screenshake wirkt im Split-Screen nur in der Kamera des betroffenen Spielers, es gibt höchstens 3 Blitze pro Sekunde, und auf einem Controller ohne Vibration läuft das Spiel fehlerfrei weiter.

## Kontext

- **Einstellungen (entstehen in S5):** S5.1 legt `src/core/settings.ts` an (Speicher im `localStorage`, Standard alles an, kaputter Speicher → Standardwerte; Felder laut Vorschlag u. a. `screenshake` und `flash` als Boolean, **Name und Form dort nicht beschlossen**: die tatsächlichen Namen aus dem Stand von `develop` lesen). Die Optionen-Szene mit den Schaltern liefert S5.2. Fehlt `src/core/settings.ts` noch: `Status: blockiert`, nicht selbst einen zweiten Speicher bauen. Die Einstellung gilt je Gerät (Speicherort `localStorage`); sie beim Auslösen jedes Effekts lesen, damit eine Änderung in den Optionen ohne Neuladen wirkt.
- **Screenshake je Kamera:** `GameScene` hat je Zelle eine Kamera (`this.cameras.cameras[i]`, Kamera i gehört zu Zelle i, siehe `hudCells()` in `src/scenes/GameScene.ts`, Layout in `src/scenes/layout.ts`). Ein Schütteln über `camera.shake(...)` nur an der Kamera der Zelle des **betroffenen** Spielers: `hit` mit `target: 'player'` und `id` = Spieler-Index (`docs/protocol.md` › Ereignisse), Zuordnung Spieler → Sitz → Zelle über `client.you` (`SlotSeat` mit `slot` und `monarch`) wie bei `hudCells()`. Treffer an Gegnern oder Gebäuden schütteln keine Kamera. Ein Spieler ohne Zelle (anderes Gerät) → kein Effekt.
- **Blitzgrenze (AC-05):** höchstens 3 Blitze pro Sekunde (barrierefrei, B-164 › Regeln). Wert und Mindestabstand (z. B. ≥ 334 ms) in der Effekt-Konfiguration aus GR5.1 festhalten; Test der reinen Funktion „Blitz erlaubt?“ (Zeit des letzten Blitzes, Abstand) – ein Blitz im Sperrfenster wird verworfen, nicht aufgeschoben. „Blitz“ heißt Treffer-Blitz und jeder Vollbild-/Kamera-Flash; Partikel zählen nicht.
- **Vibration:** über die Gamepad API (`pad.vibrationActuator.playEffect('dual-rumble', …)`); Muster und Prüfung der Verfügbarkeit stehen in `src/tools/gamepadTest.ts` (`rumble()`, Feld `vibrationActuator`). Die Spielseite hat die Pads als Phaser-Gamepads in `GameScene` (`GamepadInput.pad`, `src/input/playerInput.ts`; `src/input/` gehört der Domäne PLAT und wird hier **nicht** geändert, nur gelesen). Controller ohne `vibrationActuator` oder mit ablehnendem Promise → nichts tun, kein Fehler, kein Log-Spam (`clientLog` aus `src/core/clientLog.ts` höchstens einmal). Ob Edge auf der Xbox Vibration kann, steht im Bericht der Gamepad-Testseite (`reports/*.json`, Feld `rumble`/`vibration`); bis das gemessen ist, gilt „angenommen“, die Funktion ist ohnehin optional. Nur das Pad des betroffenen Spielers vibriert.
- **Regeln:** `src/scenes` rechnet nichts; 2 Spieler im Split-Screen; B-Taste nicht belegen; Dateien ≤ 400 Zeilen.

## Erlaubte Dateien

- `src/scenes/GameScene.ts` (nur: Kamera je Zelle, Anbindung der Effekte, Einstellungen lesen), `src/scenes/effects*.ts` und neue Dateien in `src/scenes/` mit Tests daneben
- `src/core/settings.ts` nur lesen (gehört S5); fehlt ein Feld dort, Ticket statt Änderung
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Die Optionen-Szene selbst (S5), Event-Liste und Protokoll (F3, F4), Sound (SO2), Änderungen in `src/input/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Prüfen, dass `src/core/settings.ts` (S5.1) auf `develop` liegt; sonst `Status: blockiert`.
2. Blitz- und Shake-Auslösung hinter die Einstellungen legen (Schalter „aus“ → Effekt entfällt; „an“ → sichtbar); Test der reinen Entscheidung (Einstellung, Zeit seit letztem Blitz → ausführen ja/nein).
3. Screenshake nur an der Kamera der betroffenen Zelle; Test der Zuordnung Event → Zelle (2 lokale Spieler, 1 Spieler, Spieler eines anderen Geräts).
4. Vibration mit Rückfall (Actuator fehlt, Promise abgelehnt) und Test mit gemocktem Pad; Controller-Mock: `pad` ohne `vibrationActuator` laut `CLAUDE.md` › „Im Browser-Pane testen“.
5. Im Browser-Pane mit zwei Spielern im Split-Screen prüfen: Schütteln nur in einer Zelle; Schalter aus → kein Schütteln und kein Blitz; Screenshot.
6. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-02: Mit Screenshake und Blitz „aus“ in den Einstellungen treten beide nicht auf, mit „an“ sind sie sichtbar (Test der Entscheidung und Beobachtung im Browser-Pane; am TV: Abnahme durch 🧑, im Review als „angenommen, Validierung offen“ geführt).
- [x] AC-04: Mit 2 lokalen Spielern schüttelt nur die Kamera des betroffenen Spielers (Test der Zuordnung, Screenshot im Browser-Pane).
- [x] AC-05: Die Blitzfrequenz liegt bei höchstens 3 pro Sekunde (Test der Konfiguration und der Entscheidungsfunktion).
- [x] AC-06: Controller ohne Vibration (Mock ohne `vibrationActuator`) → keine Ausnahme, Spiel läuft weiter (Test).
- [x] AC-07: `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Umgesetzt: `src/scenes/effectRules.ts` (reine Entscheidungen: `runEffect`, `flashAllowed`, `hurtSeat`, `shakeCell`, `rumblePad`), Konfiguration `FLASH_MIN_GAP_MS` (334 ms), `SHAKE`, `RUMBLE` in `effects.ts`, Anbindung in `GameScene.spawnEffects`/`feedback` (Einstellungen je Frame über `loadSettings()`).

- AC-02: umgesetzt, geprüft per Test `runEffect` (Blitz aus: entfällt, an: läuft); Schalter `screenshake` gatet `camera.shake`. Beobachtung im Browser-Pane nicht gemacht (laut Auftrag); am TV: angenommen, Validierung offen (🧑).
- AC-04: geprüft per Test `shakeCell` (2 lokale Spieler, 1 Spieler mit Partner-Zelle, Spieler eines anderen Geräts, Gegner). Screenshot im Browser-Pane nicht gemacht (🧑).
- AC-05: geprüft per Test (Konfiguration 3 pro Sekunde, Simulation im 16-ms-Takt: höchstens 3 Blitze in 1 s; Sperrfenster verwirft).
- AC-06: geprüft per Test `rumblePad` (Pad ohne `vibrationActuator`, abgelehntes Promise, werfender Actuator: keine Ausnahme); Log höchstens einmal.
- AC-07: `task check` grün (50 Testdateien, 903 Tests).
- Vibration nur am Pad des getroffenen Spielers (`hit` mit `target: 'player'`); kein eigener Schalter (die Optionen kennen keinen); Messung auf der Xbox offen (angenommen).
