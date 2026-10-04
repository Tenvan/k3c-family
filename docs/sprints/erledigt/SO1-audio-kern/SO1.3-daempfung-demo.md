# SO1.3 · Positions-Dämpfung im Split-Screen und Demo-Ereignis

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** so1/3-daempfung-demo
- **Abhängig von:** SO1.2
- **Tickets:** B-011
- **Kriterien:** AC-06, AC-07, AC-08

## Ziel

Im Split-Screen hört jeder Spieler seinen Bereich (Effekte leiser, je weiter die Quelle von den Kameras entfernt ist), Warnungen sind global laut; mindestens ein Spiel-Ereignis löst einen Ton aus.

## Kontext

- **Ereignisse:** Der Client bekommt sie je Frame (`frame.state.events` → `pendingEvents` in `src/scenes/GameScene.ts`, Typ `GameEvent` in `src/model/types.ts`). Typen heute u. a. `built`, `wave`, `dusk`, `night`, `dawn`, `chest`, `playerDown`, `castleFallen`, `goldStolen`; F3 (erledigt) ergänzt Feedback-Ereignisse (Q08: Treffer, Kill, Münze, Pfeil, Schlag, Bau, Tod, Wiederbeleben, Skill, Nacht naht, Portal). Audio reagiert nur auf Ereignisse und Snapshot, rechnet keine Regel.
- **Dämpfung:** reine Funktion (Quell-x, Kameras der lokalen Spieler mit Mitte und Breite, global ja/nein) → Lautstärke 0–1; Warnungen (z. B. `wave`, Nacht naht, `castleFallen`) global. 1 Spieler = keine Dämpfung im sichtbaren Bereich. Kameras je Zelle: `src/scenes/layout.ts` / `GameScene`.
- **Demo-Ereignis:** ein Ton (aus dem Atlas, SO1.2) z. B. auf `built`; welche Ereignisse „wichtig“ sind, klärt SO2 (Q15, B-167) — hier nur der erste Schritt.
- Ohne Entsperrung (SO1.2) bleibt alles stumm, ohne Fehler.

## Erlaubte Dateien

- `src/audio/` (Dämpfung, Ereignis-Anbindung, Tests)
- `src/scenes/GameScene.ts` (nur: Ereignisse und Kamera-Ausschnitte an den Audio-Kern geben)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

SFX-Katalog und weitere Ereignisse (SO2), Musik (SO4), Optionen-Oberfläche (S5).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Reine Dämpfungs-Funktion mit Test: Quelle in Zelle von Spieler 1 → für ihn laut, außerhalb beider Zellen leise; Warnung immer laut; 1 Spieler ohne Dämpfung.
3. Ereignis-Anbindung: `built` spielt den Demo-Ton mit Dämpfung nach Position.
4. Nachweis im Browser-Pane mit 2 lokalen Spielern: Ton bei Bau (Konsole/Audio-Graph, Lautstärke je nach Position); am TV in SO1.5.
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-06: Test der Dämpfungs-Funktion (Bereich je Spieler, Warnungen global).
- [x] AC-07: Ein Spiel-Ereignis löst einen Ton aus (Nachweis im Browser-Pane).
- [x] AC-08: `task check` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check
```

## Ergebnis

- **AC-06** geprüft: `src/audio/events.test.ts` (`attenuation`): Quelle im Bereich von Spieler 1 oder 2 laut, außerhalb beider leiser und mit Abstand fallend bis 0, Warnungen (`wave`, `castleFallen`) global, 1 Spieler im Bereich ungedämpft.
- **AC-07** geprüft (Test, nicht Browser-Pane: laut Auftrag keine Browser-Prüfung): `audioCore.test.ts` › `onEvent`: `built` startet den Atlas-Ton `coin` mit Dämpfung nach Position (`GameScene.spawnEffects` → `audioCore().onEvent`, Ort aus `buildSpots`, Kameras aus `hudCells`); vor Entsperrung stumm. Browser-Nachweis bleibt SO1.5 (TV).
- **AC-08** geprüft: `task check` grün (925 Tests); `audioCore.ts` 115, `GameScene.ts` 374 Zeilen, alle Funktionen < 60.
- Abweichung: Schritt 4 (Browser-Pane mit 2 Spielern) nicht ausgeführt, siehe AC-07. Dämpfung: `EDGE_GAIN` 0,5 am Rand, linear 0 bei einer Bildschirmbreite Abstand; der lauteste lokale Spieler gilt.
