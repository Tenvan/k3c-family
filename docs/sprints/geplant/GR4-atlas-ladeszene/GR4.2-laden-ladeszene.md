# GR4.2 · Spiel lädt aus Atlanten, Lade-Szene mit Fortschritt und Fehlermeldung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr4/2-laden-ladeszene
- **Abhängig von:** GR4.1
- **Tickets:** B-163, B-029
- **Kriterien:** AC-02, AC-03, AC-05

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

- [ ] AC-02: Figuren (und eingebaute Umgebungs-Grafiken) kommen aus Atlanten; Requests vorher/nachher dokumentiert.
- [ ] AC-03: Lade-Szene zeigt Fortschritt bis alles geladen ist, bei Fehler eine Meldung (Nachweis im Browser-Pane).
- [ ] AC-05: `task check` und `task build` grün.

## Prüfen

```bash
task check
task build
```

## Ergebnis

–
