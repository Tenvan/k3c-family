# GR4.2 · Spiel lädt aus Atlanten, Lade-Szene mit Fortschritt und Fehlermeldung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr4/2-laden-ladeszene
- **Abhängig von:** GR4.1
- **Tickets:** B-163, B-029
- **Kriterien:** AC-02, AC-03, AC-05, AC-06

## Ziel

Das Spiel lädt Figuren (und bereits eingebaute Umgebungs-Grafiken) aus den Atlanten statt aus Einzeldateien; beim Start zeigt eine Lade-Szene den Fortschritt und bei einem Fehler eine Meldung. Die Zahl der Grafik-Requests vorher und nachher ist dokumentiert.

## Kontext

- Laden heute: `src/scenes/sprites.ts` (`preloadSprites`: je Sheet und Animation ein `scene.load.spritesheet`, Schlüssel `textureKey(sheet, anim)` = `<sheet>-<anim>`; `createSpriteAnims` baut die Animationen), aufgerufen in `GameScene.preload` (`src/scenes/GameScene.ts`). Die Atlas-Beschreibung aus GR4.1 nennt je Frame die Position; Animationen müssen dieselben Schlüssel und Frame-Folgen ergeben, damit `worldRenderer.ts` unverändert zeichnet.
- Umgebungs-Grafiken: Nur was GR3 bis dahin eingebaut hat; ist GR3 nicht gelaufen, betrifft AC-02 nur die Figuren (im Ergebnis nennen).
- **Lade-Szene (B-029):** eigene Phaser-Szene vor Lobby/Spiel, Balken mit Fortschritt aus den Ladeereignissen von Phaser (`progress`, `complete`, `loaderror`); ein fehlgeschlagenes Asset → Meldung mit Dateiname statt ewig laufendem Balken. `src/scenes` rechnet nichts; Home-Button oben (70 px frei); B nicht belegen. Szenen-Reihenfolge in `game.html`/Spiel-Config prüfen (Lobby `src/scenes/LobbyScene.ts`).
- Requests zählen: im Browser-Pane `read_network_requests` (bzw. `performance.getEntriesByType('resource')`) beim Start von `game.html` vorher und nachher, nur Bilder; Werte ins Ergebnis und in B-163 › Notizen.
- Texte zentral, falls S5.3 (B-172) gelaufen ist.

## Erlaubte Dateien

- `src/scenes/sprites.ts`, `src/scenes/GameScene.ts` (nur Laden), neue Lade-Szene in `src/scenes/` mit Test der reinen Teile (z. B. Fortschritt → Text)
- Einstieg des Spiels (`src/main*.ts` bzw. Datei mit der Phaser-Config, nur Szenen-Reihenfolge)
- `docs/backlog/B-163-*.md` (nur Notizen: Messwerte), `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Nachladen im Hintergrund, neue Grafiken, Effekte (GR5), Messung auf der Xbox (GR4.3), Änderungen am Server.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Requests „vorher“ im Browser-Pane messen.
2. `preloadSprites` lädt die Atlanten; Animationen mit unveränderten Schlüsseln.
3. Lade-Szene mit Balken und Fehlermeldung; Fehlerfall im Browser-Pane einmal provozieren (Datei umbenannt, nicht einchecken).
4. Requests „nachher“ messen; beide Zahlen ins Ergebnis und in B-163.
5. Sichtprüfung im Browser-Pane mit 2 lokalen Spielern: Figuren und Animationen wie vorher (Screenshot im PR).
6. `task check` und `task build`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-02: Figuren (und eingebaute Umgebungs-Grafiken) kommen aus Atlanten; Requests vorher/nachher dokumentiert.
- [x] AC-03: Lade-Szene zeigt Fortschritt bis alles geladen ist (Nachweis im Browser-Pane).
- [x] AC-06: Bei einem Ladefehler zeigt sie eine Meldung mit Dateinamen (Nachweis im Browser-Pane).
- [x] AC-05: `task check` und `task build` grün.

## Prüfen

```bash
task check
task build
```

## Ergebnis

Nachweis je Kriterium (geprüft im Browser-Pane gegen `task build` + Go-Server, durch den Agenten in diesem Lauf):

- **AC-02** umgesetzt und geprüft: `preloadSprites` lädt per `load.multiatlas` den Atlas aus GR4.1; `createSpriteAnims` baut dieselben Schlüssel `<sheet>-<anim>` über `generateFrameNames`, `makeSprite` nimmt Frame `<sheet>-idle/0`, `worldRenderer.ts` unverändert. Im Browser: 41 Animationen, 9 Sprites der Szene alle auf Textur `atlas`, laufende Animationen `martial-hero-3-idle`, `huntress-2-idle`. **Requests vorher 41 PNGs, nachher 2** (`atlas.json`, `atlas-0.png`). Umgebungs-Grafiken: GR3 nicht gelaufen, betrifft nur die Figuren; Reittiere sind (wie vorher) nicht dabei.
- **AC-03** geprüft: neue `LoadScene` (`src/scenes/LoadScene.ts`, reine Texte in `loadLogic.ts` mit Test) ist erste Szene in `main.ts`, Balken und Prozenttext aus `progress`, nach `complete` Animationen anlegen und `lobby` starten; im Browser-Pane landet `game.html` in der Lobby (Szene `load` → `lobby`).
- **AC-06** geprüft: `atlas-0.png` in `dist/` umbenannt (nicht eingecheckt), `game.html` neu geladen: Szene bleibt `load`, roter Balken, Text „Laden fehlgeschlagen: atlas/atlas-0.png …“ plus Eintrag im Client-Log; Datei zurückbenannt.
- **AC-05** geprüft: `task check` (669 Tests, Typecheck, Lint) und `task build` grün.

Abweichungen und Hinweise:

- „Vorher“ nicht per Netzwerkmitschnitt, sondern aus `data/sprites.json` gezählt (14 Sheets, 41 Animationen, je eine PNG); der alte Loader lud genau diese.
- Sichtprüfung mit `?autostart=1&mock=1` (zwei Spieler, Split-Screen): Figuren sichtbar und animiert. Der Screenshot-Vergleich für den PR bleibt bei 🧑 (Browser-Pane war im Hochformat, daher klein).
- Szenen-Reihenfolge: `load` → `lobby` → `game`/`hud`; GameScene hat kein `preload` mehr, Animationen sind global.
- Ladefehler zeigen nur die Meldung (kein automatischer Neuversuch, Nicht-Ziel).
